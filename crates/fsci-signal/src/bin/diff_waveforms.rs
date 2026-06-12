//! Differential oracle probe: signal waveforms vs scipy.signal (gitignored).
//! Lines: `name,arg,i,value`. Run via release-perf; pipe to python comparator.
use fsci_signal as sig;
use fsci_signal::ChirpMethod;

fn dump(name: &str, arg: &str, v: &[f64]) {
    for (i, &x) in v.iter().enumerate() {
        println!("{name},{arg},{i},{x:.17e}");
    }
}

fn main() {
    let t: Vec<f64> = (0..200).map(|i| i as f64 * 0.01).collect(); // 0..2 s
    dump(
        "chirp_linear",
        "0",
        &sig::chirp(&t, 6.0, 1.0, 2.0, ChirpMethod::Linear).unwrap(),
    );
    dump(
        "chirp_quadratic",
        "0",
        &sig::chirp(&t, 6.0, 1.0, 2.0, ChirpMethod::Quadratic).unwrap(),
    );
    dump(
        "chirp_logarithmic",
        "0",
        &sig::chirp(&t, 6.0, 1.0, 2.0, ChirpMethod::Logarithmic).unwrap(),
    );
    dump("sawtooth_1", "1.0", &sig::sawtooth(&t, 1.0).unwrap());
    dump("sawtooth_0.5", "0.5", &sig::sawtooth(&t, 0.5).unwrap());
    dump("square_0.5", "0.5", &sig::square(&t, 0.5).unwrap());
    dump("square_0.3", "0.3", &sig::square(&t, 0.3).unwrap());
    // gausspulse on symmetric t about 0
    let t2: Vec<f64> = (0..200).map(|i| (i as f64 - 100.0) * 0.005).collect();
    dump("gausspulse", "5|0.5", &sig::gausspulse(&t2, 5.0, 0.5));
    dump("sweep_poly", "poly", &sig::sweep_poly(&t, &[0.025, -0.36, 1.25, 2.0]));
}
