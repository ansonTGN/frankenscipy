use fsci_stats::*;
fn main(){
    let qs=[1e-10,1e-6,1e-3,0.01,0.5,0.99,1.0-1e-6,1.0-1e-10];
    let nct: Vec<(f64,f64)> = vec![(5.0,2.0),(10.0,5.0),(30.0,1.0)];
    for (df,nc) in &nct { let d=NoncentralT::new(*df,*nc); for &q in &qs { println!("nct,{df},{nc},{q:.17e},{:.17e}",d.ppf(q)); } }
    let ncx2: Vec<(f64,f64)> = vec![(4.0,2.0),(2.0,20.0),(10.0,5.0)];
    for (df,nc) in &ncx2 { let d=NoncentralChiSquared::new(*df,*nc); for &q in &qs { println!("ncx2,{df},{nc},{q:.17e},{:.17e}",d.ppf(q)); println!("ncx2i,{df},{nc},{q:.17e},{:.17e}",d.isf(q)); } }
    let ncf: Vec<(f64,f64,f64)> = vec![(5.0,12.0,3.0),(4.0,20.0,2.0)];
    for (a,b,c) in &ncf { let d=NoncentralF::new(*a,*b,*c); for &q in &qs { println!("ncf,{a}_{b},{c},{q:.17e},{:.17e}",d.ppf(q)); } }
}
