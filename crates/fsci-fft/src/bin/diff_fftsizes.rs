use fsci_fft::{Complex64, FftOptions, fft, rfft};
fn main() {
    let o = FftOptions::default();
    for &n in &[
        1usize, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 16, 17, 20, 24, 25, 31, 32, 36, 100,
    ] {
        let x: Vec<Complex64> = (0..n)
            .map(|k| {
                let t = k as f64;
                (
                    (0.5 * t).cos() + 0.2 * t - 0.03 * t * t,
                    (0.3 * t).sin() - 0.1 * t,
                )
            })
            .collect();
        if let Ok(y) = fft(&x, &o) {
            print!("fft_{n}|");
            let s: Vec<String> = y
                .iter()
                .map(|c| format!("{:.15e};{:.15e}", c.0, c.1))
                .collect();
            println!("{}", s.join(","));
        }
        // rfft on real signal
        let xr: Vec<f64> = (0..n)
            .map(|k| (0.4 * k as f64).sin() + 0.1 * k as f64)
            .collect();
        if let Ok(yr) = rfft(&xr, &o) {
            print!("rfft_{n}|");
            let s: Vec<String> = yr
                .iter()
                .map(|c| format!("{:.15e};{:.15e}", c.0, c.1))
                .collect();
            println!("{}", s.join(","));
        }
    }
}
