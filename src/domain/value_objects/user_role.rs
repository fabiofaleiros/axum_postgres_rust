use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    User,
    Manager,
    Admin,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::User => "User",
            UserRole::Manager => "Manager", 
            UserRole::Admin => "Admin",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "User" => Ok(UserRole::User),
            "Manager" => Ok(UserRole::Manager),
            "Admin" => Ok(UserRole::Admin),
            _ => Err(format!("Invalid user role: {}", s)),
        }
    }

    pub fn can_approve(&self) -> bool {
        match self {
            UserRole::User => false,
            UserRole::Manager | UserRole::Admin => true,
        }
    }

    pub fn can_manage_users(&self) -> bool {
        match self {
            UserRole::User | UserRole::Manager => false,
            UserRole::Admin => true,
        }
    }

    pub fn has_elevated_permissions(&self) -> bool {
        match self {
            UserRole::User => false,
            UserRole::Manager | UserRole::Admin => true,
        }
    }
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_role_serialization() {
        assert_eq!(UserRole::User.as_str(), "User");
        assert_eq!(UserRole::Manager.as_str(), "Manager");
        assert_eq!(UserRole::Admin.as_str(), "Admin");
    }

    #[test]
    fn test_user_role_parsing() {
        assert_eq!(UserRole::from_str("User").unwrap(), UserRole::User);
        assert_eq!(UserRole::from_str("Manager").unwrap(), UserRole::Manager);
        assert_eq!(UserRole::from_str("Admin").unwrap(), UserRole::Admin);
    }

    #[test]
    fn test_invalid_user_role_parsing() {
        let result = UserRole::from_str("InvalidRole");
        assert!(result.is_err());
    }

    #[test]
    fn test_approval_permissions() {
        assert!(!UserRole::User.can_approve());
        assert!(UserRole::Manager.can_approve());
        assert!(UserRole::Admin.can_approve());
    }

    #[test]
    fn test_user_management_permissions() {
        assert!(!UserRole::User.can_manage_users());
        assert!(!UserRole::Manager.can_manage_users());
        assert!(UserRole::Admin.can_manage_users());
    }

    #[test]
    fn test_elevated_permissions() {
        assert!(!UserRole::User.has_elevated_permissions());
        assert!(UserRole::Manager.has_elevated_permissions());
        assert!(UserRole::Admin.has_elevated_permissions());
    }

    #[test]
    fn test_default_role() {
        let default_role = UserRole::default();
        assert_eq!(default_role, UserRole::User);
    }
}