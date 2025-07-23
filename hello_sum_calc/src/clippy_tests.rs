// Clippy Demo - Testing Different Warning Types
// This file demonstrates various Clippy warnings and how to test them

fn main() {
    // Test 1: Disallowed/placeholder name warning
    // Clippy warns: "use of a disallowed/placeholder name `foo`"
    let foo: i32 = 5;

    // Test 2: Unused variable warning
    // Clippy warns: "unused variable: `bar`" and suggests: let _bar = 10;
    let bar: i32 = 10; // This should show unused variable warning

    // Test 3: Variable is used, so no unused warning
    // foo is used in println!, so no unused variable warning
    println!("{foo}");

    // Test 4: Intentional unused variable (no warning)
    let _intentional = 42; // No warning - underscore prefix

    // Test 5: Discarding return value (no warning)
    let _ = 100; // No warning - discarding value

    // Test 6: Meaningful variable name (no warnings)
    let user_count: i32 = 5;
    println!("User count: {user_count}"); // No warnings - meaningful name

    // Call some_function to make it used (but it has dead code inside)
    some_function();

    // Test 9: Unnecessary type annotation
    let x: i32 = 5; // Clippy suggests: let x = 5; (type can be inferred)

    // Test 10: Redundant clone
    let vec1 = vec![1, 2, 3];
    let vec2 = vec1.clone(); // Clippy suggests: let vec2 = vec1; (if vec1 not used after)

    // Test 11: Unnecessary parentheses
    let result = (2 + 3) * 4; // Clippy suggests: let result = 2 + 3 * 4;

    // Test 12: Single character variable name
    let a = 42; // Clippy suggests: use a more descriptive name
}

// Test 7: Unused function warning
fn unused_function() {
    println!("This function is never called"); // Will show unused function warning
}

// Test 8: Dead code inside function (different from unused function)
fn some_function() {
    println!("This function is called");
    println!("This line is never reached"); // Dead code - unreachable
    return; // Function ends here
    println!("This will never execute"); // Dead code - after return
}
