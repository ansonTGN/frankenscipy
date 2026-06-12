//! Differential oracle probe: gaussian_filter1d + uniform_filter1d vs scipy.ndimage (gitignored).
use fsci_ndimage::{BoundaryMode, NdArray, gaussian_filter1d, uniform_filter1d};

fn dump(name: &str, v: &[f64]) {
    let s: Vec<String> = v.iter().map(|x| format!("{x:.17e}")).collect();
    println!("{name}|{}", s.join(";"));
}

fn main() {
    let data = vec![1.0, 2.0, 4.0, 7.0, 3.0, 9.0, 5.0, 8.0, 2.0, 6.0, 1.0, 4.0];
    let input = NdArray::new(data, vec![12]).unwrap();
    let modes = [
        ("reflect", BoundaryMode::Reflect),
        ("constant", BoundaryMode::Constant),
        ("nearest", BoundaryMode::Nearest),
        ("wrap", BoundaryMode::Wrap),
    ];
    for &sigma in &[0.8_f64, 1.5, 2.5] {
        for order in [0usize, 1, 2, 3] {
            for (mn, m) in &modes {
                let r = gaussian_filter1d(&input, sigma, 0, order, *m, 0.0).unwrap();
                dump(&format!("g_s{sigma}_o{order}_{mn}"), &r.data);
            }
        }
    }
    for &size in &[2usize, 3, 5] {
        for (mn, m) in &modes {
            let r = uniform_filter1d(&input, size, 0, *m, 0.0).unwrap();
            dump(&format!("u_sz{size}_{mn}"), &r.data);
        }
    }
}
