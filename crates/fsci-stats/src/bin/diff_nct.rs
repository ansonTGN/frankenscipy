use fsci_stats::*;
fn main(){
    for &df in &[2.0,5.0,10.0,30.0,100.0,300.0]{
        for &nc in &[0.5,2.0,5.0]{
            let d=NoncentralT::new(df,nc);
            for &t in &[-1.0,0.0,nc,nc+2.0,nc+5.0,10.0]{
                println!("{df},{nc},{t},{:.16e}",d.cdf(t));
            }
        }
    }
}
