# AI Development Team Agents

This directory contains AI agents representing different roles in a software development team, each specialized for working with our Rust REST API project built using **Hexagonal Architecture**.

## 🏗️ Agent Directory

### [Software Architect](./software-architect.md)
**Specialization**: Hexagonal Architecture & System Design  
**When to use**: Planning major architectural changes, designing new features that span multiple layers, making technology decisions, ensuring architectural compliance

**Key scenarios**:
- Adding new bounded contexts or domains
- Designing integration with external services
- Refactoring for better separation of concerns
- Making framework or technology choices

---

### [Backend Developer](./backend-developer.md)
**Specialization**: Rust, Axum Framework & API Development  
**When to use**: Implementing REST endpoints, handling HTTP requests, writing async Rust code, integrating web layer with application services

**Key scenarios**:
- Creating new API endpoints
- Implementing middleware and request handling
- Adding authentication and authorization
- Optimizing API performance

---

### [Database Engineer](./database-engineer.md)
**Specialization**: PostgreSQL, SQLx & Repository Pattern  
**When to use**: Database schema design, query optimization, implementing repository patterns, handling data persistence

**Key scenarios**:
- Designing new database tables and relationships
- Writing complex queries and optimizations
- Creating database migrations
- Implementing repository interfaces

---

### [DevOps Engineer](./devops-engineer.md)
**Specialization**: Docker, Deployment & Infrastructure  
**When to use**: Containerization, deployment pipelines, environment management, infrastructure automation

**Key scenarios**:
- Optimizing Docker builds and container configurations
- Setting up CI/CD pipelines
- Managing multi-environment deployments
- Implementing monitoring and health checks

---

### [QA Engineer](./qa-engineer.md)
**Specialization**: Testing Strategy & Quality Assurance  
**When to use**: Designing test strategies, implementing automated tests, ensuring quality coverage across all architectural layers

**Key scenarios**:
- Planning comprehensive test coverage
- Writing unit, integration, and API tests
- Setting up test automation
- Debugging test failures and improving reliability

---

### [Domain Expert](./domain-expert.md)
**Specialization**: Business Logic & Domain Modeling  
**When to use**: Implementing business rules, designing domain entities, translating business requirements into code

**Key scenarios**:
- Modeling new business entities and value objects
- Implementing complex business rules and validations
- Designing domain services and workflows
- Ensuring business logic stays in the domain layer

---

### [Product Owner](./product-owner.md)
**Specialization**: Requirements Definition & Business Strategy  
**When to use**: Defining feature requirements, prioritizing business needs, stakeholder communication, user experience design

**Key scenarios**:
- Creating user stories and acceptance criteria
- Prioritizing features based on business value
- Gathering stakeholder feedback and requirements
- Validating technical solutions against business needs

---

### [Technical Lead](./technical-lead.md)
**Specialization**: Code Review, Technical Decisions & Team Coordination  
**When to use**: Making technical decisions, reviewing complex changes, coordinating cross-functional work, resolving technical conflicts

**Key scenarios**:
- Reviewing architecture-impacting pull requests
- Making technology adoption decisions
- Coordinating complex multi-layer features
- Mentoring team on best practices

---

## 🎯 How to Choose the Right Agent

### By Task Type

| Task Type | Primary Agent | Secondary Agent |
|-----------|---------------|-----------------|
| **Requirements Gathering** | Product Owner | Domain Expert |
| **New Feature Planning** | Product Owner | Software Architect |
| **API Endpoint Creation** | Backend Developer | QA Engineer |
| **Database Schema Changes** | Database Engineer | Software Architect |
| **Business Logic Implementation** | Domain Expert | QA Engineer |
| **Performance Optimization** | Technical Lead | Database Engineer |
| **Deployment Issues** | DevOps Engineer | Technical Lead |
| **Test Strategy Design** | QA Engineer | Technical Lead |
| **Architecture Refactoring** | Software Architect | Technical Lead |

### By Layer (Hexagonal Architecture)

| Layer | Primary Agents | Use Cases |
|-------|----------------|-----------|
| **Domain Layer** | Domain Expert, QA Engineer | Business entities, validation rules, domain services, unit tests |
| **Application Layer** | Software Architect, QA Engineer | Use cases, DTOs, integration tests, workflow orchestration |
| **Infrastructure Layer** | Backend Developer, Database Engineer, DevOps Engineer | Web controllers, repositories, database migrations, deployment |

### By Problem Domain

| Problem | Recommended Agent | Why |
|---------|-------------------|-----|
| **"What should we build next?"** | Product Owner | Business prioritization and requirements expertise |
| **"Users are asking for this feature"** | Product Owner | User needs analysis and business value assessment |
| **"Our API is slow"** | Technical Lead | Needs cross-layer analysis and technical decision-making |
| **"How do we model this business rule?"** | Domain Expert | Business logic and domain modeling expertise |
| **"Tests are flaky and slow"** | QA Engineer | Testing strategy and reliability expertise |
| **"Docker builds are taking forever"** | DevOps Engineer | Container optimization and build pipeline expertise |
| **"Should we use this new crate?"** | Technical Lead | Technology evaluation and risk assessment |
| **"Database queries are complex"** | Database Engineer | SQL optimization and data modeling expertise |

## 🚀 Usage Guidelines

### 1. **Start with Planning Agents for New Features**
- Begin with **Product Owner** for requirements and business value
- Follow with **Software Architect** for feature design
- Consult **Domain Expert** for business logic modeling
- Involve **QA Engineer** early for testing strategy

### 2. **Use Implementation Agents During Development**
- **Backend Developer** for API and web layer implementation
- **Database Engineer** for data persistence concerns
- **DevOps Engineer** for deployment and infrastructure

### 3. **Leverage Review Agents for Quality**
- **Technical Lead** for complex technical decisions
- **QA Engineer** for comprehensive testing
- **Software Architect** for architectural compliance

### 4. **Cross-functional Collaboration**
Most complex tasks benefit from multiple agents:
- **Feature Development**: Product Owner → Software Architect → Domain Expert → Backend Developer → QA Engineer
- **Performance Issues**: Technical Lead → Database Engineer → DevOps Engineer
- **Requirements Clarification**: Product Owner → Domain Expert → Technical Lead
- **Deployment**: DevOps Engineer → QA Engineer → Technical Lead

## 🔧 Agent Communication Patterns

Each agent is designed to:
- **Understand hexagonal architecture** principles and boundaries
- **Provide specific, actionable guidance** within their domain
- **Reference other agents** when cross-functional expertise is needed
- **Maintain consistency** with the project's architectural decisions
- **Focus on practical implementation** rather than theoretical concepts

## 📚 Best Practices

1. **Layer Awareness**: Always consider which architectural layer your task affects
2. **Separation of Concerns**: Use the right agent for the right responsibility
3. **Cross-functional Coordination**: Don't hesitate to involve multiple agents
4. **Quality Focus**: Include QA and Technical Lead perspectives for complex changes
5. **Documentation**: Each agent emphasizes proper documentation within their domain

This agent system ensures that every aspect of our hexagonal architecture Rust application is handled by specialists who understand both the technical implementation details and the architectural principles that guide our development.