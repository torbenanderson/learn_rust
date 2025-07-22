use rand::Rng;

// Constants for magic numbers
const MIN_NUMBER: u32 = 1;
const MAX_NUMBER: u32 = 100;

fn main() {
    let random_number = rand::rng().random_range(MIN_NUMBER..=MAX_NUMBER);
    println!("Hello, world! {}", random_number);
}
