use fsci_stats::*;
fn em<D: ContinuousDistribution>(label: &str, d: &D, xr: &[f64], xl: &[f64]) {
    for &x in xr {
        println!("{label},logsf,{x},{:.16e}", d.logsf(x));
    }
    for &x in xl {
        println!("{label},logcdf,{x},{:.16e}", d.logcdf(x));
    }
}
fn main() {
    let big = [40.0, 100.0, 300.0, 700.0];
    let bigneg = [-40.0, -100.0, -300.0, -700.0];
    em("logistic", &Logistic::new(0.0, 1.0), &big, &bigneg);
    em("laplace", &Laplace::new(0.0, 1.0), &big, &bigneg);
    em(
        "gumbel_r",
        &Gumbel::new(0.0, 1.0),
        &[10.0, 30.0, 80.0, 300.0],
        &[-3.0, -5.0],
    );
    em("moyal", &Moyal, &[10.0, 30.0, 80.0, 300.0], &bigneg);
    em("hypsecant", &HypSecant, &big, &bigneg);
    em(
        "gompertz|1.5",
        &Gompertz::new(1.5),
        &[5.0, 10.0, 30.0, 80.0],
        &[],
    );
    em("genlogistic|2", &GenLogistic::new(2.0), &big, &bigneg);
    em(
        "skewnorm|4",
        &SkewNorm::new(4.0),
        &[8.0, 15.0, 30.0],
        &[-8.0, -15.0, -30.0],
    );
    em(
        "pareto|2.5",
        &Pareto::new(2.5, 1.0),
        &[1e10, 1e50, 1e150],
        &[],
    );
    em("lomax|3", &Lomax::new(3.0), &[1e10, 1e50, 1e150], &[]);
    em(
        "tukeylambda|0.3",
        &TukeyLambda::new(0.3),
        &[3.0, 3.3, 3.32],
        &[-3.3],
    );
}
