//! Differential oracle probe: filter representation conversions vs scipy.signal (gitignored).
//! Lines: `name,i,value` (or `name,i,re,im` for complex). Inputs match the python comparator.
use fsci_signal::{BaCoeffs, SosSection, ZpkCoeffs, group_delay, sos2tf, tf2sos, tf2zpk, zpk2tf};

fn dump(name: &str, v: &[f64]) {
    for (i, &x) in v.iter().enumerate() {
        println!("{name},{i},{x:.17e}");
    }
}

fn main() {
    // butter(4, 0.2) lowpass (b, a) from scipy.
    let b = vec![
        0.004824343357716228,
        0.019297373430864913,
        0.02894606014629737,
        0.019297373430864913,
        0.004824343357716228,
    ];
    let a = vec![
        1.0,
        -2.369513007182038,
        2.313988414415881,
        -1.054665405878568,
        0.18737949236818502,
    ];

    // group_delay(b, a) with default 512 freqs.
    if let Ok((w, gd)) = group_delay(&b, &a, None) {
        dump("gd_w", &w);
        dump("gd_val", &gd);
    }

    // tf2zpk -> sorted zeros/poles (by re then im) + gain.
    if let Ok(zpk) = tf2zpk(&b, &a) {
        let mut zeros: Vec<(f64, f64)> = zpk
            .zeros_re
            .iter()
            .zip(&zpk.zeros_im)
            .map(|(&r, &i)| (r, i))
            .collect();
        let mut poles: Vec<(f64, f64)> = zpk
            .poles_re
            .iter()
            .zip(&zpk.poles_im)
            .map(|(&r, &i)| (r, i))
            .collect();
        zeros.sort_by(|x, y| x.partial_cmp(y).unwrap());
        poles.sort_by(|x, y| x.partial_cmp(y).unwrap());
        for (i, (r, im)) in zeros.iter().enumerate() {
            println!("zpk_zero,{i},{r:.17e},{im:.17e}");
        }
        for (i, (r, im)) in poles.iter().enumerate() {
            println!("zpk_pole,{i},{r:.17e},{im:.17e}");
        }
        println!("zpk_gain,0,{:.17e}", zpk.gain);
    }

    // zpk2tf round trip: feed a fixed zpk (the analog-ish set), compare b,a.
    let zpk = ZpkCoeffs {
        zeros_re: vec![-1.0, -1.0, 0.5],
        zeros_im: vec![0.0, 0.0, 0.0],
        poles_re: vec![0.3, 0.3, -0.4],
        poles_im: vec![0.5, -0.5, 0.0],
        gain: 0.75,
    };
    let ba: BaCoeffs = zpk2tf(&zpk);
    dump("zpk2tf_b", &ba.b);
    dump("zpk2tf_a", &ba.a);

    // tf2sos -> sos2tf round trip should recover (b, a) up to scaling.
    if let Ok(sos) = tf2sos(&b, &a) {
        let flat: Vec<f64> = sos
            .iter()
            .flat_map(|s: &SosSection| s.iter().copied())
            .collect();
        dump("tf2sos_flat", &flat);
        let recon = sos2tf(&sos);
        dump("sos2tf_b", &recon.b);
        dump("sos2tf_a", &recon.a);
    }
}
