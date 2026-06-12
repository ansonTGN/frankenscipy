//! 2nd exotic continuous-dist entropy/moments probe vs scipy (gitignored).
use fsci_stats::*;
fn emit<D: ContinuousDistribution>(label:&str,d:&D){
    println!("{label},entropy,{:.17e}",d.entropy());
    println!("{label},skew,{:.17e}",d.skewness());
    println!("{label},kurt,{:.17e}",d.kurtosis());
    println!("{label},mean,{:.17e}",d.mean());
    println!("{label},var,{:.17e}",d.var());
}
fn main(){
    emit("foldcauchy|1.5",&FoldedCauchy::new(1.5));
    emit("foldnorm|1.5",&FoldedNormal::new(1.5));
    emit("johnsonsb|2_1.5",&JohnsonSB::new(2.0,1.5));
    emit("johnsonsu|2_1.5",&JohnsonSU::new(2.0,1.5));
    emit("kappa4|0.5_0.3",&Kappa4::new(0.5,0.3));
    emit("recipinvgauss|0.6",&RecipInvGauss::new(0.6));
    emit("skewcauchy|0.5",&SkewCauchy::new(0.5));
    emit("laplaceasym|2.0",&LaplaceAsymmetric::new(2.0));
    emit("loglaplace|3.0",&LogLaplace::new(3.0));
    emit("loglogistic|4.0",&Loglogistic::new(4.0));
    emit("powerlognorm|2.1_1.4",&PowerLognorm::new(2.1,1.4));
    emit("powernorm|4.5",&PowerNorm::new(4.5));
    emit("pearson3|0.8",&Pearson3::new(0.8));
    emit("gengamma|4.4_3.1",&GenGamma::new(4.4,3.1));
    emit("doubleweibull|2.07",&DoubleWeibull::new(2.07));
    emit("fatiguelife|2.0",&FatigueLife::new(2.0));
    emit("exponnorm|1.5",&ExponNorm::new(1.5));
    emit("exponweib|2.0_2.0",&ExponWeibull::new(2.0,2.0));
    emit("crystalball|2.0_3.0",&CrystalBall::new(2.0,3.0));
}
