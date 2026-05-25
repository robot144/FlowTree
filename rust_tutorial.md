# Rust tutorial

## Some very useful links
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)

The examples below are loosely based/inspired by the official Rust documentation, but adapted to be run with **pixi**. You can copy and paste the code snippets into your Rust files and run them using the provided commands.

## A first Rust program hello world

```rust
fn main() {
    println!("Hello, world!");
}
```

## Explanation

- `fn main()` — Entry point of every Rust program
- `println!` — Macro that prints text to the console
- `!` — Indicates a macro (not a regular function)

## Compile & Run

Using **Cargo** (Rust's package manager and build system) with **pixi**:

```bash
# Initialize a new project
pixi run cargo init --name hello

# Copy your source file to src/main.rs
cp hello.rs src/main.rs

# Build and run
pixi run cargo run
```

Or build and run separately:

```bash
pixi run cargo build
pixi run ./target/debug/hello
```

## Key Features Shown

1. **Zero-cost abstractions** — `println!` is efficient despite being a macro
2. **Explicit entry point** — `main()` is required
3. **Semicolons** — Statements end with `;`

## Number Guessing Game

A simple interactive game where the computer generates a random number between 1 and 100, and the user tries to guess it.

### New Concepts (vs Hello World)

| Concept | Description | Example |
|---------|-------------|---------|
| **External crates** | Add dependencies in `Cargo.toml` | `rand = "0.10.1"` |
| **Random numbers** | Generate random values | `rand::random::<u32>()` |
| **User input** | Read from stdin | `io::stdin().read_line(&mut buf)` |
| **Error handling** | `Result` type with `match` | `match str.parse() { Ok(n) => ..., Err(_) => ... }` |
| **Loops** | Infinite loop with `loop` | `loop { ... break; }` |
| **Control flow** | Conditional branches | `if guess < secret { ... } else if ...` |
| **Type conversion** | Parse strings to numbers | `guess.trim().parse::<u32>()` |
| **Mutability** | Declare mutable variables | `let mut guess = String::new()` |

### To Run

```bash
pixi run cargo run --bin guessing_game
```
