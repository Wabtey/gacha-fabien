use rand::{thread_rng, Rng};

pub fn roll() -> i32{
    let mut rng = thread_rng();

    let reward = rng.gen_range(0..5);

    reward
}