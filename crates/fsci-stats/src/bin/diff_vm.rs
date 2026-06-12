use fsci_stats::*;
fn main(){
    for &k in &[0.5,1.0,2.0,5.0,10.0,50.0]{
        let d=VonMises::new(k,0.0);
        let mut x=-3.14159;
        while x<3.14159 { println!("vm,{k},{x:.6},{:.16e}",d.cdf(x)); x+=0.25; }
    }
}
