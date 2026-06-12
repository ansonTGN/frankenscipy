use fsci_stats::*;
fn main(){
    // Rice: sweep b (noncentrality); cdf integrates pdf 0..x
    for &b in &[0.5,2.0,5.0,10.0,20.0,40.0]{
        let d=Rice::new(b);
        let m=b.max(1.0);
        for j in 0..12 { let x=(b-3.0).max(0.01)+ (m*0.6)*j as f64/11.0; println!("rice,{b},{x:.6},{:.16e}",d.cdf(x)); }
    }
    // Argus: sweep chi
    for &chi in &[0.3,1.0,3.0,5.0,8.0]{
        let d=Argus::new(chi);
        for j in 0..11 { let x=0.05+0.9*j as f64/10.0; println!("argus,{chi},{x:.6},{:.16e}",d.cdf(x)); }
    }
}
