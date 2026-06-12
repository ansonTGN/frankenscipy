use fsci_spatial as sp;
fn main(){
    let af=[false,false,false,false];
    let bu=[true,false,true,true,false,true,false,false];
    let bv=[true,true,false,true,false,false,true,false];
    println!("dice_allfalse,{}", sp::dice(&af,&af));
    println!("sokalsneath_allfalse,{}", sp::sokalsneath(&af,&af));
    println!("dice_normal,{:.17e}", sp::dice(&bu,&bv));
    println!("sokalsneath_normal,{:.17e}", sp::sokalsneath(&bu,&bv));
}
