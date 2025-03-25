# DQR Codebase Guidelines

## Commands
- **Build**: `cargo build`
- **Run**: `cargo run`
- **Test**: `cargo test`
- **Test specific**: `cargo test <test_name>`
- **Lint**: `cargo clippy`
- **Format**: `cargo fmt`

## Code Style Guidelines
- **Formatting**: Follow Rust's standard formatting with `cargo fmt`
- **Imports**: Group standard library imports first, then external crates, then local modules
- **Types**: Use strong typing; avoid `impl Trait` except in function returns when appropriate
- **Naming**: Use snake_case for variables/functions, CamelCase for types/traits, SCREAMING_CASE for constants
- **Error Handling**: Use Result type for functions that can fail; propagate errors with `?` operator
- **Comments**: Document public functions with /// comments including examples when useful
- **File Structure**: Keep modules in separate files; use lib.rs for shared functionality

## Rust Best Practices
- Prefer immutable variables (`let` over `let mut`)
- Favor composition over inheritance
- Use pattern matching over conditional chains
- Handle all Result and Option values explicitly