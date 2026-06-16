use fsci_signal::{buttord, cheb1ord, cheb2ord, ellipord};
fn d(name: &str, r: Result<(u32, f64), fsci_signal::SignalError>) {
    match r {
        Ok((o, w)) => println!("{name},{o},{w:.17e}"),
        Err(e) => println!("{name},ERR,{e:?}"),
    }
}
fn main() {
    // (wp, ws, gpass, gstop) -- lowpass (wp<ws) and highpass (wp>ws)
    let specs = [
        (0.2, 0.3, 1.0, 40.0),
        (0.1, 0.4, 0.5, 60.0),
        (0.3, 0.2, 2.0, 30.0),
        (0.4, 0.15, 1.0, 50.0),
        (0.25, 0.35, 3.0, 45.0),
    ];
    for (i, (wp, ws, gp, gs)) in specs.iter().enumerate() {
        d(&format!("buttord_{i}"), buttord(*wp, *ws, *gp, *gs));
        d(&format!("cheb1ord_{i}"), cheb1ord(*wp, *ws, *gp, *gs));
        d(&format!("cheb2ord_{i}"), cheb2ord(*wp, *ws, *gp, *gs));
        d(&format!("ellipord_{i}"), ellipord(*wp, *ws, *gp, *gs));
    }
}
