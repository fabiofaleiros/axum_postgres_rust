# Task Status Transitions with Business Rules

## Product Owner Recommendation

**Recommended by**: Product Owner Agent  
**Priority**: High Business Impact  
**Effort Estimation**: 7-10 days (3 phases)  
**Business Value**: Foundation for advanced task management

---

## Business Value Assessment

**Current State**: The API has basic CRUD operations for tasks with name and priority fields.

**Gap Analysis**: Tasks lack proper lifecycle management - there's no way to track task progress, implement approval workflows, or enforce business rules around task completion.

## Feature Overview

**Epic**: Task Lifecycle Management
```
As a team using the task management system
I want tasks to have proper status transitions with business rules
So that we can track work progress and implement approval workflows
```

## Why This Feature?

1. **High Business Impact**: Essential for any real task management system
2. **Perfect Architecture Fit**: Demonstrates domain modeling with state transitions
3. **Incremental Value**: Can be built in phases with immediate utility
4. **Foundation Feature**: Enables future features like notifications, reporting, and automation

---

## User Stories & Agent Responsibilities

### Phase 1: Basic Status Management
**Primary Agent**: **Domain Expert**  
**Supporting Agents**: Backend Developer, QA Engineer

```
As a user
I want to transition tasks through states (Pending → In Progress → Completed)
So that I can track my work progress

Acceptance Criteria:
- Tasks have status: Pending, InProgress, Completed, Cancelled
- Valid transitions: Pending → InProgress → Completed
- Valid transitions: Any status → Cancelled
- Invalid transitions return clear error messages
- Status changes update the updated_at timestamp
```

**Agent Rationale**: Domain Expert leads because status transitions are core business concepts that belong in the domain layer. They ensure business rules are properly modeled and validated while maintaining separation between business logic and technical implementation.

### Phase 2: Business Rules
**Primary Agent**: **Domain Expert**  
**Supporting Agents**: Software Architect, Backend Developer

```
As a manager
I want high-priority tasks to require approval before completion
So that critical work is properly reviewed

Acceptance Criteria:
- Priority 1-3 tasks require "PendingReview" before "Completed"
- Only managers can approve tasks (transition from PendingReview → Completed)
- Tasks in PendingReview cannot be modified except for approval
- Approval includes optional comments
```

**Agent Rationale**: Domain Expert continues as primary for approval workflows since these are complex business rules requiring domain modeling expertise. Software Architect provides cross-layer workflow architecture guidance.

### Phase 3: Enhanced Tracking
**Primary Agent**: **Database Engineer**  
**Supporting Agents**: Backend Developer, QA Engineer

```
As a user
I want to see task history and time tracking
So that I can understand how long work takes

Acceptance Criteria:
- Track time spent in each status
- Maintain status change history with timestamps
- Calculate total time from creation to completion
- API endpoints for task analytics
```

**Agent Rationale**: Database Engineer leads because historical tracking and analytics require sophisticated database design, complex queries for time calculations, and performance optimization for large datasets.

---

## Technical Implementation (Hexagonal Architecture)

### Domain Layer Changes

#### New Value Objects
```rust
// Task Status Enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    PendingReview,
    Completed,
    Cancelled,
}

// User Role for Business Rules
#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    User,
    Manager,
    Admin,
}

// Status History for Tracking
#[derive(Debug, Clone)]
pub struct StatusHistory {
    pub from_status: Option<TaskStatus>,
    pub to_status: TaskStatus,
    pub changed_at: DateTime<Utc>,
    pub changed_by: UserId,
    pub comment: Option<String>,
}
```

#### Enhanced Task Entity
```rust
impl Task {
    pub fn start_progress(&mut self) -> Result<(), String> {
        match self.status {
            TaskStatus::Pending => {
                self.status = TaskStatus::InProgress;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err("Can only start progress on pending tasks".to_string())
        }
    }
    
    pub fn complete(&mut self, user_role: UserRole) -> Result<(), String> {
        match (self.status, self.priority.is_high_priority()) {
            (TaskStatus::InProgress, false) => {
                self.status = TaskStatus::Completed;
                self.updated_at = Utc::now();
                Ok(())
            }
            (TaskStatus::InProgress, true) => {
                self.status = TaskStatus::PendingReview;
                self.updated_at = Utc::now();
                Ok(())
            }
            (TaskStatus::PendingReview, _) if user_role.can_approve() => {
                self.status = TaskStatus::Completed;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err("Invalid status transition".to_string())
        }
    }
    
    pub fn cancel(&mut self) -> Result<(), String> {
        if self.status == TaskStatus::Completed {
            return Err("Cannot cancel completed tasks".to_string());
        }
        self.status = TaskStatus::Cancelled;
        self.updated_at = Utc::now();
        Ok(())
    }
}
```

#### Domain Service for Status Transitions
```rust
pub struct TaskStatusService;

impl TaskStatusService {
    pub fn can_transition(
        &self, 
        from: TaskStatus, 
        to: TaskStatus, 
        priority: TaskPriority, 
        user_role: UserRole
    ) -> Result<(), String> {
        match (from, to) {
            // Basic transitions
            (TaskStatus::Pending, TaskStatus::InProgress) => Ok(()),
            (TaskStatus::InProgress, TaskStatus::Completed) if !priority.is_high_priority() => Ok(()),
            (TaskStatus::InProgress, TaskStatus::PendingReview) if priority.is_high_priority() => Ok(()),
            (TaskStatus::PendingReview, TaskStatus::Completed) if user_role.can_approve() => Ok(()),
            
            // Cancel from any state except completed
            (_, TaskStatus::Cancelled) if from != TaskStatus::Completed => Ok(()),
            
            _ => Err(format!("Invalid transition from {:?} to {:?}", from, to))
        }
    }
    
    pub fn get_valid_transitions(&self, current: TaskStatus, priority: TaskPriority, user_role: UserRole) -> Vec<TaskStatus> {
        // Return list of valid next states
        match current {
            TaskStatus::Pending => vec![TaskStatus::InProgress, TaskStatus::Cancelled],
            TaskStatus::InProgress if priority.is_high_priority() => vec![TaskStatus::PendingReview, TaskStatus::Cancelled],
            TaskStatus::InProgress => vec![TaskStatus::Completed, TaskStatus::Cancelled],
            TaskStatus::PendingReview if user_role.can_approve() => vec![TaskStatus::Completed, TaskStatus::Cancelled],
            TaskStatus::PendingReview => vec![TaskStatus::Cancelled],
            _ => vec![]
        }
    }
}
```

### Application Layer Changes

#### New DTOs
```rust
#[derive(Debug, Deserialize)]
pub struct UpdateTaskStatusDto {
    pub status: TaskStatus,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TaskWithTransitionsDto {
    pub task: TaskDto,
    pub valid_transitions: Vec<TaskStatus>,
}

#[derive(Debug, Serialize)]
pub struct TaskHistoryDto {
    pub task_id: String,
    pub history: Vec<StatusHistoryDto>,
    pub total_time_in_progress: Option<Duration>,
}
```

#### Enhanced Use Cases
```rust
impl TaskUseCases {
    pub async fn update_task_status(
        &self,
        task_id: TaskId,
        dto: UpdateTaskStatusDto,
        user_role: UserRole
    ) -> Result<TaskDto, ApplicationError> {
        let mut task = self.repository
            .find_by_id(&task_id)
            .await?
            .ok_or_else(|| ApplicationError::NotFound("Task not found".to_string()))?;
        
        // Validate transition using domain service
        self.status_service.can_transition(
            task.status(),
            dto.status,
            task.priority(),
            user_role
        )?;
        
        // Apply transition
        match dto.status {
            TaskStatus::InProgress => task.start_progress()?,
            TaskStatus::Completed => task.complete(user_role)?,
            TaskStatus::Cancelled => task.cancel()?,
            _ => return Err(ApplicationError::BadRequest("Invalid status transition".to_string()))
        }
        
        // Save and record history
        let updated_task = self.repository.save(task).await?;
        
        if let Some(comment) = dto.comment {
            self.record_status_change(&task_id, dto.status, comment).await?;
        }
        
        Ok(TaskDto::from(updated_task))
    }
    
    pub async fn get_task_with_transitions(
        &self,
        task_id: TaskId,
        user_role: UserRole
    ) -> Result<TaskWithTransitionsDto, ApplicationError> {
        let task = self.repository
            .find_by_id(&task_id)
            .await?
            .ok_or_else(|| ApplicationError::NotFound("Task not found".to_string()))?;
        
        let valid_transitions = self.status_service.get_valid_transitions(
            task.status(),
            task.priority(),
            user_role
        );
        
        Ok(TaskWithTransitionsDto {
            task: TaskDto::from(task),
            valid_transitions,
        })
    }
}
```

### Infrastructure Layer Changes

#### Database Schema Updates
```sql
-- Migration: Add status column to tasks table
ALTER TABLE tasks ADD COLUMN status VARCHAR(20) NOT NULL DEFAULT 'Pending';
ALTER TABLE tasks ADD CONSTRAINT check_status CHECK (status IN ('Pending', 'InProgress', 'PendingReview', 'Completed', 'Cancelled'));

-- Create status_history table
CREATE TABLE status_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    from_status VARCHAR(20),
    to_status VARCHAR(20) NOT NULL,
    changed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    changed_by UUID, -- Future: reference to users table
    comment TEXT,
    CONSTRAINT check_from_status CHECK (from_status IN ('Pending', 'InProgress', 'PendingReview', 'Completed', 'Cancelled')),
    CONSTRAINT check_to_status CHECK (to_status IN ('Pending', 'InProgress', 'PendingReview', 'Completed', 'Cancelled'))
);

CREATE INDEX idx_status_history_task_id ON status_history(task_id);
CREATE INDEX idx_status_history_changed_at ON status_history(changed_at);
```

#### New API Endpoints
```rust
// In TaskController
async fn update_task_status(
    State(use_cases): State<Arc<TaskUseCases>>,
    Path(id): Path<String>,
    Json(dto): Json<UpdateTaskStatusDto>,
) -> Result<impl IntoResponse, ApiError> {
    let task_id = TaskId::from_string(id)?;
    let user_role = UserRole::User; // TODO: Extract from JWT token
    
    let task = use_cases.update_task_status(task_id, dto, user_role).await?;
    Ok((StatusCode::OK, Json(task)))
}

async fn get_task_transitions(
    State(use_cases): State<Arc<TaskUseCases>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let task_id = TaskId::from_string(id)?;
    let user_role = UserRole::User; // TODO: Extract from JWT token
    
    let result = use_cases.get_task_with_transitions(task_id, user_role).await?;
    Ok((StatusCode::OK, Json(result)))
}
```

---

## Implementation Phases & Agent Coordination

### Phase 1: Basic Status Transitions (2-3 days)
**Agent Flow**: **Domain Expert** → **Backend Developer** → **QA Engineer**

**Agent-Specific Tasks**:

**Domain Expert**:
- [ ] Design TaskStatus enum with business validation rules
- [ ] Model Task entity status transition methods with business logic
- [ ] Define valid/invalid transition rules and error messages
- [ ] Create unit tests for domain status transition logic

**Backend Developer**:
- [ ] Create database migration for status column with constraints
- [ ] Implement PATCH /tasks/{id}/status API endpoint
- [ ] Update TaskDto to include status field
- [ ] Integrate domain status logic with web layer

**QA Engineer**:
- [ ] Write integration tests for status API endpoints
- [ ] Test invalid transition error handling
- [ ] Ensure >90% test coverage for status functionality
- [ ] Validate performance requirements

**Definition of Done**:
- Tasks can transition: Pending → InProgress → Completed
- Invalid transitions return proper error messages
- All tests pass with >90% coverage

### Phase 2: Business Rules and Approval (3-4 days)
**Agent Flow**: **Domain Expert** → **Software Architect** → **Backend Developer**

**Agent-Specific Tasks**:

**Domain Expert**:
- [ ] Model UserRole enum and permission logic
- [ ] Design TaskStatusService domain service with approval rules
- [ ] Implement high-priority task approval business logic
- [ ] Create unit tests for approval workflow domain logic

**Software Architect**:
- [ ] Design cross-layer approval workflow architecture
- [ ] Plan status_history table integration strategy
- [ ] Define error handling patterns for approval failures
- [ ] Review domain service integration with application layer

**Backend Developer**:
- [ ] Create status_history table and migration
- [ ] Implement approval API endpoints
- [ ] Add role-based authorization middleware
- [ ] Integrate approval workflow with existing API

**QA Engineer**:
- [ ] Test approval workflow end-to-end scenarios
- [ ] Validate role-based access control
- [ ] Test status history tracking accuracy

**Definition of Done**:
- High-priority tasks require approval before completion
- Status history is properly tracked
- Approval workflow is fully functional

### Phase 3: Analytics and History (2-3 days)  
**Agent Flow**: **Database Engineer** → **Backend Developer** → **QA Engineer**

**Agent-Specific Tasks**:

**Database Engineer**:
- [ ] Design optimized status_history table schema and indexes
- [ ] Create complex time calculation queries
- [ ] Implement performance optimization for historical data
- [ ] Design StatusHistory repository with efficient queries

**Backend Developer**:
- [ ] Implement GET /tasks/{id}/history endpoint
- [ ] Create GET /tasks/analytics/completion-times endpoint
- [ ] Add time tracking calculations to DTOs
- [ ] Integrate analytics queries with application layer

**QA Engineer**:
- [ ] Test analytics endpoint accuracy and performance
- [ ] Validate time calculation correctness
- [ ] Ensure <200ms response time requirements
- [ ] Test with large datasets for scalability

**Definition of Done**:
- Complete status history tracking
- Analytics endpoints provide meaningful insights
- Performance meets requirements (<200ms response time)

---

## Success Metrics

### Adoption Metrics
- **Status Transitions**: 80% of created tasks transition through at least 2 states
- **API Usage**: New status endpoints account for 30% of total API calls
- **Approval Workflow**: 100% of high-priority tasks go through approval process

### Technical Metrics
- **Error Rate**: <2% error rate on status transition requests
- **Response Time**: <200ms for 95% of status-related API calls
- **Test Coverage**: >90% coverage for all status-related code

### Business Metrics
- **Task Completion Rate**: Increase from baseline after status visibility
- **High-Priority Task Compliance**: 100% approval rate for priority 1-3 tasks
- **User Adoption**: 70% of users actively use status transitions within 30 days

---

## Technical Considerations

### Performance
- Index on status column for efficient filtering
- Pagination for status history endpoints
- Caching for valid transitions lookup

### Security
- Role-based authorization for status transitions
- Audit trail for all status changes
- Input validation and sanitization

### Scalability
- Status history table partitioning for large datasets
- Async processing for heavy analytics queries
- Connection pooling optimization

---

## Documentation Updates Post-Epic Completion

### Documentation Responsibility Matrix

**Primary Documentation Agent**: **Technical Lead**  
**Rationale**: Technical Lead has oversight of the entire feature implementation across all phases and can provide comprehensive documentation that bridges business requirements with technical implementation.

### Documentation Tasks by Agent

#### Technical Lead (Primary Documentation Owner):
- [ ] **Update Main README.md**: Add new status-related API endpoints to the API reference table
- [ ] **Update CLAUDE.md**: Add status transition patterns and business rules guidance for future development
- [ ] **Create Status Transitions Guide**: New documentation file explaining the business rules, valid transitions, and usage patterns
- [ ] **Update API Documentation**: Comprehensive endpoint documentation with examples and error responses
- [ ] **Architecture Documentation Review**: Ensure all architectural decisions are properly documented

#### Software Architect (Architecture Documentation):
- [ ] **Update Hexagonal Architecture docs**: Document how status transitions demonstrate domain modeling principles
- [ ] **Domain Layer Documentation**: Add status transition patterns to domain design guidelines
- [ ] **Create ADR (Architecture Decision Record)**: Document key architectural decisions made during implementation

#### Product Owner (Business Documentation):
- [ ] **Update User Stories Documentation**: Document lessons learned and user feedback
- [ ] **Create Feature Acceptance Documentation**: Final acceptance criteria validation and sign-off
- [ ] **Business Rules Documentation**: Comprehensive documentation of approval workflows and business logic

#### QA Engineer (Testing Documentation):
- [ ] **Update Testing Strategy docs**: Document new testing patterns for status transitions
- [ ] **Create Test Coverage Report**: Document final test coverage and quality metrics
- [ ] **Performance Testing Results**: Document performance benchmarks and optimization techniques

#### Backend Developer (API Documentation):
- [ ] **OpenAPI/Swagger Documentation**: Update API specification with new endpoints
- [ ] **Code Documentation**: Ensure all new code has proper inline documentation
- [ ] **Integration Examples**: Provide example code for common status transition scenarios

#### Database Engineer (Data Documentation):
- [ ] **Database Schema Documentation**: Document new tables, indexes, and relationships
- [ ] **Migration Documentation**: Document migration procedures and rollback strategies
- [ ] **Performance Optimization Guide**: Document query optimization techniques and monitoring

### Documentation Timeline

**Week 1 (Immediately after epic completion)**:
- Technical Lead updates main project documentation (README.md, CLAUDE.md)
- Product Owner validates and documents final acceptance criteria
- QA Engineer publishes test coverage and performance reports

**Week 2 (Consolidation phase)**:
- Software Architect creates comprehensive architecture documentation
- Backend Developer completes API documentation with examples
- Database Engineer documents schema changes and optimization strategies

### Documentation Quality Standards

- **Completeness**: All new features must be documented with examples
- **Accuracy**: Documentation must reflect the actual implemented behavior
- **Maintainability**: Documentation should be easy to update as the feature evolves
- **Accessibility**: Documentation should be understandable by both technical and business stakeholders

### Documentation Review Process

1. **Technical Lead** coordinates all documentation updates
2. **Software Architect** reviews for architectural consistency
3. **Product Owner** validates business requirement coverage
4. **QA Engineer** verifies examples and test case documentation

---

## Future Enhancements

After this foundation is complete, consider:

1. **Notifications**: Email/webhook notifications on status changes
2. **Automation**: Auto-transition based on time or external events
3. **Workflows**: Custom approval chains for different task types
4. **Analytics Dashboard**: Visual reporting on task lifecycle metrics
5. **SLA Tracking**: Monitor and alert on tasks exceeding time limits

---

## Related Documentation

- [Hexagonal Architecture Overview](../docs/06_hexagonal_architecture.md)
- [Domain Layer Design](../docs/07_domain_layer.md)
- [Application Layer & Use Cases](../docs/08_application_layer.md)
- [Testing Strategy](../agents/qa-engineer.md)

---

**Plan Created**: 2025-08-15  
**Estimated Completion**: 7-10 business days  
**Documentation Timeline**: 2 weeks post-completion  
**Next Review**: After Phase 1 completion