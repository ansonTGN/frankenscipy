//! uncovered window probe (taylor/exponential/general_cosine/kbd/dpss) vs scipy (gitignored).
//! Lines: name,arg,n,i,value
use fsci_signal as sig;
fn dump(name: &str, arg: &str, w: &[f64]) {
    let n = w.len();
    for (i, &v) in w.iter().enumerate() {
        println!("{name},{arg},{n},{i},{v:.17e}");
    }
}
fn main() {
    for &n in &[8usize, 16, 31, 64, 101] {
        dump(
            "taylor_4_30",
            &format!("{n}"),
            &sig::taylor(n, 4, 30.0, true, true),
        );
        dump(
            "taylor_6_60",
            &format!("{n}"),
            &sig::taylor(n, 6, 60.0, true, true),
        );
        dump(
            "taylor_nonorm",
            &format!("{n}"),
            &sig::taylor(n, 4, 30.0, false, true),
        );
        dump(
            "gencos",
            &format!("{n}"),
            &sig::general_cosine(n, &[0.5, 0.5], true),
        );
        if let Ok(w) = sig::exponential(n, None, 1.0, true) {
            dump("expo_tau1", &format!("{n}"), &w);
        }
        if let Ok(w) = sig::exponential(n, None, 3.0, true) {
            dump("expo_tau3", &format!("{n}"), &w);
        }
        if n % 2 == 0
            && let Ok(w) = sig::kaiser_bessel_derived(n, 8.0, true)
        {
            dump("kbd_8", &format!("{n}"), &w);
        }
        for &nw in &[2.5_f64, 4.0] {
            if let Ok(r) = sig::dpss(n, nw, None, true, None, false)
                && let Some(window) = r.windows.first()
            {
                dump(&format!("dpss_{nw}"), &format!("{n}"), window);
            }
        }
    }
}
