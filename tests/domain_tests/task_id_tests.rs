use axum_postgres_rust::domain::TaskId;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_id_new() {
        let id = TaskId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_task_id_value() {
        let id = TaskId::new(123);
        assert_eq!(id.value(), 123);
    }

    #[test]
    fn test_task_id_from_i32() {
        let id: TaskId = 99.into();
        assert_eq!(id.value(), 99);
    }

    #[test]
    fn test_i32_from_task_id() {
        let id = TaskId::new(456);
        let value: i32 = id.into();
        assert_eq!(value, 456);
    }

    #[test]
    fn test_task_id_equality() {
        let id1 = TaskId::new(100);
        let id2 = TaskId::new(100);
        let id3 = TaskId::new(200);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
        assert_ne!(id2, id3);
    }

    #[test]
    fn test_task_id_clone() {
        let id1 = TaskId::new(777);
        let id2 = id1.clone();
        
        assert_eq!(id1, id2);
        assert_eq!(id1.value(), id2.value());
    }

    #[test]
    fn test_task_id_copy() {
        let id1 = TaskId::new(888);
        let id2 = id1; // Copy, not move
        
        // Both should still be usable
        assert_eq!(id1.value(), 888);
        assert_eq!(id2.value(), 888);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_task_id_debug() {
        let id = TaskId::new(321);
        let debug_output = format!("{:?}", id);
        
        assert!(debug_output.contains("TaskId"));
        assert!(debug_output.contains("321"));
    }

    #[test]
    fn test_task_id_hash() {
        let id1 = TaskId::new(500);
        let id2 = TaskId::new(500);
        let id3 = TaskId::new(600);

        let mut map = HashMap::new();
        map.insert(id1, "first");
        map.insert(id3, "third");

        // Same value should return the same entry
        assert_eq!(map.get(&id1), Some(&"first"));
        assert_eq!(map.get(&id2), Some(&"first")); // Same as id1
        assert_eq!(map.get(&id3), Some(&"third"));
    }

    #[test]
    fn test_task_id_negative_values() {
        let id = TaskId::new(-1);
        assert_eq!(id.value(), -1);

        let negative_id: TaskId = (-100).into();
        assert_eq!(negative_id.value(), -100);
    }

    #[test]
    fn test_task_id_zero() {
        let id = TaskId::new(0);
        assert_eq!(id.value(), 0);
    }

    #[test]
    fn test_task_id_large_values() {
        let id = TaskId::new(i32::MAX);
        assert_eq!(id.value(), i32::MAX);

        let id_min = TaskId::new(i32::MIN);
        assert_eq!(id_min.value(), i32::MIN);
    }

    #[test]
    fn test_task_id_conversions_roundtrip() {
        let original_value = 12345;
        let id = TaskId::new(original_value);
        let converted_back: i32 = id.into();
        
        assert_eq!(original_value, converted_back);
    }
}