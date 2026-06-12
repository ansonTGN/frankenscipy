//! Differential oracle probe: hierarchical linkage vs scipy.cluster.hierarchy (gitignored).
//! Lines: `method,kind,i,value`. kind in {coph, dist}. Inputs match the python comparator.
use fsci_cluster::{cophenet, linkage, LinkageMethod};

fn main() {
    // 8 deterministic 2-D points
    let data = vec![
        vec![0.0, 0.0],
        vec![1.0, 0.5],
        vec![0.3, 2.1],
        vec![4.0, 4.2],
        vec![5.1, 3.8],
        vec![4.5, 5.5],
        vec![9.0, 1.0],
        vec![8.2, 0.3],
    ];
    let methods = [
        ("single", LinkageMethod::Single),
        ("complete", LinkageMethod::Complete),
        ("average", LinkageMethod::Average),
        ("ward", LinkageMethod::Ward),
        ("weighted", LinkageMethod::Weighted),
        ("centroid", LinkageMethod::Centroid),
        ("median", LinkageMethod::Median),
    ];
    for (name, m) in methods {
        match linkage(&data, m) {
            Ok(z) => {
                // merge distances (column 2), in merge order
                for (i, row) in z.iter().enumerate() {
                    println!("{name},dist,{i},{:.17e}", row[2]);
                }
                // cophenetic distances (condensed, convention-independent)
                let c = cophenet(&z);
                for (i, &v) in c.iter().enumerate() {
                    println!("{name},coph,{i},{v:.17e}");
                }
            }
            Err(e) => println!("{name},ERR,{e:?}"),
        }
    }
}
