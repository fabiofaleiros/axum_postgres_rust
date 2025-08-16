-- Migration: Create status_history table for audit trail
-- Phase 2: Business Rules and Approval

-- Create status_history table
CREATE TABLE status_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id INTEGER NOT NULL REFERENCES tasks(task_id) ON DELETE CASCADE,
    from_status VARCHAR(20),
    to_status VARCHAR(20) NOT NULL,
    changed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    changed_by VARCHAR(50), -- For now, just a string identifier; future: reference to users table
    comment TEXT,
    user_role VARCHAR(10) DEFAULT 'User',
    
    -- Constraints
    CONSTRAINT check_from_status CHECK (from_status IN ('Pending', 'InProgress', 'PendingReview', 'Completed', 'Cancelled')),
    CONSTRAINT check_to_status CHECK (to_status IN ('Pending', 'InProgress', 'PendingReview', 'Completed', 'Cancelled')),
    CONSTRAINT check_user_role CHECK (user_role IN ('User', 'Manager', 'Admin'))
);

-- Create indexes for efficient querying
CREATE INDEX idx_status_history_task_id ON status_history(task_id);
CREATE INDEX idx_status_history_changed_at ON status_history(changed_at);
CREATE INDEX idx_status_history_to_status ON status_history(to_status);

-- Create a function to automatically track status changes
CREATE OR REPLACE FUNCTION track_task_status_change()
RETURNS TRIGGER AS $$
BEGIN
    -- Only track if status actually changed
    IF OLD.status IS DISTINCT FROM NEW.status THEN
        INSERT INTO status_history (
            task_id,
            from_status,
            to_status,
            changed_at,
            changed_by,
            user_role
        ) VALUES (
            NEW.task_id,
            OLD.status,
            NEW.status,
            NEW.updated_at,
            COALESCE(current_setting('task.changed_by', true), 'system'),
            COALESCE(current_setting('task.user_role', true), 'User')
        );
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically track status changes
CREATE TRIGGER trigger_task_status_history
    AFTER UPDATE ON tasks
    FOR EACH ROW
    EXECUTE FUNCTION track_task_status_change();

-- Create initial history entries for existing tasks
INSERT INTO status_history (task_id, from_status, to_status, changed_at, changed_by, user_role)
SELECT 
    task_id,
    NULL as from_status,
    status as to_status,
    created_at as changed_at,
    'system' as changed_by,
    'User' as user_role
FROM tasks;