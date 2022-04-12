use bevy::prelude::*;
use std::fs;
use rand::{thread_rng, Rng};
mod randomness;
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

    App::new().run();
    // let object = "Pins";

    // let mut history = format!("{} History : ", object);
    // for i in 1..50{
    //     let reward = randomness::roll();
    //     let roll_n = format!("\n - roll {} : {}", i, reward);
    //     history.push_str(&roll_n);
    // }

    
    
    // /run/media/wabtey/WD ELEMENT/Code/projects_rust/gacha-fabien/roll/BasicRoll.txt

    // fs::write("E:/Code/projects_rust/gacha-fabien/roll/BasicRoll.txt",
	// 		  history.to_string())
	// 	.expect("Unable to write file");

    /*
    let fonc1bool: bool = true;
    let fonc1: &str = 
    match fonc1bool {
        true => "OK",
        _ => "NOT OK"
    };
    println!("GACHA ON");
    let results: vec![&str, i32] = rolls(100);
    println!("fonctionality One, RUNNNG : {} ", fonc1);
    */

}
