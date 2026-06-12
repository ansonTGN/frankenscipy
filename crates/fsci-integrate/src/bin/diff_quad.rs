//! Differential oracle probe: discrete-data quadrature vs scipy.integrate (gitignored).
use fsci_integrate::quad;

fn main() {
    // irregular x, smooth y
    let x: Vec<f64> = vec![0.0, 0.3, 0.7, 1.2, 1.5, 2.1, 2.6, 3.0, 3.7, 4.2, 5.0];
    let y: Vec<f64> = x.iter().map(|&t| t.sin() + 0.5 * t * t).collect();
    // uniform x for cumulative_simpson (scipy prefers uniform)
    let xu: Vec<f64> = (0..=10).map(|i| i as f64 * 0.4).collect();
    let yu: Vec<f64> = xu.iter().map(|&t| t.cos() + 0.2 * t).collect();

    match quad::simpson(&y, &x) {
        Ok(r) => println!("simpson_irreg,0,{:.17e}", r.integral),
        Err(e) => println!("simpson_irreg,ERR,{e:?}"),
    }
    match quad::simpson(&yu, &xu) {
        Ok(r) => println!("simpson_unif,0,{:.17e}", r.integral),
        Err(e) => println!("simpson_unif,ERR,{e:?}"),
    }
    if let Ok(c) = quad::cumulative_trapezoid(&y, &x) {
        for (i, &v) in c.iter().enumerate() {
            println!("cumtrap,{i},{v:.17e}");
        }
    }
    if let Ok(c) = quad::cumulative_simpson(&yu, &xu) {
        for (i, &v) in c.iter().enumerate() {
            println!("cumsimp,{i},{v:.17e}");
        }
    }
    for n in [1usize, 2, 3, 4, 5, 6, 7, 8] {
        if let Ok(w) = quad::newton_cotes(n) {
            for (i, &wi) in w.iter().enumerate() {
                println!("nc{n},{i},{wi:.17e}");
            }
        }
    }
}
