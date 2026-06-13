use fsci_stats::{CrystalBall, ContinuousDistribution};
fn main(){
    for &(b,m) in &[(1.5,3.0),(2.0,5.0),(1.0,2.5)] {
        let d=CrystalBall::new(b,m);
        for &x in &[-10.0,-3.0,0.0,2.0,20.0,38.6,60.0] {
            println!("cb,{b},{m},{x},{:.17e}",d.logpdf(x));
        }
    }
}
