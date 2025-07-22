use rand::Rng;

/// Constants for magic numbers used in random number generation
const MIN_NUMBER: u32 = 1;
const MAX_NUMBER: u32 = 100;

/// Generates a random number between MIN_NUMBER and MAX_NUMBER
/// 
/// # Returns
/// A random u32 between the configured min and max values
/// 
/// # Examples
/// 
/// ```
/// let number = generate_random_number();
/// assert!(number >= MIN_NUMBER && number <= MAX_NUMBER);
/// ```
fn generate_random_number() -> u32 {
    rand::rng().random_range(MIN_NUMBER..=MAX_NUMBER)
}

/// Main function that demonstrates random number generation
/// 
/// Generates a random number and prints it with a greeting message
fn main() {
    let random_number = generate_random_number();
    println!("Hello, world! {}", random_number);
}
