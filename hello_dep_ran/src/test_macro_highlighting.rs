// Simple Todo Tree Test - Rust Macros
// This file tests Todo Tree highlighting for Rust macros

fn main() {
    println!("Testing Todo Tree macro highlighting");
    
    // These macros should be highlighted in Todo Tree:
    fn test_macros() {
        todo!("Implement this function");
        panic!("This will crash the program");
        unimplemented!("This is not implemented yet");
    }
    
    // Note: These macros never return - they either panic or are placeholders
    // They should appear in Todo Tree sidebar with black text on white background
    
    println!("Test completed");
} 