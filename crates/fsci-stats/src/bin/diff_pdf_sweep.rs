//! direct pdf/cdf shape probe across continuous dists vs scipy (gitignored).
//! Lines: label,fn,x,value
use fsci_stats::*;
fn emit<D: ContinuousDistribution>(label:&str,d:&D,xs:&[f64]){
    for &x in xs { println!("{label},pdf,{x},{:.15e}",d.pdf(x)); println!("{label},cdf,{x},{:.15e}",d.cdf(x)); }
}
fn main(){
    let g=[0.05,0.3,0.7,1.5,3.0,6.0]; // generic positive grid
    let r=[-2.5,-0.7,0.2,1.1,2.8];    // real-line grid
    emit("burr3|4_5",&Burr3::new(4.0,5.0),&g);
    emit("invweibull|4.0",&InvWeibull::new(4.0),&g);
    emit("nct|10_0.5",&NoncentralT::new(10.0,0.5),&r);
    emit("ncx2|4_2",&NoncentralChiSquared::new(4.0,2.0),&g);
    emit("skewnorm|4.0",&SkewNorm::new(4.0),&r);
    emit("tukeylambda|0.3",&TukeyLambda::new(0.3),&r);
    emit("dgamma|1.7",&DoubleGamma::new(1.7),&r);
    emit("loggamma|2.5",&LogGamma::new(2.5),&r);
    emit("johnsonsu|2_1.5",&JohnsonSU::new(2.0,1.5),&r);
    emit("johnsonsb|2_1.5",&JohnsonSB::new(2.0,1.5),&[0.05,0.2,0.5,0.8,0.95]);
    emit("recipinvgauss|0.6",&RecipInvGauss::new(0.6),&g);
    emit("foldcauchy|1.5",&FoldedCauchy::new(1.5),&g);
    emit("powerlognorm|2.1_1.4",&PowerLognorm::new(2.1,1.4),&g);
    emit("gengamma|4.4_3.1",&GenGamma::new(4.4,3.1),&g);
    emit("exponnorm|1.5",&ExponNorm::new(1.5),&r);
    emit("crystalball|2.0_3.0",&CrystalBall::new(2.0,3.0),&r);
    emit("mielke|10.5_3.6",&Mielke::new(10.5,3.6),&g);
    emit("rice|0.7",&Rice::new(0.7),&g);
    emit("argus|1.0",&Argus::new(1.0),&[0.1,0.3,0.5,0.7,0.9]);
    emit("vonmises|2.0",&VonMises::new(2.0,0.0),&[-2.0,-0.5,0.5,2.0]);
    emit("kappa4|0.5_0.3",&Kappa4::new(0.5,0.3),&r);
}
