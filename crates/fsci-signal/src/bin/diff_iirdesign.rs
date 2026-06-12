//! Differential oracle probe: IIR filter design vs scipy.signal (gitignored).
//! Lines: `name|b0;b1;..|a0;a1;..` or `name|ERR`.
use fsci_signal::{bessel, butter, cheby1, cheby2, ellip, BaCoeffs, FilterType, SignalError};

fn dump(name: &str, r: Result<BaCoeffs, SignalError>) {
    match r {
        Ok(c) => {
            let b: Vec<String> = c.b.iter().map(|x| format!("{x:.17e}")).collect();
            let a: Vec<String> = c.a.iter().map(|x| format!("{x:.17e}")).collect();
            println!("{name}|{}|{}", b.join(";"), a.join(";"));
        }
        Err(e) => println!("{name}|ERR:{e:?}"),
    }
}

fn main() {
    use FilterType::*;
    let lp = [0.3_f64];
    let hp = [0.4_f64];
    let bp = [0.2_f64, 0.5];
    let bs = [0.25_f64, 0.6];
    for order in [2usize, 3, 4, 5] {
        dump(&format!("butter_lp_{order}"), butter(order, &lp, Lowpass));
        dump(&format!("butter_hp_{order}"), butter(order, &hp, Highpass));
        dump(&format!("butter_bp_{order}"), butter(order, &bp, Bandpass));
        dump(&format!("butter_bs_{order}"), butter(order, &bs, Bandstop));
        dump(&format!("cheby1_lp_{order}"), cheby1(order, 1.0, &lp, Lowpass));
        dump(&format!("cheby1_bp_{order}"), cheby1(order, 1.0, &bp, Bandpass));
        dump(&format!("cheby2_lp_{order}"), cheby2(order, 40.0, &lp, Lowpass));
        dump(&format!("cheby2_hp_{order}"), cheby2(order, 40.0, &hp, Highpass));
        dump(&format!("ellip_lp_{order}"), ellip(order, 1.0, 40.0, &lp, Lowpass));
        dump(&format!("ellip_bp_{order}"), ellip(order, 1.0, 40.0, &bp, Bandpass));
        dump(&format!("bessel_lp_{order}"), bessel(order, &lp, Lowpass));
        dump(&format!("bessel_hp_{order}"), bessel(order, &hp, Highpass));
    }
}
