# Rust Examples

This folder contains example patterns and best practices for Rust development that should be followed when implementing new features.

## Structure Overview

```
examples/
├── handlers/          # HTTP request handlers
│   ├── auth.rs
│   └── tasks.rs
├── models/            # Data models and DTOs
│   ├── user.rs
│   └── task.rs
├── services/          # Business logic layer
│   ├── auth_service.rs
│   └── task_service.rs
├── database/          # Database integration
│   ├── connection.rs
│   └── migrations.rs
├── middleware/        # HTTP middleware
│   ├── auth.rs
│   └── logging.rs
├── errors/            # Error types and handling
│   └── api_error.rs
└── utils/            # Helper functions
    ├── validation.rs
    └── crypto.rs
```

## Key Patterns to Follow

### 1. Error Handling
- Use `Result<T, E>` for all fallible operations
- Create custom error types with proper `Display` and `Error` implementations
- Use the `?` operator for error propagation
- Never use `unwrap()` or `expect()` in production code

### 2. Async Programming
- Use `async/await` for I/O operations
- Prefer `tokio::spawn` for concurrent tasks
- Use appropriate async data structures (`Mutex<T>`, `RwLock<T>`)
- Handle cancellation and timeouts properly

### 3. Database Integration
- Use SQLx for compile-time checked queries
- Implement proper connection pooling
- Handle database errors gracefully
- Use transactions for multi-step operations

### 4. HTTP API Design
- Use Axum extractors for request parsing
- Implement proper status codes and error responses
- Use middleware for cross-cutting concerns
- Validate input data with serde and custom validators

### 5. Security Practices
- Hash passwords with argon2 or bcrypt
- Use secure JWT implementations
- Validate all user inputs
- Implement proper rate limiting

## Best Practices Demonstrated

- **Memory Safety**: Examples show proper ownership and borrowing patterns
- **Error Handling**: Comprehensive error management with custom types
- **Testing**: Unit tests with `#[cfg(test)]` and integration tests
- **Documentation**: Proper doc comments with examples
- **Performance**: Efficient data structures and async patterns
- **Security**: Input validation and secure authentication flows

## Common Patterns

### Error Handling Pattern
```rust
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
}
```

### Async Handler Pattern
```rust
pub async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<TaskResponse>, ApiError> {
    // Implementation
}
```

### Service Layer Pattern
```rust
pub struct TaskService {
    db: Pool<Postgres>,
}

impl TaskService {
    pub async fn create(&self, task: CreateTask) -> Result<Task, ApiError> {
        // Implementation
    }
}
```