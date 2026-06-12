//! Differential oracle probe: spline interpolators vs scipy.interpolate (gitignored).
//! Lines: `name,i,value` or `name,ERR,..`. Same (xi,yi,xnew) as the python comparator.
use fsci_interpolate as ip;
use fsci_interpolate::{CubicSplineStandalone, SplineBc};

fn dump(name: &str, vals: &[f64]) {
    for (i, &v) in vals.iter().enumerate() {
        println!("{name},{i},{v:.17e}");
    }
}

fn main() {
    // f(x) = sin(x) + 0.3 x^2 - 0.5 x on a non-uniform grid
    let xi: Vec<f64> = vec![0.0, 0.5, 1.1, 1.7, 2.4, 3.0, 3.8, 4.5, 5.2, 6.0];
    let yi: Vec<f64> = xi
        .iter()
        .map(|&x| x.sin() + 0.3 * x * x - 0.5 * x)
        .collect();
    let xnew: Vec<f64> = {
        let mut v = Vec::new();
        let mut x = 0.2;
        while x <= 5.9 {
            v.push(x);
            x += 0.17;
        }
        v
    };

    // CubicSpline with each boundary condition.
    for (label, bc) in [
        ("cubic_natural", SplineBc::Natural),
        ("cubic_notaknot", SplineBc::NotAKnot),
        ("cubic_clamped", SplineBc::Clamped(0.4, -0.7)),
    ] {
        match CubicSplineStandalone::new(&xi, &yi, bc) {
            Ok(s) => {
                dump(label, &s.eval_many(&xnew));
                // first and second derivatives
                let d1 = s.derivative(1);
                let d2 = s.derivative(2);
                let dv1: Vec<f64> = xnew.iter().map(|&x| d1.eval(x)).collect();
                let dv2: Vec<f64> = xnew.iter().map(|&x| d2.eval(x)).collect();
                dump(&format!("{label}_d1"), &dv1);
                dump(&format!("{label}_d2"), &dv2);
                // definite integral over the full span
                println!("{label}_integ,0,{:.17e}", s.integrate(0.0, 6.0));
            }
            Err(e) => println!("{label},ERR,{e:?}"),
        }
    }

    // Periodic spline needs y[0]==y[n-1].
    {
        let mut yp = yi.clone();
        let n = yp.len();
        yp[n - 1] = yp[0];
        match CubicSplineStandalone::new(&xi, &yp, SplineBc::Periodic) {
            Ok(s) => dump("cubic_periodic", &s.eval_many(&xnew)),
            Err(e) => println!("cubic_periodic,ERR,{e:?}"),
        }
    }

    // make_interp_spline (B-spline) for k=1,2,3 evaluated via splev-equivalent.
    for k in [1usize, 2, 3] {
        match ip::make_interp_spline(&xi, &yi, k) {
            Ok(bs) => {
                let vals: Vec<f64> = xnew.iter().map(|&x| bs.eval(x)).collect();
                dump(&format!("bspline_k{k}"), &vals);
            }
            Err(e) => println!("bspline_k{k},ERR,{e:?}"),
        }
    }

    // splrep / splev round trip (smoothing s=0 interpolating cubic).
    match ip::splrep(&xi, &yi, 3, 0.0) {
        Ok(tck) => match ip::splev(&xnew, &tck) {
            Ok(vals) => dump("splev_k3", &vals),
            Err(e) => println!("splev_k3,ERR,{e:?}"),
        },
        Err(e) => println!("splrep_k3,ERR,{e:?}"),
    }
}
