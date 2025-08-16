use crate::domain::entities::Task;

pub struct TaskDomainService;

impl TaskDomainService {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_task_name(&self, name: &str) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("Task name cannot be empty".to_string());
        }
        if name.len() > 255 {
            return Err("Task name cannot exceed 255 characters".to_string());
        }
        Ok(())
    }

    pub fn validate_priority(&self, priority: Option<i32>) -> Result<(), String> {
        if let Some(priority) = priority {
            if priority < 1 || priority > 10 {
                return Err("Priority must be between 1 and 10".to_string());
            }
        }
        Ok(())
    }

    pub fn can_update_task(&self, _task: &Task, new_name: Option<&str>, new_priority: Option<i32>) -> Result<(), String> {
        if let Some(name) = new_name {
            self.validate_task_name(name)?;
        }
        self.validate_priority(new_priority)?;
        Ok(())
    }
}

