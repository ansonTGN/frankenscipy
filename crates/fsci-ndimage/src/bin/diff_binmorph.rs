use fsci_ndimage::{NdArray, binary_closing, binary_dilation, binary_erosion, binary_opening};
fn dump(name: &str, v: &[f64]) {
    for (i, &x) in v.iter().enumerate() {
        println!("{name},{i},{x:.1}");
    }
}
fn main() {
    let bin = vec![
        0., 0., 0., 0., 0., 0., 0., 0., 1., 1., 1., 0., 1., 0., 0., 1., 1., 1., 1., 1., 0., 0., 1.,
        1., 0., 0., 1., 0., 0., 0., 1., 1., 1., 1., 0., 0., 0., 0., 0., 0., 0., 0.,
    ];
    let img = NdArray::new(bin, vec![6, 7]).unwrap();
    for it in [1usize, 2] {
        dump(
            &format!("ero3_it{it}"),
            &binary_erosion(&img, 3, it).unwrap().data,
        );
        dump(
            &format!("dil3_it{it}"),
            &binary_dilation(&img, 3, it).unwrap().data,
        );
    }
    dump("open3", &binary_opening(&img, 3, 1).unwrap().data);
    dump("close3", &binary_closing(&img, 3, 1).unwrap().data);
}
