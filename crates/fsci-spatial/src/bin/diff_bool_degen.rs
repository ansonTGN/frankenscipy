use fsci_spatial as sp;
fn s(n: &str, v: f64) {
    println!("{n},{v:.17e}");
}
fn main() {
    let af = [false, false, false, false];
    let at = [true, true, true, true];
    let id = [true, false, true, false];
    let cases: &[(&str, &[bool], &[bool])] = &[
        ("allfalse", &af, &af),
        ("alltrue", &at, &at),
        ("identical", &id, &id),
    ];
    for (lbl, u, v) in cases {
        s(&format!("dice_{lbl}"), sp::dice(u, v));
        s(&format!("yule_{lbl}"), sp::yule(u, v));
        s(&format!("rogerstanimoto_{lbl}"), sp::rogerstanimoto(u, v));
        s(&format!("russellrao_{lbl}"), sp::russellrao(u, v));
        s(&format!("sokalsneath_{lbl}"), sp::sokalsneath(u, v));
        s(&format!("matching_{lbl}"), sp::matching(u, v));
    }
}
