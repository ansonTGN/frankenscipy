use fsci_stats::*;
fn main() {
    let d = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    for n in 1..=4 {
        println!("kstat{n},{:.17e},0", kstat(&d, n));
    }
    for n in 1..=2 {
        println!("kstatvar{n},{:.17e},0", kstatvar(&d, n));
    }
    let x = [0.1, 1.2, 2.5, 3.9, 5.1, 6.0, 0.5];
    println!("circmean,{:.17e},0", circmean(&x));
    println!("circvar,{:.17e},0", circvar(&x));
    println!("circstd,{:.17e},0", circstd(&x));
    // variation, gstd, sem
    let p = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    println!("variation,{:.17e},0", variation(&p));
    println!("gstd,{:.17e},0", gstd(&p));
    println!("sem,{:.17e},0", sem(&p));
}
