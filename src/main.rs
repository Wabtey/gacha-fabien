use std::fs;

mod randomness;

fn main() {
    let object = "Pins";

    let mut history = format!("{} History : ", object);
    for i in 1..50{
        let reward = randomness::roll();
        let roll_n = format!("\n - roll {} : {}", i, reward);
        history.push_str(&roll_n);
    }

    
    

    fs::write("E:/Code/projects Rust/gacha-fabien/roll/BasicRoll.txt",
			  history.to_string())
		.expect("Unable to write file");
}
