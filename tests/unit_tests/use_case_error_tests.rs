use axum_postgres_rust::application::use_cases::task_use_cases::UseCaseError;
use axum_postgres_rust::domain::RepositoryError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usecase_error_display() {
        let validation_error = UseCaseError::ValidationError("Invalid input".to_string());
        assert_eq!(validation_error.to_string(), "Validation error: Invalid input");

        let not_found_error = UseCaseError::NotFound("Resource missing".to_string());
        assert_eq!(not_found_error.to_string(), "Not found: Resource missing");

        let repository_error = UseCaseError::RepositoryError("DB connection lost".to_string());
        assert_eq!(repository_error.to_string(), "Repository error: DB connection lost");
    }

    #[test]
    fn test_usecase_error_from_repository_error() {
        let repo_not_found = RepositoryError::NotFound("Not found".to_string());
        let use_case_error = UseCaseError::from(repo_not_found);
        match use_case_error {
            UseCaseError::NotFound(msg) => assert_eq!(msg, "Not found"),
            _ => panic!("Expected NotFound error"),
        }

        let repo_validation = RepositoryError::ValidationError("Invalid".to_string());
        let use_case_error = UseCaseError::from(repo_validation);
        match use_case_error {
            UseCaseError::ValidationError(msg) => assert_eq!(msg, "Invalid"),
            _ => panic!("Expected ValidationError"),
        }

        let repo_db = RepositoryError::DatabaseError("DB Error".to_string());
        let use_case_error = UseCaseError::from(repo_db);
        match use_case_error {
            UseCaseError::RepositoryError(msg) => assert_eq!(msg, "DB Error"),
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[test]
    fn test_usecase_error_clone() {
        let original = UseCaseError::ValidationError("Test".to_string());
        let cloned = original.clone();
        
        match (original, cloned) {
            (UseCaseError::ValidationError(msg1), UseCaseError::ValidationError(msg2)) => {
                assert_eq!(msg1, msg2);
            }
            _ => panic!("Expected ValidationError for both"),
        }
    }

    #[test]
    fn test_usecase_error_debug() {
        let error = UseCaseError::NotFound("Test not found".to_string());
        let debug_output = format!("{:?}", error);
        
        assert!(debug_output.contains("NotFound"));
        assert!(debug_output.contains("Test not found"));
    }

    #[test]
    fn test_all_usecase_error_variants() {
        let validation = UseCaseError::ValidationError("validation".to_string());
        let not_found = UseCaseError::NotFound("not found".to_string());
        let repository = UseCaseError::RepositoryError("repository".to_string());

        // Test that all variants can be created and matched
        match validation {
            UseCaseError::ValidationError(_) => {},
            _ => panic!("Expected ValidationError"),
        }

        match not_found {
            UseCaseError::NotFound(_) => {},
            _ => panic!("Expected NotFound"),
        }

        match repository {
            UseCaseError::RepositoryError(_) => {},
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[test]
    fn test_usecase_error_error_trait() {
        let error = UseCaseError::ValidationError("Test error".to_string());
        
        // Test that it implements the Error trait
        let error_trait: &dyn std::error::Error = &error;
        assert!(error_trait.to_string().contains("Validation error"));
    }
}