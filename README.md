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

## Hot Reload with `cargo-watch`

To enable a faster development cycle with automatic recompilation and re-running of your Rust project upon file changes, you can use `cargo-watch`. This tool provides "hot reload" functionality, giving you immediate feedback on your code changes.

### Installation

Install `cargo-watch` globally so you can use the `cargo watch` command:

```bash
cargo install cargo-watch
```

**Note:** This installs the tool globally, making `cargo watch` available as a command. Adding it as a dependency with `cargo add cargo-watch` only makes it available to your project, but doesn't install the binary tool.

### Usage

Once installed, you can use `cargo-watch` to monitor your source files and automatically re-run your application. Here's a common command:

```bash
cargo watch -q -c -w src/ -x 'run -q'
```

**Explanation of the flags:**
- `-q` (quiet): Reduces the output from `cargo-watch` itself, keeping your terminal cleaner
- `-c` (clear): Clears the terminal screen before each new run of your application, providing a fresh output
- `-w src/` (watch directory): Specifies that `cargo-watch` should monitor the `src/` directory for any file changes. This is where your Rust source code resides
- `-x 'run -q'` (execute command): This is the command that `cargo-watch` will execute whenever it detects changes
  - `run`: Tells Cargo to compile and run your project
  - `-q`: Makes the `cargo run` command also run in quiet mode, further reducing output

This setup allows you to save your Rust files, and `cargo-watch` will automatically recompile and re-run your application, providing a seamless hot-reloading experience.

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