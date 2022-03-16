use rand::distributions::{Distribution, Uniform};
use std::io;

fn is_prime(number: u32) -> bool {
    if number > 1 {
        let step = Uniform::new(2, number);
        let mut rng = rand::thread_rng();
        for _x in [0..3] {
            let randomNumber = step.sample(&mut rng) - 1;
            if (u32::pow(randomNumber, number - 1) % number != 1) {
                return false;
            }
        }
        return true;
    }
    return false;
}
fn main() {
    println!("Hello, world!");
}
