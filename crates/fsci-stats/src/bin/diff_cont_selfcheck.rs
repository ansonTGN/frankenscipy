//! Param-free log-tail underflow detector (gitignored): in the deep tail, if
//! cdf/sf underflow to exactly 0 while pdf>0, then logcdf/logsf default ln(0)
//! = -inf is WRONG (true value finite). Reports dists whose logcdf/logsf
//! underflow where a closed form would stay finite.
use fsci_stats::*;
fn check<D: ContinuousDistribution>(label: &str, d: &D) {
    // left tail: walk down until cdf underflows to 0; right tail: sf to 0.
    let mut bad = String::new();
    // probe a grid of quantile depths via ppf, then push further into the tail.
    let xl = d.ppf(1e-12);
    let xr = d.ppf(1.0 - 1e-12);
    if xl.is_finite() {
        // push left until cdf==0
        let mut x = xl;
        let step = (xl.abs().max(1.0)) * 0.5 + 1.0;
        for _ in 0..200 {
            x -= step;
            let c = d.cdf(x); let p = d.pdf(x); let lc = d.logcdf(x);
            if c == 0.0 && p > 1e-300 && !lc.is_finite() { bad.push_str(&format!(" LOGCDF@x={x:.3}(pdf={p:.1e})")); break; }
            if !p.is_finite() || p == 0.0 { break; }
        }
    }
    if xr.is_finite() {
        let mut x = xr;
        let step = (xr.abs().max(1.0)) * 0.5 + 1.0;
        for _ in 0..200 {
            x += step;
            let s = d.sf(x); let p = d.pdf(x); let ls = d.logsf(x);
            if s == 0.0 && p > 1e-300 && !ls.is_finite() { bad.push_str(&format!(" LOGSF@x={x:.3}(pdf={p:.1e})")); break; }
            if !p.is_finite() || p == 0.0 { break; }
        }
    }
    if !bad.is_empty() { println!("{label}:{bad}"); }
}
fn main(){
    check("normal",&Normal::new(0.0,1.0));
    check("gennorm|1.5",&GenNorm::new(1.5));
    check("gennorm|3",&GenNorm::new(3.0));
    check("gumbel_r",&Gumbel::new(0.0,1.0));
    check("gumbel_l",&GumbelLeft::new(0.0,1.0));
    check("laplace",&Laplace::new(0.0,1.0));
    check("logistic",&Logistic::new(0.0,1.0));
    check("genlogistic|2",&GenLogistic::new(2.0));
    check("exponnorm|1.5",&ExponNorm::new(1.5));
    check("skewnorm|4",&SkewNorm::new(4.0));
    check("hypsecant",&HypSecant);
    check("genextreme|0.2",&GenExtreme::new(0.2));
    check("weibull|1.5",&Weibull::new(1.5,1.0));
    check("gamma|3",&GammaDist::new(3.0,1.0));
    check("foldnorm|1.5",&FoldedNormal::new(1.5));
    check("powernorm|2",&PowerNorm::new(2.0));
    check("dgamma|2",&DoubleGamma::new(2.0));
    check("dweibull|2",&DoubleWeibull::new(2.0));
    check("gennorm|0.7",&GenNorm::new(0.7));
    check("maxwell",&Maxwell::new(1.0));
    check("rayleigh",&Rayleigh::new(1.0));
    println!("DONE");
}
