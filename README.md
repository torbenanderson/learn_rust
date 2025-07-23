# Learn Rust

A collection of Rust learning projects demonstrating different concepts and best practices.

## Projects

### **hello_rust** - Basic Rust Project
- **Purpose**: First Rust project with basic "Hello, world!" program
- **Concepts**: Basic Rust syntax, `cargo new`, `cargo run`
- **Learning**: Getting started with Rust development

### **hello_cargo** - Cargo Project Management
- **Purpose**: Understanding Cargo project structure and build system
- **Concepts**: `Cargo.toml`, `Cargo.lock`, project organization
- **Learning**: How Cargo manages dependencies and builds

### **hello_dep_ran** - Dependencies and Advanced Concepts
- **Purpose**: Working with external crates and advanced Rust features
- **Concepts**: 
  - Adding dependencies (`rand` crate)
  - Constants for magic numbers
  - Documentation comments (`///`)
  - Error handling with `Result<T, E>`
  - Unit tests with `#[test]`
  - Development tools (CodeLLDB, Even Better TOML)
- **Learning**: Production-ready Rust development practices



## Learning Progression

1. **Start with `hello_rust`** - Basic Rust syntax and Cargo
2. **Move to `hello_cargo`** - Understanding project structure
3. **Complete with `hello_dep_ran`** - Advanced concepts and best practices
4. **Explore `hello_sum_calc`** - Clippy linting and rust-analyzer configuration

## Development Environment

All projects are configured with:
- **Cursor/VS Code** with Rust extensions
- **rust-analyzer** for language support
- **CodeLLDB** for debugging
- **Even Better TOML** for configuration files

## Running Projects

```bash
# Navigate to any project
cd hello_rust
cd hello_cargo  
cd hello_dep_ran

# Run the project
cargo run

# Run tests (where applicable)
cargo test

# Check for errors
cargo check
```

## Credits

https://doc.rust-lang.org/book/title-page.html