use fsci_integrate::gauss_legendre;
fn main() {
    // n-point GL is exact for polynomials up to degree 2n-1.
    for n in [2usize, 3, 5, 8, 10, 16] {
        // integrate x^(2n-1) over [0,1] => 1/(2n)
        let deg = 2 * n - 1;
        let v = gauss_legendre(|x: f64| x.powi(deg as i32), 0.0, 1.0, n);
        let exact = 1.0 / ((deg + 1) as f64);
        println!(
            "gl_poly,n{n},deg{deg},ours={v:.17e},exact={exact:.17e},relerr={:.3e}",
            ((v - exact) / exact).abs()
        );
    }
    // also reproduce the cos case
    let v = gauss_legendre(|x: f64| (3.0 * x).cos(), 0.0, 2.0, 10);
    println!("gl_cos n10 = {v:.17e} (true -9.313849939964196e-2)");
}
