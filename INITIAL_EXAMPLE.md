## FEATURE:

- Rust web API service using Axum framework for a task management system.
- RESTful endpoints for CRUD operations on tasks and user management.
- JWT-based authentication and authorization middleware.
- PostgreSQL database integration using SQLx with async operations.
- Comprehensive error handling with custom error types and proper HTTP status codes.
- Structured logging using tracing and configuration management with environment variables.
- CLI interface for database migrations and admin operations using clap.

## EXAMPLES:

In the `examples/` folder, there is a README for you to read to understand what the example is all about and also how to structure your own README when you create documentation for the above feature.

- `examples/handlers/` - use these as templates for creating clean REST API handlers
- `examples/models/` - data models with proper serialization and validation patterns
- `examples/services/` - business logic layer with proper error handling
- `examples/database/` - database integration patterns using SQLx and migrations
- `examples/middleware/` - authentication and logging middleware examples
- `examples/errors/` - custom error types and error handling patterns
- `examples/utils/` - helper functions and utilities for common operations

Don't copy any of these examples directly, it is for a different project entirely. But use this as inspiration and for best practices.

## DOCUMENTATION:

Axum documentation: https://docs.rs/axum/
SQLx documentation: https://docs.rs/sqlx/
Tokio documentation: https://docs.rs/tokio/
Serde documentation: https://docs.rs/serde/
Tracing documentation: https://docs.rs/tracing/
JWT documentation: https://docs.rs/jsonwebtoken/

## OTHER CONSIDERATIONS:

- Include a .env.example file for database URLs and JWT secrets, use dotenv for environment variables
- Include a comprehensive README with setup instructions including database setup and migration commands
- Add proper error handling for database connection failures and validation errors
- Implement request/response logging middleware for debugging and monitoring
- Ensure proper async/await usage throughout the application with tokio runtime
- Include database migrations using SQLx migrate functionality
- Add comprehensive unit and integration tests for all endpoints
- Follow Rust security best practices for password hashing and JWT handling
- Include Docker configuration for easy deployment and development setup
