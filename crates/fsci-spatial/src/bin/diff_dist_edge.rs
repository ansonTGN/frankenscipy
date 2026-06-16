use fsci_spatial as sp;
fn s(n: &str, v: f64) {
    println!("{n},{v:.17e}");
}
fn main() {
    // canberra/braycurtis with BOTH-zero components (0/0 edge)
    let u = [0.0, 1.0, 0.0, 2.0];
    let v = [0.0, 3.0, 0.0, 2.0];
    s("canberra_00", sp::canberra(&u, &v));
    s("braycurtis_00", sp::braycurtis(&u, &v));
    // cosine of identical
    let a = [1.0, 2.0, 3.0];
    s("cosine_id", sp::cosine(&a, &a));
    s("correlation_id", sp::correlation(&a, &a));
    // jaccard/dice/yule edge cases
    let bu = [true, true, false, false];
    let bv = [true, true, false, false]; // identical
    let fbu = [1.0, 1.0, 0.0, 0.0];
    s("jaccard_id", sp::jaccard(&fbu, &fbu));
    s("dice_id", sp::dice(&bu, &bv));
    s("yule_id", sp::yule(&bu, &bv));
    let af = [false, false, false, false]; // all false both
    let faf = [0.0, 0.0, 0.0, 0.0];
    s("jaccard_allfalse", sp::jaccard(&faf, &faf));
    s("dice_allfalse", sp::dice(&af, &af));
    s("yule_allfalse", sp::yule(&af, &af));
    s("rogerstanimoto_allfalse", sp::rogerstanimoto(&af, &af));
    s("sokalsneath_id", sp::sokalsneath(&bu, &bv));
    // yule with no discordant (b*c=0)
    let y1 = [true, true, true, false];
    let y2 = [true, true, false, false];
    s("yule_nodiscord", sp::yule(&y1, &y2));
    // jaccard float (nonzero/zero pattern)
    let fu = [1.0, 0.0, 2.0, 0.0];
    let fv = [0.0, 0.0, 3.0, 0.0];
    s("jaccard_float", sp::jaccard(&fu, &fv));
    // hamming float
    s("hamming_float", sp::hamming(&fu, &fv));
}
