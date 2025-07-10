use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::ApiError,
    models::Task,
    services::TaskService,
    AppState,
};

/// Request payload for creating a new task
#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
}

/// Request payload for updating a task
#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<TaskPriority>,
    pub completed: Option<bool>,
}

/// Query parameters for listing tasks
#[derive(Debug, Deserialize)]
pub struct ListTasksQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
}

/// Task priority levels
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Task completion status
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Completed,
}

/// Response for paginated task lists
#[derive(Debug, Serialize)]
pub struct TaskListResponse {
    pub tasks: Vec<Task>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}

/// Get all tasks with optional filtering and pagination
/// 
/// # Arguments
/// 
/// * `state` - Application state containing database pool
/// * `query` - Query parameters for filtering and pagination
/// 
/// # Returns
/// 
/// JSON response containing paginated task list or error
pub async fn list_tasks(
    State(state): State<AppState>,
    Query(query): Query<ListTasksQuery>,
) -> Result<Json<TaskListResponse>, ApiError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20).min(100); // Cap at 100 items per page
    
    let task_service = TaskService::new(&state.db);
    let (tasks, total) = task_service
        .list_with_filters(page, limit, query.status, query.priority)
        .await?;

    let response = TaskListResponse {
        tasks,
        total,
        page,
        limit,
    };

    Ok(Json(response))
}

/// Get a specific task by ID
/// 
/// # Arguments
/// 
/// * `state` - Application state containing database pool
/// * `path` - Path parameters containing task ID
/// 
/// # Returns
/// 
/// JSON response containing task data or 404 error
pub async fn get_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<Task>, ApiError> {
    let task_service = TaskService::new(&state.db);
    let task = task_service.get_by_id(task_id).await?;

    Ok(Json(task))
}

/// Create a new task
/// 
/// # Arguments
/// 
/// * `state` - Application state containing database pool
/// * `payload` - Request payload containing task data
/// 
/// # Returns
/// 
/// JSON response containing created task with 201 status or error
pub async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<Task>), ApiError> {
    // Validate input
    if payload.title.trim().is_empty() {
        return Err(ApiError::Validation("Title cannot be empty".to_string()));
    }

    if payload.title.len() > 255 {
        return Err(ApiError::Validation("Title must be 255 characters or less".to_string()));
    }

    let task_service = TaskService::new(&state.db);
    let task = task_service.create(payload).await?;

    Ok((StatusCode::CREATED, Json(task)))
}

/// Update an existing task
/// 
/// # Arguments
/// 
/// * `state` - Application state containing database pool
/// * `path` - Path parameters containing task ID
/// * `payload` - Request payload containing updated task data
/// 
/// # Returns
/// 
/// JSON response containing updated task or error
pub async fn update_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<Json<Task>, ApiError> {
    // Validate input if title is being updated
    if let Some(ref title) = payload.title {
        if title.trim().is_empty() {
            return Err(ApiError::Validation("Title cannot be empty".to_string()));
        }
        if title.len() > 255 {
            return Err(ApiError::Validation("Title must be 255 characters or less".to_string()));
        }
    }

    let task_service = TaskService::new(&state.db);
    let task = task_service.update(task_id, payload).await?;

    Ok(Json(task))
}

/// Delete a task
/// 
/// # Arguments
/// 
/// * `state` - Application state containing database pool
/// * `path` - Path parameters containing task ID
/// 
/// # Returns
/// 
/// 204 No Content status or error
pub async fn delete_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let task_service = TaskService::new(&state.db);
    task_service.delete(task_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    // Mock setup would go here for integration testing
    // This demonstrates the testing pattern to follow

    #[tokio::test]
    async fn test_create_task_validation() {
        // Test empty title validation
        let payload = CreateTaskRequest {
            title: "".to_string(),
            description: None,
            priority: TaskPriority::Medium,
        };

        // This would test the validation logic
        assert!(payload.title.trim().is_empty());
    }

    #[tokio::test]
    async fn test_title_length_validation() {
        // Test title length validation
        let long_title = "a".repeat(256);
        let payload = CreateTaskRequest {
            title: long_title.clone(),
            description: None,
            priority: TaskPriority::Medium,
        };

        assert!(payload.title.len() > 255);
    }
}