# Task Management API - Bruno Collection

This Bruno collection contains comprehensive API tests for the Task Management service built with Rust, Axum, and PostgreSQL.

## Prerequisites

1. **Start the service** using one of these methods:
   ```bash
   # Local development (recommended)
   make run-local
   
   # Or with Docker
   make run-docker
   ```

2. **Install Bruno** (if not already installed):
   - Download from: https://www.usebruno.com/
   - Or install via package manager

## Collection Structure

### Basic Endpoints
- **Health Check** - Service health verification
- **API Info** - Service information and metadata

### Task Management (CRUD)
- **Get All Tasks** - Retrieve all tasks
- **Get Tasks by Priority** - Filter tasks by priority level
- **Get Task by ID** - Retrieve specific task
- **Create Task** - Create new task (regular priority)
- **Create High Priority Task** - Create high-priority task (requires review)
- **Update Task** - Update task name and priority
- **Delete Task** - Remove task

### Status Management
- **Update Task Status** - Change task to InProgress
- **Complete Task (Low Priority)** - Direct completion for low-priority tasks
- **Complete High Priority Task** - Requires review workflow
- **Approve Task Completion (Manager)** - Manager approval for high-priority tasks
- **Cancel Task** - Cancel task with reason
- **Get Valid Transitions** - Get available status transitions

### Analytics
- **Get Task History** - View task status change history
- **Get Task Analytics** - Individual task completion metrics
- **Get Completion Analytics (Date Range)** - Analytics for completed tasks in date range
- **Get Average Completion Times by Priority** - Performance metrics by priority

## Environment Variables

The collection uses these environment variables (defined in `environments/Local.bru`):

- `base_url`: http://127.0.0.1:7878 (service endpoint)
- `task_id`: 1 (default task ID for testing)

## Usage Tips

1. **Start with Health Check** to verify the service is running
2. **Create tasks first** before testing status updates
3. **Use different priority levels** (1-10) to test priority-based workflows
4. **High-priority tasks** (priority 1-3) require manager approval
5. **Status transitions** are validated - check valid transitions first

## Business Rules Tested

- **Priority Validation**: Only priorities 1-10 are allowed
- **Status Transitions**: Valid state machine transitions
- **Role-Based Access**: Managers can approve, developers can complete low-priority tasks
- **High-Priority Workflow**: Priority 1-3 tasks require review before completion
- **Audit Trail**: All status changes are logged with user and timestamp

## Running Tests

1. Open Bruno
2. Import this collection folder
3. Select "Local" environment
4. Run individual requests or entire folders
5. Check test assertions in the "Tests" tab

## Test Scenarios

### Basic Workflow
1. Create Task → Update to InProgress → Complete (for low priority)
2. Create High Priority Task → Update to InProgress → Complete (goes to PendingReview) → Manager Approval

### Error Testing
- Try invalid status transitions
- Test with invalid priorities
- Test role-based restrictions

All requests include comprehensive test assertions to verify correct API behavior.