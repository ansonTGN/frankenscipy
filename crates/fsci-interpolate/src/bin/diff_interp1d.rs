//! Differential oracle probe: 1-D interpolators vs scipy.interpolate (gitignored).
//! Lines: `name,i,value`. Uses a fixed (xi, yi, x_new) defined identically in the python cmp.
use fsci_interpolate as ip;

fn main() {
    // sample nodes and values: f(x) = sin(x) + 0.3 x^2 - 0.5 x
    let xi: Vec<f64> = vec![0.0, 0.5, 1.1, 1.7, 2.4, 3.0, 3.8, 4.5, 5.2, 6.0];
    let yi: Vec<f64> = xi
        .iter()
        .map(|&x| x.sin() + 0.3 * x * x - 0.5 * x)
        .collect();
    // query points strictly interior
    let xnew: Vec<f64> = {
        let mut v = Vec::new();
        let mut x = 0.2;
        while x <= 5.9 {
            v.push(x);
            x += 0.17;
        }
        v
    };

    let emit = |name: &str, r: Result<Vec<f64>, ip::InterpError>| match r {
        Ok(vals) => {
            for (i, &v) in vals.iter().enumerate() {
                println!("{name},{i},{v:.17e}");
            }
        }
        Err(e) => println!("{name},ERR,{e:?}"),
    };

    emit("barycentric", ip::barycentric_interpolate(&xi, &yi, &xnew));
    emit("krogh", ip::krogh_interpolate(&xi, &yi, &xnew));
    emit("pchip", ip::pchip_interpolate(&xi, &yi, &xnew));
    emit("akima", ip::akima1d_interpolate(&xi, &yi, &xnew));
}
