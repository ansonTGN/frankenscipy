//! FITPACK spline-op probe (gitignored): prints fsci's tck + op results so a
//! python comparator can feed the SAME tck to scipy and diff the operations.
use fsci_interpolate::{splder, splev, splint, splrep, sproot};
fn main() {
    let xs: Vec<f64> = (0..11).map(|i| i as f64 * 0.7).collect();
    let ys: Vec<f64> = xs.iter().map(|&x| (x * 0.8).sin() + 0.1 * x).collect();
    let tck = splrep(&xs, &ys, 3, 0.0).expect("splrep");
    let (t, c, k) = &tck;
    println!(
        "t,{}",
        t.iter()
            .map(|v| format!("{v:.17e}"))
            .collect::<Vec<_>>()
            .join(";")
    );
    println!(
        "c,{}",
        c.iter()
            .map(|v| format!("{v:.17e}"))
            .collect::<Vec<_>>()
            .join(";")
    );
    println!("k,{k}");
    let ev: Vec<f64> = (0..30).map(|i| i as f64 * 0.23).collect();
    let sv = splev(&ev, &tck).unwrap();
    println!(
        "splev,{}",
        sv.iter()
            .map(|v| format!("{v:.17e}"))
            .collect::<Vec<_>>()
            .join(";")
    );
    println!("splint,{:.17e}", splint(0.5, 6.5, &tck).unwrap());
    let d = splder(&tck).unwrap();
    let dv = splev(&ev, &d).unwrap();
    println!(
        "splder_ev,{}",
        dv.iter()
            .map(|v| format!("{v:.17e}"))
            .collect::<Vec<_>>()
            .join(";")
    );
    let roots = sproot(&tck).unwrap();
    println!(
        "sproot,{}",
        roots
            .iter()
            .map(|v| format!("{v:.17e}"))
            .collect::<Vec<_>>()
            .join(";")
    );
}
