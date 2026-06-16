//! exotic continuous-dist entropy/skew/kurt probe vs scipy (gitignored).
use fsci_stats::*;
fn emit<D: ContinuousDistribution>(label: &str, d: &D) {
    println!("{label},entropy,{:.17e}", d.entropy());
    println!("{label},skew,{:.17e}", d.skewness());
    println!("{label},kurt,{:.17e}", d.kurtosis());
    println!("{label},mean,{:.17e}", d.mean());
    println!("{label},var,{:.17e}", d.var());
}
fn main() {
    emit("truncexpon|2", &TruncExpon::new(2.0));
    emit("truncexpon|0.7", &TruncExpon::new(0.7));
    emit("exponpow|2.7", &ExponPow::new(2.7));
    emit("fisk|3.1", &Fisk::new(3.1));
    emit("lomax|4.5", &Lomax::new(4.5));
    emit("bradford|1.3", &Bradford::new(1.3));
    emit("genhalflogistic|0.8", &GenHalfLogistic::new(0.8));
    emit("halfgennorm|2.5", &HalfGenNorm::new(2.5));
    emit("mielke|10.5_3.6", &Mielke::new(10.5, 3.6));
    emit("burr12|4.0_5.0", &Burr12::new(4.0, 5.0));
    emit("kappa3|2.0", &Kappa3::new(2.0));
    emit("alpha|3.5", &Alpha::new(3.5));
    emit("argus|1.0", &Argus::new(1.0));
    emit("rice|0.7", &Rice::new(0.7));
}
