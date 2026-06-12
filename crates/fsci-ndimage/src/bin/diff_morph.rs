//! Differential oracle probe: grey morphology + label + center_of_mass vs scipy.ndimage (gitignored).
//! Lines: `name,i,value`. Inputs match the python comparator.
use fsci_ndimage::{BoundaryMode, NdArray, center_of_mass, grey_dilation, grey_erosion, label};

fn dump(name: &str, v: &[f64]) {
    for (i, &x) in v.iter().enumerate() {
        println!("{name},{i},{x:.17e}");
    }
}

fn main() {
    // 5x6 greyscale image
    let data = vec![
        1.0, 2.0, 4.0, 7.0, 3.0, 5.0, //
        9.0, 5.0, 8.0, 2.0, 6.0, 1.0, //
        1.0, 4.0, 3.0, 7.0, 5.0, 8.0, //
        8.0, 2.0, 9.0, 1.0, 6.0, 4.0, //
        3.0, 7.0, 2.0, 5.0, 9.0, 2.0,
    ];
    let img = NdArray::new(data, vec![5, 6]).unwrap();
    let modes = [
        ("reflect", BoundaryMode::Reflect),
        ("nearest", BoundaryMode::Nearest),
        ("constant", BoundaryMode::Constant),
        ("wrap", BoundaryMode::Wrap),
    ];
    for (mn, m) in &modes {
        for sz in [2usize, 3] {
            dump(
                &format!("ero_s{sz}_{mn}"),
                &grey_erosion(&img, sz, *m, 0.0).unwrap().data,
            );
            dump(
                &format!("dil_s{sz}_{mn}"),
                &grey_dilation(&img, sz, *m, 0.0).unwrap().data,
            );
        }
    }

    // binary image for label + center_of_mass
    let bin = vec![
        1.0, 1.0, 0.0, 0.0, 1.0, 0.0, //
        1.0, 0.0, 0.0, 1.0, 1.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 1.0, 0.0, 1.0, 1.0, //
        0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
    ];
    let binimg = NdArray::new(bin.clone(), vec![5, 6]).unwrap();
    let (labels, count) = label(&binimg).unwrap();
    println!("label_count,0,{count}");
    dump("label_data", &labels.data);

    // center_of_mass of the whole image (labels all 1 -> single global COM)
    let ones = NdArray::new(vec![1.0; bin.len()], vec![5, 6]).unwrap();
    let com = center_of_mass(&binimg, &ones, 1).unwrap();
    for (i, coord) in com.iter().enumerate() {
        for (k, &c) in coord.iter().enumerate() {
            println!("com,{},{c:.17e}", i * 10 + k);
        }
    }
}
