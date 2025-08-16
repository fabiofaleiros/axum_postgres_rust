-- Migration: Add status and timestamp columns to tasks table
-- Phase 1: Basic Status Management

-- Add status column with default value and constraints
ALTER TABLE tasks ADD COLUMN status VARCHAR(20) NOT NULL DEFAULT 'Pending';
ALTER TABLE tasks ADD CONSTRAINT check_status CHECK (status IN ('Pending', 'InProgress', 'PendingReview', 'Completed', 'Cancelled'));

-- Add timestamp columns
ALTER TABLE tasks ADD COLUMN created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW();
ALTER TABLE tasks ADD COLUMN updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW();

-- Update existing records to have proper timestamps
UPDATE tasks SET created_at = NOW(), updated_at = NOW() WHERE created_at IS NULL;

-- Make timestamp columns NOT NULL
ALTER TABLE tasks ALTER COLUMN created_at SET NOT NULL;
ALTER TABLE tasks ALTER COLUMN updated_at SET NOT NULL;

-- Create index on status for efficient filtering
CREATE INDEX idx_tasks_status ON tasks(status);

-- Create index on created_at for sorting
CREATE INDEX idx_tasks_created_at ON tasks(created_at);