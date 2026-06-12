//! 3rd exotic continuous-dist entropy/moments probe vs scipy (gitignored).
use fsci_stats::*;
fn emit<D: ContinuousDistribution>(label:&str,d:&D){
    println!("{label},entropy,{:.17e}",d.entropy());
    println!("{label},skew,{:.17e}",d.skewness());
    println!("{label},kurt,{:.17e}",d.kurtosis());
    println!("{label},mean,{:.17e}",d.mean());
    println!("{label},var,{:.17e}",d.var());
}
fn main(){
    emit("betaprime|3_5",&BetaPrime::new(3.0,5.0));
    emit("burr3|4_5",&Burr3::new(4.0,5.0));
    emit("invweibull|4.0",&InvWeibull::new(4.0));
    emit("frechet|3.5",&FrechetR::new(3.5));
    emit("nct|10_0.5",&NoncentralT::new(10.0,0.5));
    emit("ncf|5_12_3",&NoncentralF::new(5.0,12.0,3.0));
    emit("ncx2|4_2",&NoncentralChiSquared::new(4.0,2.0));
    emit("relbw|36",&RelBreitWigner::new(36.0));
    emit("skewnorm|4.0",&SkewNorm::new(4.0));
    emit("truncpareto|2_5",&TruncPareto::new(2.0,5.0));
    emit("tukeylambda|0.3",&TukeyLambda::new(0.3));
    emit("vonmises|2.0",&VonMises::new(2.0,0.0));
    emit("dgamma|1.7",&DoubleGamma::new(1.7));
    emit("loggamma|2.5",&LogGamma::new(2.5));
    emit("powerlaw|1.7",&PowerLaw::new(1.7));
    emit("rdist|4.0",&RDist::new(4.0));
}
