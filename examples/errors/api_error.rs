use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Main API error type that encompasses all possible errors
/// 
/// This enum demonstrates proper error handling patterns:
/// - Using thiserror for automatic trait implementations
/// - Mapping different error sources to appropriate HTTP status codes
/// - Providing helpful error messages for debugging
#[derive(Debug, Error)]
pub enum ApiError {
    /// Database-related errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Validation errors for user input
    #[error("Validation error: {0}")]
    Validation(String),

    /// Authentication errors
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Authorization errors (user lacks permission)
    #[error("Authorization failed: {0}")]
    Authorization(String),

    /// Resource not found errors
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Conflicts (e.g., duplicate resources)
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Rate limiting errors
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    /// External service errors
    #[error("External service error: {0}")]
    ExternalService(String),

    /// Internal server errors
    #[error("Internal server error: {0}")]
    Internal(String),

    /// JWT token errors
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl ApiError {
    /// Get the appropriate HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Validation(_) => StatusCode::BAD_REQUEST,
            ApiError::Authentication(_) => StatusCode::UNAUTHORIZED,
            ApiError::Authorization(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::RateLimit(_) => StatusCode::TOO_MANY_REQUESTS,
            ApiError::ExternalService(_) => StatusCode::BAD_GATEWAY,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Jwt(_) => StatusCode::UNAUTHORIZED,
            ApiError::Serialization(_) => StatusCode::BAD_REQUEST,
        }
    }

    /// Get error code for client-side error handling
    pub fn error_code(&self) -> &'static str {
        match self {
            ApiError::Database(_) => "DATABASE_ERROR",
            ApiError::Validation(_) => "VALIDATION_ERROR",
            ApiError::Authentication(_) => "AUTHENTICATION_ERROR",
            ApiError::Authorization(_) => "AUTHORIZATION_ERROR",
            ApiError::NotFound(_) => "NOT_FOUND",
            ApiError::Conflict(_) => "CONFLICT",
            ApiError::RateLimit(_) => "RATE_LIMIT_EXCEEDED",
            ApiError::ExternalService(_) => "EXTERNAL_SERVICE_ERROR",
            ApiError::Internal(_) => "INTERNAL_ERROR",
            ApiError::Jwt(_) => "JWT_ERROR",
            ApiError::Serialization(_) => "SERIALIZATION_ERROR",
        }
    }

    /// Check if this error should be logged (internal errors vs user errors)
    pub fn should_log(&self) -> bool {
        match self {
            ApiError::Database(_) 
            | ApiError::ExternalService(_) 
            | ApiError::Internal(_) => true,
            _ => false,
        }
    }

    /// Get user-friendly message (hiding internal details)
    pub fn user_message(&self) -> String {
        match self {
            ApiError::Database(_) => "A database error occurred. Please try again later.".to_string(),
            ApiError::ExternalService(_) => "An external service is temporarily unavailable.".to_string(),
            ApiError::Internal(_) => "An internal error occurred. Please try again later.".to_string(),
            // For other errors, use the display message
            _ => self.to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        
        // Log internal errors
        if self.should_log() {
            tracing::error!("API Error: {:?}", self);
        }

        let body = Json(json!({
            "error": {
                "code": self.error_code(),
                "message": self.user_message(),
                "status": status_code.as_u16()
            }
        }));

        (status_code, body).into_response()
    }
}

/// Helper functions for common error scenarios
impl ApiError {
    /// Create a validation error with a custom message
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    /// Create a not found error for a specific resource
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound(format!("{} not found", resource.into()))
    }

    /// Create an authentication error
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::Authentication(message.into())
    }

    /// Create an authorization error
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::Authorization(message.into())
    }

    /// Create a conflict error
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict(message.into())
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }
}

/// Result type alias for API operations
pub type ApiResult<T> = Result<T, ApiError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_codes() {
        assert_eq!(ApiError::validation("test").status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(ApiError::not_found("user").status_code(), StatusCode::NOT_FOUND);
        assert_eq!(ApiError::unauthorized("invalid token").status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(ApiError::forbidden("access denied").status_code(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(ApiError::validation("test").error_code(), "VALIDATION_ERROR");
        assert_eq!(ApiError::not_found("user").error_code(), "NOT_FOUND");
        assert_eq!(ApiError::unauthorized("invalid").error_code(), "AUTHENTICATION_ERROR");
    }

    #[test]
    fn test_should_log() {
        assert!(ApiError::internal("test").should_log());
        assert!(!ApiError::validation("test").should_log());
        assert!(!ApiError::not_found("user").should_log());
    }

    #[test]
    fn test_user_messages() {
        // Internal errors should have generic messages
        let db_error = ApiError::Database(sqlx::Error::RowNotFound);
        assert_eq!(db_error.user_message(), "A database error occurred. Please try again later.");

        // User errors should show actual message
        let validation_error = ApiError::validation("Invalid email format");
        assert_eq!(validation_error.user_message(), "Validation error: Invalid email format");
    }
}