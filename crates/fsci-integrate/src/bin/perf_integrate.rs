//! Profiling-only harness for integrate hot paths.
//!
//! NOT a product binary. It exists so RCH, Criterion follow-ups, and sha256
//! checks can attach to deterministic `solve_ivp` scenarios.
//!
//! Usage:
//!   `perf_integrate lorenz <repeats>`
//!   `perf_integrate exponential <repeats>`
//!   `perf_integrate bdf-stiff64 <repeats>`
//!   `perf_integrate radau-stiff32 <repeats>`
//!   `perf_integrate stiff-suite <repeats>`
//!   `perf_integrate golden [path]`

use std::fmt::Write as _;
use std::hint::black_box;
use std::path::Path;
use std::time::Instant;

use fsci_integrate::{SolveIvpOptions, SolveIvpResult, SolverKind, ToleranceValue, solve_ivp};
use fsci_runtime::RuntimeMode;

fn exponential_decay(_t: f64, y: &[f64]) -> Vec<f64> {
    vec![-y[0]]
}

fn lorenz(_t: f64, y: &[f64]) -> Vec<f64> {
    let (sigma, rho, beta) = (10.0, 28.0, 8.0 / 3.0);
    vec![
        sigma * (y[1] - y[0]),
        y[0] * (rho - y[2]) - y[1],
        y[0] * y[1] - beta * y[2],
    ]
}

fn exponential_options<'a>(y0: &'a [f64; 1]) -> SolveIvpOptions<'a> {
    SolveIvpOptions {
        t_span: (0.0, 10.0),
        y0,
        method: SolverKind::Rk45,
        rtol: 1e-6,
        atol: ToleranceValue::Scalar(1e-9),
        max_step: f64::INFINITY,
        mode: RuntimeMode::Strict,
        ..Default::default()
    }
}

fn lorenz_options<'a>(y0: &'a [f64; 3]) -> SolveIvpOptions<'a> {
    SolveIvpOptions {
        t_span: (0.0, 1.0),
        y0,
        method: SolverKind::Rk45,
        rtol: 1e-6,
        atol: ToleranceValue::Scalar(1e-9),
        max_step: f64::INFINITY,
        mode: RuntimeMode::Strict,
        ..Default::default()
    }
}

fn stiff_decay_rhs(_t: f64, y: &[f64]) -> Vec<f64> {
    let denom = y.len().saturating_sub(1).max(1) as f64;
    y.iter()
        .enumerate()
        .map(|(idx, &value)| {
            let rate = 1.0 + 999.0 * (idx as f64 / denom);
            -rate * value
        })
        .collect()
}

fn stiff_decay_options<'a>(y0: &'a [f64], method: SolverKind, t_bound: f64) -> SolveIvpOptions<'a> {
    SolveIvpOptions {
        t_span: (0.0, t_bound),
        y0,
        method,
        rtol: 1e-6,
        atol: ToleranceValue::Scalar(1e-8),
        max_step: f64::INFINITY,
        mode: RuntimeMode::Strict,
        ..Default::default()
    }
}

fn solve_exponential() -> SolveIvpResult {
    let y0 = [1.0];
    let opts = exponential_options(&y0);
    let mut rhs = exponential_decay;
    solve_ivp(&mut rhs, &opts).expect("solve exponential")
}

fn solve_lorenz() -> SolveIvpResult {
    let y0 = [1.0, 1.0, 1.0];
    let opts = lorenz_options(&y0);
    let mut rhs = lorenz;
    solve_ivp(&mut rhs, &opts).expect("solve lorenz")
}

fn solve_stiff_decay(method: SolverKind, n: usize, t_bound: f64) -> SolveIvpResult {
    let y0 = vec![1.0; n];
    let opts = stiff_decay_options(&y0, method, t_bound);
    let mut rhs = stiff_decay_rhs;
    solve_ivp(&mut rhs, &opts).expect("solve stiff decay")
}

fn write_result(output: &mut String, label: &str, result: &SolveIvpResult) {
    writeln!(
        output,
        "case={label} status={} success={} nfev={} njev={} nlu={} sol={} t_events={} y_events={} message={:?}",
        result.status,
        u8::from(result.success),
        result.nfev,
        result.njev,
        result.nlu,
        u8::from(result.sol.is_some()),
        u8::from(result.t_events.is_some()),
        u8::from(result.y_events.is_some()),
        result.message,
    )
    .expect("write result header");

    output.push_str("t=");
    for value in &result.t {
        write!(output, "{:016x},", value.to_bits()).expect("write t bits");
    }
    output.push('\n');

    output.push_str("y=");
    for row in &result.y {
        output.push('[');
        for value in row {
            write!(output, "{:016x},", value.to_bits()).expect("write y bits");
        }
        output.push(']');
    }
    output.push('\n');
}

fn golden_text() -> String {
    let mut output = String::new();
    write_result(
        &mut output,
        "exponential-rk45-no-output",
        &solve_exponential(),
    );
    write_result(&mut output, "lorenz-rk45-no-output", &solve_lorenz());
    write_result(
        &mut output,
        "stiff64-bdf-no-output",
        &solve_stiff_decay(SolverKind::Bdf, 64, 0.5),
    );
    write_result(
        &mut output,
        "stiff32-radau-no-output",
        &solve_stiff_decay(SolverKind::Radau, 32, 0.25),
    );
    output
}

fn write_or_print_golden(output: String, path: Option<&str>) {
    if let Some(path) = path {
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create golden artifact parent");
        }
        std::fs::write(path, output).expect("write golden artifact");
    } else {
        print!("{output}");
    }
}

fn timed_mode(mode: &str, repeats: usize) {
    let t0 = Instant::now();
    let mut checksum = 0.0_f64;
    let mut nfev = 0usize;
    let mut njev = 0usize;
    let mut nlu = 0usize;
    for _ in 0..repeats {
        let result = match mode {
            "lorenz" => solve_lorenz(),
            "exponential" => solve_exponential(),
            "bdf-stiff64" => solve_stiff_decay(SolverKind::Bdf, 64, 0.5),
            "bdf-stiff128" => solve_stiff_decay(SolverKind::Bdf, 128, 0.35),
            "radau-stiff32" => solve_stiff_decay(SolverKind::Radau, 32, 0.25),
            "radau-stiff64" => solve_stiff_decay(SolverKind::Radau, 64, 0.2),
            _ => unreachable!("validated mode"),
        };
        checksum += result
            .y
            .iter()
            .flat_map(|row| row.iter())
            .copied()
            .sum::<f64>()
            + result.t.iter().copied().sum::<f64>();
        nfev += result.nfev;
        njev += result.njev;
        nlu += result.nlu;
        black_box(&result);
    }
    let elapsed = t0.elapsed();
    let total_ms = elapsed.as_secs_f64() * 1e3;
    let per_call_us = elapsed.as_secs_f64() * 1e6 / repeats as f64;
    println!(
        "{{\"mode\":\"{mode}\",\"repeats\":{repeats},\"total_ms\":{total_ms:.3},\"per_call_us\":{per_call_us:.6},\"nfev\":{nfev},\"njev\":{njev},\"nlu\":{nlu},\"checksum\":{checksum:.12e}}}",
    );
}

fn timed_suite(repeats: usize) {
    for mode in [
        "bdf-stiff64",
        "bdf-stiff128",
        "radau-stiff32",
        "radau-stiff64",
    ] {
        timed_mode(mode, repeats);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("lorenz");

    if mode == "golden" {
        write_or_print_golden(golden_text(), args.get(2).map(String::as_str));
        return;
    }

    if mode == "stiff-suite" {
        let repeats = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(20);
        timed_suite(repeats);
        return;
    }

    if !matches!(
        mode,
        "lorenz"
            | "exponential"
            | "bdf-stiff64"
            | "bdf-stiff128"
            | "radau-stiff32"
            | "radau-stiff64"
    ) {
        eprintln!("unknown mode: {mode}");
        std::process::exit(2);
    }

    let repeats = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(20);
    timed_mode(mode, repeats);
}
