use fsci_integrate::newton_cotes;
fn main() {
    for n in [1usize, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
        match newton_cotes(n) {
            Ok(w) => {
                // ours sums to 1; scipy sums to n => scale by n
                let s: Vec<String> = w
                    .iter()
                    .map(|x| format!("{:.17e}", x * (n as f64)))
                    .collect();
                println!("nc,{n},{}", s.join(";"));
            }
            Err(e) => println!("nc,{n},ERR:{e:?}"),
        }
    }
}
