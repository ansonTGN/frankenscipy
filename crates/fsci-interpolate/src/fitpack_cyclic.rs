//! Cyclic-tridiagonal solvers from FITPACK (`fpcyt1`/`fpcyt2`).
//!
//! Faithful safe-Rust port of Dierckx's `fpcyt1` (LU-style factorization of a
//! cyclic-tridiagonal matrix) and `fpcyt2` (solve using that factorization).
//! These are the foundation for the periodic v-direction of the gridded
//! spherical spline (`spgrid`/`fpgrsp`, bead frankenscipy-flath, in progress).
//! 1-based indexing mirrors the Fortran line-for-line.
//!
//! Storage convention: `a[i]` (rows `1..=n`) holds the cyclic-tridiagonal band
//! columns `1..=6`, where on input `a[i][1]` is the sub-diagonal, `a[i][2]` the
//! diagonal, `a[i][3]` the super-diagonal, with the wrap corners in `a[1][1]`
//! (element `(1, n)`) and `a[n][3]` (element `(n, 1)`). Columns `4..=6` receive
//! the factorization.

/// `fpcyt1`: factorize the cyclic-tridiagonal matrix stored in `a` (rows
/// `1..=n`, requires `n >= 3`), writing the factorization into columns `4..=6`.
#[allow(dead_code)]
pub(crate) fn fpcyt1(a: &mut [Vec<f64>], n: usize) {
    let one = 1.0_f64;
    let n2 = n - 2;
    let mut beta = one / a[1][2];
    let mut gamma = a[n][3];
    let mut teta = a[1][1] * beta;
    a[1][4] = beta;
    a[1][5] = gamma;
    a[1][6] = teta;
    let mut sum = gamma * teta;
    for i in 2..=n2 {
        let v = a[i - 1][3] * beta;
        let aa = a[i][1];
        beta = one / (a[i][2] - aa * v);
        gamma = -gamma * v;
        teta = -teta * aa * beta;
        a[i][4] = beta;
        a[i][5] = gamma;
        a[i][6] = teta;
        sum += gamma * teta;
    }
    let n1 = n - 1;
    let v = a[n2][3] * beta;
    let aa = a[n1][1];
    beta = one / (a[n1][2] - aa * v);
    gamma = a[n][1] - gamma * v;
    teta = (a[n1][3] - teta * aa) * beta;
    a[n1][4] = beta;
    a[n1][5] = gamma;
    a[n1][6] = teta;
    a[n][4] = one / (a[n][2] - (sum + gamma * teta));
}

/// `fpcyt2`: solve the cyclic-tridiagonal system using the [`fpcyt1`]
/// factorization in `a`; right-hand side `b[1..=n]`, solution written to
/// `c[1..=n]`.
#[allow(dead_code)]
pub(crate) fn fpcyt2(a: &[Vec<f64>], n: usize, b: &[f64], c: &mut [f64]) {
    c[1] = b[1] * a[1][4];
    let mut sum = c[1] * a[1][5];
    let n1 = n - 1;
    for i in 2..=n1 {
        c[i] = (b[i] - a[i][1] * c[i - 1]) * a[i][4];
        sum += c[i] * a[i][5];
    }
    let cc = (b[n] - sum) * a[n][4];
    c[n] = cc;
    c[n1] -= cc * a[n1][6];
    let mut j = n1;
    for _i in 3..=n {
        let j1 = j - 1;
        c[j1] = c[j1] - c[j] * a[j1][3] * a[j1][4] - cc * a[j1][6];
        j = j1;
    }
}

#[cfg(test)]
mod tests {
    use super::{fpcyt1, fpcyt2};

    // Dense apply of the cyclic-tridiagonal matrix encoded in `a` (1-based,
    // columns 1=sub, 2=diag, 3=super; corners a[1][1]=(1,n), a[n][3]=(n,1)).
    fn apply(a: &[Vec<f64>], n: usize, c: &[f64]) -> Vec<f64> {
        let mut out = vec![0.0_f64; n + 1];
        for i in 1..=n {
            let mut s = a[i][2] * c[i];
            if i < n {
                s += a[i][3] * c[i + 1];
            }
            if i > 1 {
                s += a[i][1] * c[i - 1];
            }
            out[i] = s;
        }
        // wrap corners
        out[1] += a[1][1] * c[n];
        out[n] += a[n][3] * c[1];
        out
    }

    #[test]
    fn fpcyt_solves_cyclic_tridiagonal() {
        let n = 6;
        // Diagonally-dominant cyclic tridiagonal so the factorization is stable.
        let sub = [0.0, 0.0, -1.0, -1.2, -0.8, -1.1, -0.9]; // a[i][1], a[1][1]=corner
        let dia = [0.0, 5.0, 5.5, 6.0, 5.2, 5.8, 6.1];
        let sup = [0.0, -1.3, -0.7, -1.0, -1.1, -0.6, 0.0]; // a[i][3], a[n][3]=corner
        let corner_1n = -0.9; // (1,n)
        let corner_n1 = -1.4; // (n,1)
        let mut a = vec![vec![0.0_f64; 7]; n + 1];
        for i in 1..=n {
            a[i][1] = sub[i];
            a[i][2] = dia[i];
            a[i][3] = sup[i];
        }
        a[1][1] = corner_1n;
        a[n][3] = corner_n1;
        let a_orig = a.clone();
        let b: Vec<f64> = vec![0.0, 1.0, -2.0, 3.0, 0.5, -1.5, 2.5];
        fpcyt1(&mut a, n);
        let mut c = vec![0.0_f64; n + 1];
        fpcyt2(&a, n, &b, &mut c);
        // residual: A c == b.
        let bc = apply(&a_orig, n, &c);
        for i in 1..=n {
            assert!((bc[i] - b[i]).abs() <= 1e-10, "row {i}: {} vs {}", bc[i], b[i]);
        }
    }
}
