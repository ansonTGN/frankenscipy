//! Differential oracle probe: distance metrics vs scipy.spatial.distance (gitignored).
//! Lines: `name,value`.
use fsci_spatial as sp;

fn s(name: &str, v: f64) {
    println!("{name},{v:.17e}");
}

fn main() {
    let u = [1.0, 2.0, 3.0, 4.0, 5.0, 0.0, 7.0];
    let v = [2.0, 0.0, 3.0, 1.0, 9.0, 4.0, 6.0];
    s("euclidean", sp::euclidean(&u, &v));
    s("sqeuclidean", sp::sqeuclidean(&u, &v));
    s("cityblock", sp::cityblock(&u, &v));
    s("chebyshev", sp::chebyshev(&u, &v));
    s("cosine", sp::cosine(&u, &v));
    s("correlation", sp::correlation(&u, &v));
    s("canberra", sp::canberra(&u, &v));
    s("braycurtis", sp::braycurtis(&u, &v));
    s("hamming", sp::hamming(&u, &v));
    s("jaccard", sp::jaccard(&u, &v));
    for &p in &[0.5_f64, 1.0, 2.0, 3.0] {
        s(&format!("minkowski_{p}"), sp::minkowski(&u, &v, p));
    }
    // seuclidean with variance vector
    let var = [1.5, 2.0, 0.5, 3.0, 1.0, 2.5, 0.8];
    s("seuclidean", sp::seuclidean(&u, &v, &var));
    // jensenshannon on normalized distributions
    let p = [0.1, 0.2, 0.3, 0.05, 0.15, 0.1, 0.1];
    let q = [0.2, 0.1, 0.1, 0.25, 0.1, 0.15, 0.1];
    s("jensenshannon", sp::jensenshannon(&p, &q, None));
    s("jensenshannon_b2", sp::jensenshannon(&p, &q, Some(2.0)));
    // mahalanobis with a 3-D example
    let x = [1.0, 2.0, 3.0];
    let y = [2.0, 0.0, 5.0];
    let vi = vec![
        vec![1.2, 0.3, 0.1],
        vec![0.3, 0.9, 0.2],
        vec![0.1, 0.2, 1.5],
    ];
    s("mahalanobis", sp::mahalanobis(&x, &y, &vi));
    // boolean metrics
    let bu = [true, false, true, true, false, true, false, false];
    let bv = [true, true, false, true, false, false, true, false];
    s("dice", sp::dice(&bu, &bv));
    s("kulsinski", sp::kulsinski(&bu, &bv));
    s("rogerstanimoto", sp::rogerstanimoto(&bu, &bv));
    s("russellrao", sp::russellrao(&bu, &bv));
    s("sokalmichener", sp::sokalmichener(&bu, &bv));
    s("sokalsneath", sp::sokalsneath(&bu, &bv));
    s("yule", sp::yule(&bu, &bv));
}
