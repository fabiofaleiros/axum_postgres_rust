use crate::domain::{TaskStatus, UserRole};

pub struct TaskStatusService;

impl TaskStatusService {
    pub fn new() -> Self {
        Self
    }

    pub fn can_transition(
        &self,
        from: &TaskStatus,
        to: &TaskStatus,
        is_high_priority: bool,
        user_role: &UserRole,
    ) -> Result<(), String> {
        // First check if the basic transition is allowed
        if !from.can_transition_to(to) {
            return Err(format!("Invalid transition from {:?} to {:?}", from, to));
        }

        // Apply business rules based on priority and user role
        match (from, to) {
            // High priority tasks must go through review
            (TaskStatus::InProgress, TaskStatus::Completed) if is_high_priority => {
                Err("High-priority tasks must go through review before completion".to_string())
            }
            
            // Only managers can approve completion from review
            (TaskStatus::PendingReview, TaskStatus::Completed) if !user_role.can_approve() => {
                Err("Only managers can approve task completion".to_string())
            }
            
            // All other valid transitions are allowed
            _ => Ok(()),
        }
    }

    pub fn get_valid_transitions(
        &self,
        current: &TaskStatus,
        is_high_priority: bool,
        user_role: &UserRole,
    ) -> Vec<TaskStatus> {
        let mut valid_transitions = Vec::new();

        // Check all possible statuses
        let all_statuses = [
            TaskStatus::Pending,
            TaskStatus::InProgress,
            TaskStatus::PendingReview,
            TaskStatus::Completed,
            TaskStatus::Cancelled,
        ];

        for status in &all_statuses {
            if self.can_transition(current, status, is_high_priority, user_role).is_ok() {
                valid_transitions.push(status.clone());
            }
        }

        valid_transitions
    }

    pub fn validate_status_change(
        &self,
        from: &TaskStatus,
        to: &TaskStatus,
        is_high_priority: bool,
        user_role: &UserRole,
    ) -> Result<String, String> {
        self.can_transition(from, to, is_high_priority, user_role)?;

        let message = match (from, to) {
            (TaskStatus::Pending, TaskStatus::InProgress) => "Task started successfully",
            (TaskStatus::InProgress, TaskStatus::Completed) => "Task completed successfully",
            (TaskStatus::InProgress, TaskStatus::PendingReview) => "Task sent for review",
            (TaskStatus::PendingReview, TaskStatus::Completed) => "Task approved and completed",
            (_, TaskStatus::Cancelled) => "Task cancelled",
            _ => "Task status updated",
        };

        Ok(message.to_string())
    }

    pub fn requires_comment(&self, from: &TaskStatus, to: &TaskStatus) -> bool {
        match (from, to) {
            // Require comments for approval
            (TaskStatus::PendingReview, TaskStatus::Completed) => true,
            // Require comments for cancellation
            (_, TaskStatus::Cancelled) => true,
            _ => false,
        }
    }

    pub fn get_next_assignee_role(&self, from: &TaskStatus, to: &TaskStatus) -> Option<UserRole> {
        match (from, to) {
            // When task goes to review, it should be assigned to a manager
            (TaskStatus::InProgress, TaskStatus::PendingReview) => Some(UserRole::Manager),
            _ => None,
        }
    }
}

impl Default for TaskStatusService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_transitions_for_user() {
        let service = TaskStatusService::new();
        let user_role = UserRole::User;

        // User can start pending task
        assert!(service.can_transition(
            &TaskStatus::Pending,
            &TaskStatus::InProgress,
            false,
            &user_role
        ).is_ok());

        // User can complete low-priority task
        assert!(service.can_transition(
            &TaskStatus::InProgress,
            &TaskStatus::Completed,
            false,
            &user_role
        ).is_ok());

        // User cannot complete high-priority task directly
        assert!(service.can_transition(
            &TaskStatus::InProgress,
            &TaskStatus::Completed,
            true,
            &user_role
        ).is_err());
    }

    #[test]
    fn test_high_priority_workflow() {
        let service = TaskStatusService::new();
        let user_role = UserRole::User;
        let manager_role = UserRole::Manager;

        // High-priority task must go to review
        assert!(service.can_transition(
            &TaskStatus::InProgress,
            &TaskStatus::PendingReview,
            true,
            &user_role
        ).is_ok());

        // User cannot approve from review
        assert!(service.can_transition(
            &TaskStatus::PendingReview,
            &TaskStatus::Completed,
            true,
            &user_role
        ).is_err());

        // Manager can approve from review
        assert!(service.can_transition(
            &TaskStatus::PendingReview,
            &TaskStatus::Completed,
            true,
            &manager_role
        ).is_ok());
    }

    #[test]
    fn test_get_valid_transitions() {
        let service = TaskStatusService::new();
        let user_role = UserRole::User;
        let manager_role = UserRole::Manager;

        // User with high-priority in-progress task
        let transitions = service.get_valid_transitions(
            &TaskStatus::InProgress,
            true,
            &user_role
        );
        assert!(transitions.contains(&TaskStatus::PendingReview));
        assert!(transitions.contains(&TaskStatus::Cancelled));
        assert!(!transitions.contains(&TaskStatus::Completed));

        // Manager with task in review
        let transitions = service.get_valid_transitions(
            &TaskStatus::PendingReview,
            true,
            &manager_role
        );
        assert!(transitions.contains(&TaskStatus::Completed));
        assert!(transitions.contains(&TaskStatus::Cancelled));
    }

    #[test]
    fn test_requires_comment() {
        let service = TaskStatusService::new();

        assert!(service.requires_comment(&TaskStatus::PendingReview, &TaskStatus::Completed));
        assert!(service.requires_comment(&TaskStatus::InProgress, &TaskStatus::Cancelled));
        assert!(!service.requires_comment(&TaskStatus::Pending, &TaskStatus::InProgress));
    }

    #[test]
    fn test_get_next_assignee_role() {
        let service = TaskStatusService::new();

        assert_eq!(
            service.get_next_assignee_role(&TaskStatus::InProgress, &TaskStatus::PendingReview),
            Some(UserRole::Manager)
        );

        assert_eq!(
            service.get_next_assignee_role(&TaskStatus::Pending, &TaskStatus::InProgress),
            None
        );
    }

    #[test]
    fn test_validate_status_change() {
        let service = TaskStatusService::new();
        let user_role = UserRole::User;

        let result = service.validate_status_change(
            &TaskStatus::Pending,
            &TaskStatus::InProgress,
            false,
            &user_role
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Task started successfully");
    }
}