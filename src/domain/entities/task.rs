use crate::domain::value_objects::TaskId;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub priority: Option<i32>,
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

        Ok(Task {
            id,
            name: name.trim().to_string(),
            priority,
        })
    }

    pub fn update_name(&mut self, name: String) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("Task name cannot be empty".to_string());
        }
        self.name = name.trim().to_string();
        Ok(())
    }

    pub fn update_priority(&mut self, priority: Option<i32>) -> Result<(), String> {
        if let Some(priority) = priority {
            if priority < 1 || priority > 10 {
                return Err("Priority must be between 1 and 10".to_string());
            }
        }
        self.priority = priority;
        Ok(())
    }
}