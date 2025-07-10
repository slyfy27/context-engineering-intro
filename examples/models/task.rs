use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, Type};
use uuid::Uuid;

/// Task priority levels for categorization
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "task_priority", rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::Medium
    }
}

/// Task status for tracking completion
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Pending
    }
}

/// Main Task model representing a task in the system
/// 
/// This model demonstrates proper MySQL integration patterns:
/// - UUID primary keys for distributed systems
/// - Proper timestamp handling with chrono
/// - Enum fields with MySQL type mapping
/// - JSON serialization for API responses
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    /// Create a new task with default values
    pub fn new(title: String, user_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            priority: TaskPriority::default(),
            status: TaskStatus::default(),
            user_id,
            created_at: now,
            updated_at: now,
            due_date: None,
            completed_at: None,
        }
    }

    /// Mark task as completed
    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Update task status
    pub fn update_status(&mut self, status: TaskStatus) {
        self.status = status;
        self.updated_at = Utc::now();
        
        if matches!(status, TaskStatus::Completed) {
            self.completed_at = Some(Utc::now());
        }
    }

    /// Check if task is overdue
    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            if !matches!(self.status, TaskStatus::Completed) {
                return Utc::now() > due_date;
            }
        }
        false
    }

    /// Get task age in days
    pub fn age_in_days(&self) -> i64 {
        let now = Utc::now();
        now.signed_duration_since(self.created_at).num_days()
    }
}

/// DTO for creating new tasks
#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<TaskPriority>,
    pub due_date: Option<DateTime<Utc>>,
}

impl CreateTaskRequest {
    /// Convert to Task model
    pub fn into_task(self, user_id: Uuid) -> Task {
        let mut task = Task::new(self.title, user_id);
        task.description = self.description;
        task.priority = self.priority.unwrap_or_default();
        task.due_date = self.due_date;
        task
    }

    /// Validate the request
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        
        if self.title.len() > 255 {
            return Err("Title must be 255 characters or less".to_string());
        }
        
        if let Some(description) = &self.description {
            if description.len() > 2000 {
                return Err("Description must be 2000 characters or less".to_string());
            }
        }
        
        if let Some(due_date) = self.due_date {
            if due_date <= Utc::now() {
                return Err("Due date must be in the future".to_string());
            }
        }
        
        Ok(())
    }
}

/// DTO for updating existing tasks
#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<TaskPriority>,
    pub status: Option<TaskStatus>,
    pub due_date: Option<DateTime<Utc>>,
}

impl UpdateTaskRequest {
    /// Apply updates to existing task
    pub fn apply_to_task(&self, task: &mut Task) {
        if let Some(title) = &self.title {
            task.title = title.clone();
        }
        
        if let Some(description) = &self.description {
            task.description = Some(description.clone());
        }
        
        if let Some(priority) = &self.priority {
            task.priority = priority.clone();
        }
        
        if let Some(status) = &self.status {
            task.update_status(status.clone());
        }
        
        if let Some(due_date) = self.due_date {
            task.due_date = Some(due_date);
        }
        
        task.updated_at = Utc::now();
    }

    /// Validate the update request
    pub fn validate(&self) -> Result<(), String> {
        if let Some(title) = &self.title {
            if title.trim().is_empty() {
                return Err("Title cannot be empty".to_string());
            }
            if title.len() > 255 {
                return Err("Title must be 255 characters or less".to_string());
            }
        }
        
        if let Some(description) = &self.description {
            if description.len() > 2000 {
                return Err("Description must be 2000 characters or less".to_string());
            }
        }
        
        if let Some(due_date) = self.due_date {
            if due_date <= Utc::now() {
                return Err("Due date must be in the future".to_string());
            }
        }
        
        Ok(())
    }
}

/// Query parameters for filtering tasks
#[derive(Debug, Deserialize)]
pub struct TaskQuery {
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub user_id: Option<Uuid>,
    pub overdue: Option<bool>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl TaskQuery {
    /// Build SQL WHERE clause based on filters
    pub fn build_where_clause(&self) -> (String, Vec<String>) {
        let mut conditions = Vec::new();
        let mut params = Vec::new();
        
        if let Some(status) = &self.status {
            conditions.push("status = ?".to_string());
            params.push(format!("{:?}", status).to_lowercase());
        }
        
        if let Some(priority) = &self.priority {
            conditions.push("priority = ?".to_string());
            params.push(format!("{:?}", priority).to_lowercase());
        }
        
        if let Some(user_id) = &self.user_id {
            conditions.push("user_id = ?".to_string());
            params.push(user_id.to_string());
        }
        
        if let Some(true) = self.overdue {
            conditions.push("due_date < NOW() AND status != 'completed'".to_string());
        }
        
        let where_clause = if conditions.is_empty() {
            "1=1".to_string()
        } else {
            conditions.join(" AND ")
        };
        
        (where_clause, params)
    }
    
    /// Get pagination offset
    pub fn get_offset(&self) -> u32 {
        let page = self.page.unwrap_or(1);
        let limit = self.get_limit();
        (page.saturating_sub(1)) * limit
    }
    
    /// Get pagination limit (capped at 100)
    pub fn get_limit(&self) -> u32 {
        self.limit.unwrap_or(20).min(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let user_id = Uuid::new_v4();
        let task = Task::new("Test Task".to_string(), user_id);
        
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.user_id, user_id);
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.priority, TaskPriority::Medium);
        assert!(task.completed_at.is_none());
    }

    #[test]
    fn test_task_completion() {
        let user_id = Uuid::new_v4();
        let mut task = Task::new("Test Task".to_string(), user_id);
        
        task.complete();
        
        assert_eq!(task.status, TaskStatus::Completed);
        assert!(task.completed_at.is_some());
    }

    #[test]
    fn test_overdue_task() {
        let user_id = Uuid::new_v4();
        let mut task = Task::new("Test Task".to_string(), user_id);
        task.due_date = Some(Utc::now() - chrono::Duration::days(1));
        
        assert!(task.is_overdue());
    }

    #[test]
    fn test_create_request_validation() {
        let request = CreateTaskRequest {
            title: "".to_string(),
            description: None,
            priority: None,
            due_date: None,
        };
        
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_future_due_date_validation() {
        let request = CreateTaskRequest {
            title: "Valid Title".to_string(),
            description: None,
            priority: None,
            due_date: Some(Utc::now() - chrono::Duration::days(1)),
        };
        
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_query_where_clause() {
        let query = TaskQuery {
            status: Some(TaskStatus::Pending),
            priority: Some(TaskPriority::High),
            user_id: None,
            overdue: None,
            page: None,
            limit: None,
        };
        
        let (where_clause, params) = query.build_where_clause();
        assert!(where_clause.contains("status = ?"));
        assert!(where_clause.contains("priority = ?"));
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_pagination() {
        let query = TaskQuery {
            status: None,
            priority: None,
            user_id: None,
            overdue: None,
            page: Some(3),
            limit: Some(10),
        };
        
        assert_eq!(query.get_offset(), 20);
        assert_eq!(query.get_limit(), 10);
    }
}