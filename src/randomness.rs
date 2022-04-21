use rand::{thread_rng, Rng};

pub fn roll(range: i32) -> i32{
    let mut rng = thread_rng();

    let reward = rng.gen_range(0..range);

    reward
}