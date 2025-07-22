use rand::Rng;

fn main() {
    let random_number = rand::rng().random_range(1..=100);
    println!("Hello, world! {}", random_number);
}
