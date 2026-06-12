use fsci_integrate::quad;
fn main() {
    // EVEN number of points (odd intervals) - scipy special-cases this
    for npts in [4usize, 6, 8, 10, 5, 7, 9] {
        let x: Vec<f64> = (0..npts).map(|i| i as f64 * 0.37).collect();
        let y: Vec<f64> = x.iter().map(|&t| t.sin() + 0.5 * t * t + 1.0).collect();
        match quad::simpson(&y, &x) {
            Ok(r) => println!("simpson_n{npts},{:.17e}", r.integral),
            Err(e) => println!("simpson_n{npts},ERR,{e:?}"),
        }
    }
}
