# Hello Sum Calc

A simple Rust project demonstrating Clippy linting and rust-analyzer configuration.

## Purpose

This project was created to test and demonstrate:
- Clippy linting warnings in the editor
- Proper rust-analyzer configuration
- Common Rust coding issues that Clippy catches

## Clippy Configuration

### Settings Fixed

The project helped identify and fix rust-analyzer configuration issues:

**Before (incorrect):**
```json
"rust-analyzer.checkOnSave": {
    "command": "clippy"
}
```

**After (correct):**
```json
"rust-analyzer.checkOnSave": true,
"rust-analyzer.check.command": "clippy"
```

### What This Enables

- **Real-time linting**: Clippy warnings appear in the editor as you type
- **Save-time checking**: Code is automatically checked when you save files
- **Better development experience**: Catch issues before running `cargo check`

## Project Structure

This project demonstrates both clean Rust code and Clippy testing:

### Main Application (`src/main.rs`)
Clean, production-ready code that calculates sums:

```rust
fn calculate_sum(numbers: &[i32]) -> Result<i32, String> {
    let sum = numbers.iter().sum();
    Ok(sum)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    match calculate_sum(&numbers) {
        Ok(sum) => println!("Sum: {}", sum),
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

### Clippy Tests (`src/clippy_tests.rs`)
A comprehensive testing playground with 12 different types of Clippy warnings:

```rust
fn main() {
    let foo: i32 = 5;                    // Test 1: Disallowed/placeholder name
    let bar: i32 = 10;                    // Test 2: Unused variable
    println!("{foo}");                    // Test 3: Variable usage (no unused warning)
    let _intentional = 42;                // Test 4: Intentional unused (no warning)
    let _ = 100;                          // Test 5: Discarding value (no warning)
    let user_count: i32 = 5;              // Test 6: Meaningful name (no warnings)
    let x: i32 = 5;                       // Test 9: Unnecessary type annotation
    let vec2 = vec1.clone();              // Test 10: Redundant clone
    let result = (2 + 3) * 4;             // Test 11: Unnecessary parentheses
    let a = 42;                           // Test 12: Single character name
}
```

### Expected Warnings in clippy_tests.rs (8 total)

1. **Disallowed/placeholder name**: `foo` - Clippy suggests more descriptive names
2. **Unused variable**: `bar` - Clippy suggests `_bar` (prefix with underscore)
3. **Unused variable**: `x` - Unnecessary type annotation when type can be inferred
4. **Unused variable**: `vec2` - Redundant clone operation
5. **Unused variable**: `result` - Unnecessary parentheses
6. **Unused variable**: `a` - Single character variable name
7. **Unused function**: `unused_function` - Function is never called
8. **Unreachable code**: In `some_function()` - Code after `return` statement

### Underscore Prefix Usage

The `_` prefix is a Rust convention for silencing unused variable warnings:
- `let bar = 10;` → Warning: unused variable
- `let _bar = 10;` → No warning (intentionally unused)
- `let _ = some_function();` → Discard return value without binding to a variable

## Learning Outcomes

- Understanding how rust-analyzer integrates with Clippy
- Seeing real-time linting feedback in the editor
- Learning to configure development tools properly
- Recognizing common Rust coding patterns that trigger warnings

## Running the Project

```bash
# Run the main sum calculator
cargo run      # Runs the sum calculator program

# Check for warnings in main code
cargo check    # Should show no warnings (clean code)

# Test Clippy warnings in the test file
cargo clippy   # Shows warnings in clippy_tests.rs

# Run Clippy on specific test file
rustc src/clippy_tests.rs --crate-type bin -o clippy_tests && ./clippy_tests
```

## IDE Integration

With the correct settings, you should see:
- Yellow squiggly lines under unused variables
- Hover tooltips showing Clippy suggestions
- Quick-fix options in the editor
- Warnings in the Problems panel

## Clippy Modernization Wave

This project also demonstrates how Clippy can be used to modernize existing code across projects.

### Improvements Applied Across Projects

**hello_dep_ran/src/main.rs:**
- **uninlined_format_args**: Updated to use captured identifiers (`{random_number}` instead of `{}`)
- **manual_range_contains**: Replaced manual range checks with `contains()` method
- **assertions_on_constants**: Removed redundant constant assertions

**hello_sum_calc/src/main.rs:**
- **uninlined_format_args**: Updated to use captured identifiers (`{sum}` instead of `{}`)

### Learning Outcomes

- **Modern Rust 2021 features**: Captured identifiers in format strings
- **Idiomatic patterns**: Using built-in range methods instead of manual comparisons
- **Code quality**: Avoiding redundant checks that the compiler already handles
- **Tool integration**: How Clippy helps improve existing code

## GitHub Actions CI/CD

This project includes a comprehensive GitHub Actions workflow for continuous integration and deployment.

### Workflow Configuration

The workflow (`.github/workflows/rust-ci.yml`) runs on every push to main and pull request:

```yaml
name: Rust CI
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
      
      - name: Set up Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install cargo-audit
        run: cargo install cargo-audit
      
      - name: Build
        run: cargo build --verbose
      
      - name: Test
        run: cargo test --verbose
      
      - name: Clippy
        run: cargo clippy --verbose -- -D warnings
      
      - name: Audit
        run: cargo audit
```

### Workflow Steps

1. **Checkout**: Downloads the repository code
2. **Setup Rust**: Installs the stable Rust toolchain
3. **Install cargo-audit**: Adds security vulnerability scanning
4. **Build**: Compiles all workspace projects
5. **Test**: Runs all unit tests across projects
6. **Clippy**: Runs linting with warnings treated as errors
7. **Audit**: Scans dependencies for security vulnerabilities

### Local Testing with Act

You can test the GitHub Actions workflow locally using `act`:

```bash
# Install act (if not already installed)
brew install act

# Test the workflow locally
act push

# Minimal output (like GitHub's interface)
act push --quiet

# Reuse Docker containers for faster testing
act push --quiet --reuse

# Suppress M1 Mac warnings
act push --quiet --container-architecture linux/amd64

# Test specific workflow
act -W .github/workflows/rust-ci.yml push
```

### Act Configuration Tips

**For Apple M1/M2 Macs:**
```bash
# Use AMD64 architecture to avoid compatibility issues
act push --container-architecture linux/amd64
```

**For faster testing:**
```bash
# Reuse containers between runs
act push --reuse

# Skip Docker image updates
act push --no-pull
```

**For debugging:**
```bash
# Verbose output for troubleshooting
act push --verbose

# List available actions
act -l
```

### Workspace Benefits

The workspace configuration enables:
- **Single command builds**: `cargo build` builds all projects
- **Unified testing**: `cargo test` runs tests across all projects
- **Consistent dependencies**: All projects use the same dependency versions
- **Simplified CI/CD**: One workflow handles all projects

### GitHub Actions vs Local Testing

| Feature | GitHub Actions | Local Act |
|---------|---------------|-----------|
| **Environment** | Ubuntu latest | Docker container |
| **Speed** | Fast (GitHub runners) | Slower (Docker overhead) |
| **Cost** | Free for public repos | Free (local resources) |
| **Debugging** | Limited logs | Full access to container |
| **Iteration** | Push → Wait → Check | Immediate feedback |

### Troubleshooting

**Common Issues:**
- **Docker not running**: Start Docker Desktop before running `act`
- **M1 compatibility**: Use `--container-architecture linux/amd64`
- **Permission errors**: Ensure Docker has proper permissions
- **Network issues**: Check Docker network configuration

**Workflow Debugging:**
```bash
# See what act would run
act -n

# Dry run without executing
act --dryrun

# Debug specific step
act push --verbose
```

**Skip GitHub Actions:**
```bash
# Skip CI/CD with commit message (RECOMMENDED)
git commit -m "feat: add new feature [skip ci]"
git push

# Skip specific workflow
git commit -m "feat: add new feature [skip rust-ci]"
git push

# Skip local pre-push hooks (does NOT skip GitHub Actions)
git push --no-verify
```

## Credits

This project was created to test rust-analyzer and Clippy integration as part of learning Rust development best practices.

**Inspiration**: This demo was inspired by [Let's Get Rusty's Clippy tutorial](https://www.youtube.com/watch?v=BU1LYFkpJuk). 