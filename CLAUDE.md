### 🔄 Project Awareness & Context
- **Always read `PLANNING.md`** at the start of a new conversation to understand the project's architecture, goals, style, and constraints.
- **Check `TASK.md`** before starting a new task. If the task isn't listed, add it with a brief description and today's date.
- **Use consistent naming conventions, file structure, and architecture patterns** as described in `PLANNING.md`.
- **Run `cargo check`** after making changes to ensure code compiles correctly.

### 🧱 Code Structure & Modularity
- **Never create a Rust file longer than 500 lines of code.** If a file approaches this limit, refactor by splitting it into separate modules or crates.
- **Organize code into clearly separated modules**, grouped by feature or responsibility.
  For Rust projects this looks like:
    - `src/main.rs` - Application entry point
    - `src/lib.rs` - Library root and public API
    - `src/models/` - Data structures and domain models
    - `src/services/` - Business logic and external integrations
    - `src/handlers/` - Request handlers and controllers
    - `src/utils/` - Helper functions and utilities
    - `src/config/` - Configuration management
    - `src/errors/` - Error types and handling
- **Use clear module hierarchy** with proper `mod.rs` files and `pub use` statements.
- **Use environment variables** through dotenv or clap for configuration management.

### 🧪 Testing & Reliability
- **Always create tests for new features** using Rust's built-in testing framework.
- **After updating any logic**, check whether existing tests need to be updated. If so, do it.
- **Tests should be in the same file as unit tests or in `/tests` for integration tests**.
  - Include at least:
    - Unit tests with `#[cfg(test)]` modules
    - Integration tests for public APIs
    - Property-based tests for complex logic using proptest
- **Use `cargo test` to run all tests** and ensure they pass before committing.
- **Use `cargo clippy` for linting** and fix all warnings.

### ✅ Task Completion
- **Mark completed tasks in `TASK.md`** immediately after finishing them.
- Add new sub-tasks or TODOs discovered during development to `TASK.md` under a "Discovered During Work" section.

### 📎 Style & Conventions
- **Use Rust** as the primary language following Rust 2021 edition conventions.
- **Follow rustfmt style** and use `cargo fmt` for consistent formatting.
- **Use idiomatic Rust patterns** including proper error handling with `Result<T, E>`.
- **Use `cargo clippy` for linting** and address all suggestions before committing.
- Write **documentation comments for public APIs** using Rust's documentation format:
  ```rust
  /// Brief summary of what this function does.
  ///
  /// # Arguments
  ///
  /// * `param1` - Description of parameter
  ///
  /// # Returns
  ///
  /// Description of return value
  ///
  /// # Examples
  ///
  /// ```
  /// let result = example_function("test");
  /// assert_eq!(result, "expected");
  /// ```
  pub fn example_function(param1: &str) -> String {
      param1.to_string()
  }
  ```

### 🦀 Rust-Specific Guidelines
- **Use ownership and borrowing correctly** - prefer borrowing over cloning when possible.
- **Handle errors explicitly** using `Result<T, E>` and `?` operator - never use `unwrap()` in production code.
- **Use appropriate data structures** - `Vec<T>` for dynamic arrays, `HashMap<K, V>` for key-value storage.
- **Prefer traits over concrete types** for function parameters to increase flexibility.
- **Use `serde` for serialization** and derive macros when appropriate.
- **Follow Rust naming conventions** - `snake_case` for functions/variables, `PascalCase` for types.
- **Use `async/await` properly** for concurrent operations with tokio runtime.

### 📚 Documentation & Explainability
- **Update `README.md`** when new features are added, dependencies change, or setup steps are modified.
- **Comment complex algorithms and unsafe code** to ensure everything is understandable to a mid-level Rust developer.
- When writing complex logic, **add inline comments** explaining the why, not just the what.
- **Use `cargo doc` to generate documentation** and ensure all public APIs are documented.

### 🧠 AI Behavior Rules
- **Never assume missing context. Ask questions if uncertain.**
- **Never hallucinate crates or APIs** – only use known, verified crates from crates.io.
- **Always confirm file paths and module names** exist before referencing them in code or tests.
- **Never delete or overwrite existing code** unless explicitly instructed to or if part of a task from `TASK.md`.
- **Check Rust edition and MSRV compatibility** for any crates or features used.