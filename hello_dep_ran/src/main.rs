use rand::Rng;

/// Constants for magic numbers used in random number generation
const MIN_NUMBER: u32 = 1;
const MAX_NUMBER: u32 = 100;

/// Generates a random number between MIN_NUMBER and MAX_NUMBER
/// 
/// # Returns
/// A Result containing either a random u32 or an error message
/// 
/// # Examples
/// 
/// ```
/// // Documentation example - shows how to handle the Result
/// match generate_random_number() {
///     Ok(number) => println!("Success: {}", number),
///     Err(error) => eprintln!("Error: {}", error),
/// }
/// ```
fn generate_random_number() -> Result<u32, String> {
    // Simulate a potential error (in real code, this might be a network call, file read, etc.)
    if std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() % 10 == 0 {
        return Err("Random error occurred (simulated)".to_string());
    }
    
    Ok(rand::rng().random_range(MIN_NUMBER..=MAX_NUMBER))
}

/// Main function that demonstrates random number generation
/// 
/// Generates a random number and prints it with a greeting message
/// Handles potential errors gracefully
fn main() {
    // Production-ready error handling
    match generate_random_number() {
        Ok(random_number) => {
            println!("Hello, world! {}", random_number);
        }
        Err(error) => {
            eprintln!("Error generating random number: {}", error);
            // In production, you might also:
            // - Log the error to a file
            // - Send error report to monitoring service
            // - Provide user-friendly error message
            // - Exit with appropriate error code
            std::process::exit(1);
        }
    }
}
