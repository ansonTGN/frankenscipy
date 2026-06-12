use fsci_stats::*;
fn main(){
    for &(a,b) in &[(1.0,0.0),(2.0,1.0),(5.0,2.0),(10.0,-5.0),(20.0,10.0),(50.0,0.0)]{
        let d=NormInvGauss::new(a,b);
        for &x in &[-5.0,-1.0,0.0,0.5,2.0,8.0]{ println!("nig,{a},{b},{x},{:.16e}",d.cdf(x)); }
    }
    for &rho in &[1.0,10.0,36.0,100.0,300.0]{
        let d=RelBreitWigner::new(rho);
        for j in 0..8 { let x=rho*0.3 + rho*1.5*j as f64/7.0; println!("relbw,{rho},0,{x:.6},{:.16e}",d.cdf(x)); }
    }
}
