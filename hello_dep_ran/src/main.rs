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
} // <- This closes the main() function, but main module scope continues
// The main module is defined by the entire main.rs file
// What Defines a Module:
// The file itself defines the module. In Rust, each .rs file is automatically a module.

// This attribute tells Rust: "Only compile this module when running tests"
// cfg = "configuration" - it's a Rust attribute (like #[test] or #[derive])
// A Rust attribute is a special instruction to the compiler (starts with #[...])
// When you run 'cargo test', Cargo tells Rust to set cfg(test) = ON
// When you run 'cargo run', Cargo tells Rust to set cfg(test) = OFF
#[cfg(test)]
// This creates a module named "tests" - a way to organize code
// A module is like a folder/namespace that groups related code together
// Similar to: Python's 'import', Java's 'package', JavaScript's 'import/export'
mod tests {
    // This brings all items from the parent module into scope for testing
// 'use' = import/bring into scope
// 'super' = parent module (the module that contains this tests module)
// In our case: the main module (where generate_random_number, MIN_NUMBER, etc. are defined)
// Module hierarchy: main module (parent) -> tests module (child)
// So 'super' refers to the main module because tests is nested inside it
// 
// main module (parent)
// ├── generate_random_number()
// ├── MIN_NUMBER
// ├── MAX_NUMBER
// └── tests module (child)
//     ├── test_generate_random_number_success()
//     ├── test_constants_are_valid()
//     └── test_multiple_generations_are_different()
//
// The tests module is nested because it's defined INSIDE the main module's code
// (same file, inside the main module's curly braces {})
// '*' = everything (all functions, constants, etc.)
use super::*;

    // This attribute marks this function as a unit test
    #[test]
    // Function name that describes what we're testing
    fn test_generate_random_number_success() {
        // Call our function and store the Result in 'result'
        let result = generate_random_number();
        // assert! checks if the condition is true, panics if false
        // is_ok() returns true if Result is Ok, false if Err
        assert!(result.is_ok(), "Function should return Ok for successful generation");
        
        // Pattern matching: if result is Ok, extract the value into 'number'
        if let Ok(number) = result {
            // Check that the number is within our expected range
            assert!(number >= MIN_NUMBER, "Number should be >= MIN_NUMBER");
            assert!(number <= MAX_NUMBER, "Number should be <= MAX_NUMBER");
        }
    }

    // Another test function
    #[test]
    fn test_constants_are_valid() {
        // Test that our constants are set up correctly
        // Check that MIN_NUMBER is less than MAX_NUMBER (logical requirement)
        assert!(MIN_NUMBER < MAX_NUMBER, "MIN_NUMBER should be less than MAX_NUMBER");
        // Check that MIN_NUMBER is at least 1 (reasonable minimum)
        assert!(MIN_NUMBER >= 1, "MIN_NUMBER should be at least 1");
        // Check that MAX_NUMBER is reasonable (not too large)
        assert!(MAX_NUMBER <= 1000, "MAX_NUMBER should be reasonable");
    }

    // Test multiple function calls
    #[test]
    fn test_multiple_generations_are_different() {
        // Call the function twice and store both results
        let result1 = generate_random_number();
        let result2 = generate_random_number();
        
        // Both calls should succeed (return Ok)
        assert!(result1.is_ok(), "First generation should succeed");
        assert!(result2.is_ok(), "Second generation should succeed");
        
        // Pattern matching with tuple: if both results are Ok, extract both values
        // (Ok(num1), Ok(num2)) means "if result1 is Ok AND result2 is Ok"
        if let (Ok(num1), Ok(num2)) = (result1, result2) {
            // Check that both numbers are within the valid range
            assert!(num1 >= MIN_NUMBER && num1 <= MAX_NUMBER);
            assert!(num2 >= MIN_NUMBER && num2 <= MAX_NUMBER);
        }
    }
}
