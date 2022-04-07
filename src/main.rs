use rand::{thread_rng, Rng};
mod gacha_bank;

// use gacha_bank::*;


fn roll(){
    let mut rng = thread_rng();
    let rand_result = rng.gen_range(1..5);
    let stamp_result =
        match rand_result {
            // 1 => gacha_bank::monkey_path,
            // _ => ""
            _ => gacha_bank::monkey_path
        };
} 

pub fn rolls(amount: i32){
    let results: vec![&str; amount];
    for i in 0..amount {
        results[i]=roll();
    }
    results
}

fn main() {
    let fonc1bool: bool = true;
    let fonc1: &str = 
    match fonc1bool {
        true => "OK",
        _ => "NOT OK"
    };
    println!("GACHA ON");
    let results: vec![&str, i32] = rolls(100);
    println!("fonctionality One, RUNNNG : {} ", fonc1);
}
