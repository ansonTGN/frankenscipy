//! Differential oracle probe: sobel/prewitt/laplace 2-D vs scipy.ndimage (gitignored).
use fsci_ndimage::{BoundaryMode, NdArray, laplace, prewitt, sobel};

fn dump(name: &str, v: &[f64]) {
    let s: Vec<String> = v.iter().map(|x| format!("{x:.17e}")).collect();
    println!("{name}|{}", s.join(";"));
}

fn main() {
    // 4x5 image
    let data = vec![
        1.0, 2.0, 4.0, 7.0, 3.0, //
        9.0, 5.0, 8.0, 2.0, 6.0, //
        1.0, 4.0, 3.0, 7.0, 5.0, //
        8.0, 2.0, 9.0, 1.0, 6.0,
    ];
    let input = NdArray::new(data, vec![4, 5]).unwrap();
    let modes = [
        ("reflect", BoundaryMode::Reflect),
        ("constant", BoundaryMode::Constant),
        ("nearest", BoundaryMode::Nearest),
        ("wrap", BoundaryMode::Wrap),
    ];
    for (mn, m) in &modes {
        for axis in [0usize, 1] {
            dump(
                &format!("sobel_a{axis}_{mn}"),
                &sobel(&input, axis, *m, 0.0).unwrap().data,
            );
            dump(
                &format!("prewitt_a{axis}_{mn}"),
                &prewitt(&input, axis, *m, 0.0).unwrap().data,
            );
        }
        dump(
            &format!("laplace_{mn}"),
            &laplace(&input, *m, 0.0).unwrap().data,
        );
    }
}
