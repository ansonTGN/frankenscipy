//! pchip/akima/krogh/barycentric/hermite probe vs scipy.interpolate (gitignored).
//! Lines: method,case,i,value
use fsci_interpolate as ip;
fn dump(m: &str, case: usize, v: &[f64]) {
    for (i, &x) in v.iter().enumerate() {
        println!("{m},{case},{i},{x:.17e}");
    }
}
fn main() {
    // case datasets: (xi, yi)
    let cases: Vec<(Vec<f64>, Vec<f64>)> = vec![
        (vec![0., 1., 2., 3., 4., 5.], vec![0., 1., 4., 9., 16., 25.]),
        (
            vec![0., 0.5, 1.7, 2.1, 4.0, 4.2, 6.0],
            vec![1., 0.5, -0.3, 2.0, 1.1, 3.0, 0.0],
        ),
        (
            vec![-3., -1., 0., 2., 5., 5.5],
            vec![2., -1., 0., 3., -2., 1.0],
        ),
        (vec![0., 1., 2., 3.], vec![1., 1., 1., 1.]),
    ];
    // eval grid for each case: interior + nodes + slight extrapolation
    for (ci, (xi, yi)) in cases.iter().enumerate() {
        let lo = xi[0];
        let hi = *xi.last().unwrap();
        let mut xn = Vec::new();
        let steps = 40;
        for k in 0..=steps {
            xn.push(lo + (hi - lo) * (k as f64) / (steps as f64));
        }
        for &x in xi {
            xn.push(x);
        } // exact nodes
        if let Ok(v) = ip::pchip_interpolate(xi, yi, &xn) {
            dump(&format!("pchip_{ci}"), ci, &v);
        }
        if let Ok(v) = ip::akima1d_interpolate(xi, yi, &xn) {
            dump(&format!("akima_{ci}"), ci, &v);
        }
        if let Ok(v) = ip::krogh_interpolate(xi, yi, &xn) {
            dump(&format!("krogh_{ci}"), ci, &v);
        }
        if let Ok(v) = ip::barycentric_interpolate(xi, yi, &xn) {
            dump(&format!("bary_{ci}"), ci, &v);
        }
    }
}
