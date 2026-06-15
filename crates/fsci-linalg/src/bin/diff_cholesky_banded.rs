//! Differential oracle probe: cholesky_banded vs scipy.linalg.cholesky_banded.
//! Lines: `name,r,c,value`. Inputs must match the python comparator.
use fsci_linalg::cholesky_banded;

fn dump(name: &str, m: &[Vec<f64>]) {
    for (r, row) in m.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            println!("{name},{r},{c},{v:.17e}");
        }
    }
}

fn main() {
    // Pentadiagonal SPD matrix, kd = 2.
    // Upper band storage (scipy convention): diagonal in last row.
    let ab_u = vec![
        vec![0.0, 0.0, 0.5, 0.5, 0.5, 0.5],
        vec![0.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        vec![4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
    ];
    if let Ok(c) = cholesky_banded(&ab_u, false) {
        dump("upper_kd2", &c);
    }

    // Lower band storage: diagonal in first row.
    let ab_l = vec![
        vec![4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
        vec![1.0, 1.0, 1.0, 1.0, 1.0, 0.0],
        vec![0.5, 0.5, 0.5, 0.5, 0.0, 0.0],
    ];
    if let Ok(c) = cholesky_banded(&ab_l, true) {
        dump("lower_kd2", &c);
    }

    // Tridiagonal SPD (kd = 1), upper.
    let tri_u = vec![
        vec![0.0, -1.0, -1.0, -1.0, -1.0],
        vec![2.0, 2.0, 2.0, 2.0, 2.0],
    ];
    if let Ok(c) = cholesky_banded(&tri_u, false) {
        dump("tri_upper", &c);
    }

    // Tridiagonal SPD (kd = 1), lower.
    let tri_l = vec![
        vec![2.0, 2.0, 2.0, 2.0, 2.0],
        vec![-1.0, -1.0, -1.0, -1.0, 0.0],
    ];
    if let Ok(c) = cholesky_banded(&tri_l, true) {
        dump("tri_lower", &c);
    }

    // Diagonal-only (kd = 0).
    let diag = vec![vec![9.0, 16.0, 25.0, 4.0]];
    if let Ok(c) = cholesky_banded(&diag, false) {
        dump("diag_only", &c);
    }
}
