use crate::domain::value_objects::{TaskId, TaskStatus};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub priority: Option<i32>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(id: TaskId, name: String, priority: Option<i32>) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("Task name cannot be empty".to_string());
        }
        
        if let Some(priority) = priority {
            if priority < 1 || priority > 10 {
                return Err("Priority must be between 1 and 10".to_string());
            }
        }

        let now = Utc::now();
        Ok(Task {
            id,
            name: name.trim().to_string(),
            priority,
            status: TaskStatus::default(),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn new_with_status(id: TaskId, name: String, priority: Option<i32>, status: TaskStatus, created_at: DateTime<Utc>, updated_at: DateTime<Utc>) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("Task name cannot be empty".to_string());
        }
        
        if let Some(priority) = priority {
            if priority < 1 || priority > 10 {
                return Err("Priority must be between 1 and 10".to_string());
            }
        }

        Ok(Task {
            id,
            name: name.trim().to_string(),
            priority,
            status,
            created_at,
            updated_at,
        })
    }

    pub fn update_name(&mut self, name: String) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("Task name cannot be empty".to_string());
        }
        self.name = name.trim().to_string();
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_priority(&mut self, priority: Option<i32>) -> Result<(), String> {
        if let Some(priority) = priority {
            if priority < 1 || priority > 10 {
                return Err("Priority must be between 1 and 10".to_string());
            }
        }
        self.priority = priority;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn status(&self) -> &TaskStatus {
        &self.status
    }

    pub fn is_high_priority(&self) -> bool {
        self.priority.map_or(false, |p| p <= 3)
    }

    pub fn start_progress(&mut self) -> Result<(), String> {
        if !self.status.can_transition_to(&TaskStatus::InProgress) {
            return Err("Cannot start progress on task in current status".to_string());
        }
        
        self.status = TaskStatus::InProgress;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn complete(&mut self) -> Result<(), String> {
        if self.is_high_priority() {
            if !self.status.can_transition_to(&TaskStatus::PendingReview) {
                return Err("Cannot complete high-priority task without review".to_string());
            }
            self.status = TaskStatus::PendingReview;
        } else {
            if !self.status.can_transition_to(&TaskStatus::Completed) {
                return Err("Cannot complete task in current status".to_string());
            }
            self.status = TaskStatus::Completed;
        }
        
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn approve_completion(&mut self) -> Result<(), String> {
        if self.status != TaskStatus::PendingReview {
            return Err("Can only approve tasks in PendingReview status".to_string());
        }
        
        self.status = TaskStatus::Completed;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), String> {
        if self.status == TaskStatus::Completed {
            return Err("Cannot cancel completed tasks".to_string());
        }
        
        self.status = TaskStatus::Cancelled;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn transition_to(&mut self, new_status: TaskStatus) -> Result<(), String> {
        if !self.status.can_transition_to(&new_status) {
            return Err(format!("Invalid transition from {:?} to {:?}", self.status, new_status));
        }
        
        match new_status {
            TaskStatus::InProgress => self.start_progress(),
            TaskStatus::Completed => {
                if self.status == TaskStatus::PendingReview {
                    self.approve_completion()
                } else {
                    self.complete()
                }
            },
            TaskStatus::PendingReview => {
                if self.is_high_priority() && self.status == TaskStatus::InProgress {
                    self.complete()
                } else {
                    Err("Only high-priority tasks can transition to PendingReview".to_string())
                }
            },
            TaskStatus::Cancelled => self.cancel(),
            _ => Err("Invalid status transition".to_string()),
        }
    }
}


