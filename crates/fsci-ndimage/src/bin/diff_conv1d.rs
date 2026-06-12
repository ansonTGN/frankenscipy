use fsci_ndimage::{BoundaryMode, NdArray, convolve1d, convolve1d_with_origin};
fn dump(name: &str, v: &[f64]) {
    let s: Vec<String> = v.iter().map(|x| format!("{x:.17e}")).collect();
    println!("{name}|{}", s.join(";"));
}
fn main() {
    let input = NdArray::new(vec![1.0, 2.0, 4.0, 7.0, 3.0, 9.0, 5.0, 8.0], vec![8]).unwrap();
    let modes = [
        ("reflect", BoundaryMode::Reflect),
        ("constant", BoundaryMode::Constant),
        ("nearest", BoundaryMode::Nearest),
        ("wrap", BoundaryMode::Wrap),
    ];
    let kernels: &[(&str, Vec<f64>)] = &[
        ("k3", vec![1.0, 2.0, -1.0]),
        ("k4", vec![0.5, 1.0, -0.5, 2.0]),
    ];
    for (kn, kw) in kernels {
        for (mn, m) in &modes {
            dump(
                &format!("conv_{kn}_{mn}"),
                &convolve1d(&input, kw, 0, *m, 0.5).unwrap().data,
            );
        }
    }
    for origin in [-1i64, 1] {
        dump(
            &format!("conv_k3_reflect_o{origin}"),
            &convolve1d_with_origin(
                &input,
                &[1.0, 2.0, -1.0],
                0,
                BoundaryMode::Reflect,
                0.0,
                origin,
            )
            .unwrap()
            .data,
        );
    }
}
