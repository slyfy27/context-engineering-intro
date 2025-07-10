use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::time::Duration;
use tracing::{info, warn};

/// Database connection configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "mysql://root:password@localhost:3306/app_db".to_string(),
            max_connections: 10,
            min_connections: 1,
            connect_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
        }
    }
}

/// Create a MySQL connection pool with proper configuration
/// 
/// # Arguments
/// 
/// * `config` - Database configuration parameters
/// 
/// # Returns
/// 
/// A configured MySQL connection pool or error
/// 
/// # Example
/// 
/// ```rust
/// let config = DatabaseConfig::default();
/// let pool = create_pool(config).await?;
/// ```
pub async fn create_pool(config: DatabaseConfig) -> Result<Pool<MySql>, sqlx::Error> {
    info!("Creating MySQL connection pool...");
    
    let pool = MySqlPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.connect_timeout)
        .idle_timeout(config.idle_timeout)
        // Enable SQL logging in debug mode
        .before_acquire(|conn, meta| {
            Box::pin(async move {
                tracing::debug!(
                    "Acquiring connection from pool (pool_size: {}, checked_out: {})",
                    meta.size,
                    meta.checked_out
                );
                Ok(())
            })
        })
        .after_release(|_conn, meta| {
            Box::pin(async move {
                tracing::debug!(
                    "Released connection back to pool (pool_size: {}, checked_out: {})",
                    meta.size,
                    meta.checked_out
                );
                Ok(())
            })
        })
        .connect(&config.url)
        .await?;

    info!("MySQL connection pool created successfully");
    Ok(pool)
}

/// Test database connectivity
/// 
/// # Arguments
/// 
/// * `pool` - MySQL connection pool to test
/// 
/// # Returns
/// 
/// Success or error from ping operation
pub async fn test_connection(pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    info!("Testing database connectivity...");
    
    let start = std::time::Instant::now();
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await?;
    let duration = start.elapsed();
    
    info!("Database connectivity test passed in {:?}", duration);
    
    if duration > Duration::from_millis(100) {
        warn!("Database response time is slow: {:?}", duration);
    }
    
    Ok(())
}

/// Get database configuration from environment variables
/// 
/// Environment variables:
/// - DATABASE_URL: Full MySQL connection string
/// - DB_MAX_CONNECTIONS: Maximum pool connections (default: 10)
/// - DB_MIN_CONNECTIONS: Minimum pool connections (default: 1)
/// - DB_CONNECT_TIMEOUT: Connection timeout in seconds (default: 30)
/// - DB_IDLE_TIMEOUT: Idle timeout in seconds (default: 600)
pub fn get_database_config() -> DatabaseConfig {
    DatabaseConfig {
        url: std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:password@localhost:3306/app_db".to_string()),
        max_connections: std::env::var("DB_MAX_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10),
        min_connections: std::env::var("DB_MIN_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1),
        connect_timeout: Duration::from_secs(
            std::env::var("DB_CONNECT_TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30)
        ),
        idle_timeout: Duration::from_secs(
            std::env::var("DB_IDLE_TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(600)
        ),
    }
}

/// Health check for database connection
/// 
/// Returns detailed information about the pool status
pub async fn health_check(pool: &Pool<MySql>) -> serde_json::Value {
    match test_connection(pool).await {
        Ok(_) => serde_json::json!({
            "status": "healthy",
            "database": "mysql",
            "pool_size": pool.size(),
            "idle_connections": pool.num_idle(),
        }),
        Err(e) => serde_json::json!({
            "status": "unhealthy",
            "database": "mysql",
            "error": e.to_string(),
            "pool_size": pool.size(),
            "idle_connections": pool.num_idle(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DatabaseConfig::default();
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_connections, 1);
        assert!(config.url.contains("mysql://"));
    }

    #[test]
    fn test_config_from_env() {
        std::env::set_var("DATABASE_URL", "mysql://test:test@localhost:3306/test_db");
        std::env::set_var("DB_MAX_CONNECTIONS", "20");
        
        let config = get_database_config();
        assert!(config.url.contains("test_db"));
        assert_eq!(config.max_connections, 20);
        
        // Clean up
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("DB_MAX_CONNECTIONS");
    }

    #[tokio::test]
    async fn test_pool_creation_invalid_url() {
        let config = DatabaseConfig {
            url: "invalid://url".to_string(),
            ..Default::default()
        };
        
        let result = create_pool(config).await;
        assert!(result.is_err());
    }
}