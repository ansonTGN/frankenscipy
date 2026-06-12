//! Differential oracle probe: quadrature vs scipy.integrate (gitignored).
//! Lines: `name,key,value`. Inputs match the python comparator.
use fsci_integrate::{
    DblquadOptions, QuadOptions, cumulative_simpson, dblquad, fixed_quad, gauss_legendre, quad,
    quad_inf, romberg,
};

fn main() {
    let o = || QuadOptions::default();

    // quad on functions with known integrals
    if let Ok(r) = quad(|x: f64| x.sin(), 0.0, std::f64::consts::PI, o()) {
        println!("quad,sin_0_pi,{:.17e}", r.integral);
    }
    if let Ok(r) = quad(|x: f64| (-x * x).exp(), -2.0, 2.0, o()) {
        println!("quad,gauss,{:.17e}", r.integral);
    }
    if let Ok(r) = quad(|x: f64| 1.0 / (1.0 + x * x), 0.0, 1.0, o()) {
        println!("quad,arctan,{:.17e}", r.integral);
    }
    if let Ok(r) = quad(|x: f64| x.ln(), 1.0, 3.0, o()) {
        println!("quad,ln,{:.17e}", r.integral);
    }

    // fixed_quad (Gauss-Legendre, n points)
    for n in [3usize, 5, 8] {
        if let Ok((v, _)) = fixed_quad(|x: f64| x.powi(4) - 2.0 * x, 0.0, 2.0, n) {
            println!("fixed_quad,n{n},{:.17e}", v);
        }
    }

    // gauss_legendre direct
    println!(
        "gauss_legendre,n10,{:.17e}",
        gauss_legendre(|x: f64| (3.0 * x).cos(), 0.0, 2.0, 10)
    );

    // quad_inf: integral from a to +inf
    if let Ok(r) = quad_inf(|x: f64| (-x).exp(), 0.0, o()) {
        println!("quad_inf,exp,{:.17e}", r.integral); // = 1
    }
    if let Ok(r) = quad_inf(|x: f64| (-x * x).exp(), 0.0, o()) {
        println!("quad_inf,halfgauss,{:.17e}", r.integral); // = sqrt(pi)/2
    }

    // romberg
    let rr = romberg(|x: f64| x.sin(), 0.0, std::f64::consts::PI, 1e-10, 12);
    println!("romberg,sin,{:.17e}", rr.integral);

    // dblquad: ∫_0^1 ∫_0^x (x*y) dy dx
    if let Ok(r) = dblquad(
        |x: f64, y: f64| x * y,
        0.0,
        1.0,
        |_x| 0.0,
        |x| x,
        DblquadOptions::default(),
    ) {
        println!("dblquad,xy,{:.17e}", r.integral);
    }

    // cumulative_simpson
    let x: Vec<f64> = (0..11).map(|i| i as f64 * 0.2).collect();
    let y: Vec<f64> = x.iter().map(|&t| t * t).collect();
    if let Ok(cs) = cumulative_simpson(&y, &x) {
        for (i, &v) in cs.iter().enumerate() {
            println!("cumsimpson,i{i},{v:.17e}");
        }
    }
}
