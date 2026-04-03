use super::{
    add_custom_sprint_section, add_template, add_template_sprint, add_template_sprint_sections,
};
use super::super::super::types::AppError;
use rusqlite::Connection;

pub fn seed(
    conn: &Connection,
    section_map: &std::collections::HashMap<String, i64>,
    sprint_map: &std::collections::HashMap<String, i64>,
) -> Result<(), AppError> {
    let planning = section_map.get("planning").copied().unwrap_or(0);
    let security = section_map.get("security").copied().unwrap_or(0);
    let testing = section_map.get("testing").copied().unwrap_or(0);
    let cicd = section_map.get("cicd").copied().unwrap_or(0);
    let docs = section_map.get("docs").copied().unwrap_or(0);
    let quality = section_map.get("quality").copied().unwrap_or(0);
    let performance = section_map.get("performance").copied().unwrap_or(0);
    let monitoring = section_map.get("monitoring").copied().unwrap_or(0);
    let database = section_map.get("database").copied().unwrap_or(0);

    let testing_qa = sprint_map.get("testing_qa").copied().unwrap_or(0);
    let cicd_deploy = sprint_map.get("cicd_deploy").copied().unwrap_or(0);
    let monitoring_ops = sprint_map.get("monitoring_ops").copied().unwrap_or(0);
    let perf_sprint = sprint_map.get("performance").copied().unwrap_or(0);
    let db_sprint = sprint_map.get("database").copied().unwrap_or(0);
    let security_quality = sprint_map.get("security_quality").copied().unwrap_or(0);

    let tpl = add_template(
        conn,
        "API & Backend Development",
        "REST, GraphQL, microservices, and backend service architecture",
        "#06b6d4",
    )?;

    let s1 = add_template_sprint(
        conn,
        tpl,
        "Planning & Setup",
        "API design and architecture",
        0,
    )?;
    add_template_sprint_sections(conn, s1, &[planning, quality])?;

    let s2 = add_template_sprint(
        conn,
        tpl,
        "API Design & Contracts",
        "OpenAPI spec and endpoint design",
        1,
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "API Specification",
        "REST/GraphQL contract definition",
        "#06b6d4",
        &[
            (
                "OpenAPI/Swagger spec written",
                "All endpoints documented with request/response schemas",
            ),
            (
                "HTTP methods and status codes defined",
                "Proper use of GET, POST, PUT, PATCH, DELETE",
            ),
            (
                "Request validation schemas created",
                "Zod, Joi, Pydantic, or JSON Schema for all inputs",
            ),
            (
                "Response envelope standard defined",
                "Consistent error format, pagination, and metadata",
            ),
            (
                "API versioning strategy chosen",
                "URL path, header, or content negotiation versioning",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Data Models & DTOs",
        "Entity and transfer object design",
        "#14b8a6",
        &[
            (
                "Domain entities defined",
                "Core business objects with relationships mapped",
            ),
            (
                "DTOs separated from domain models",
                "Input/output DTOs with validation annotations",
            ),
            (
                "Database schema designed",
                "Tables, indexes, constraints, and foreign keys",
            ),
            (
                "Migration scripts created",
                "Versioned migrations with up/down scripts",
            ),
            (
                "Enum and constant definitions",
                "Shared enums for status codes, types, and roles",
            ),
        ],
    )?;

    let s3 = add_template_sprint(
        conn,
        tpl,
        "Core API Implementation",
        "Endpoints, services, and middleware",
        2,
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Endpoint Implementation",
        "Controller/route layer",
        "#06b6d4",
        &[
            (
                "CRUD endpoints implemented",
                "Create, read, update, delete with proper HTTP semantics",
            ),
            (
                "Pagination and filtering added",
                "Cursor or offset pagination with query parameters",
            ),
            (
                "Sorting and search implemented",
                "Multi-field sorting and full-text search",
            ),
            (
                "File upload/download endpoints",
                "Multipart handling with size limits and type validation",
            ),
            (
                "Batch operations supported",
                "Bulk create/update/delete with transaction wrapping",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Service & Business Logic",
        "Application layer implementation",
        "#14b8a6",
        &[
            (
                "Service layer with business rules",
                "Domain logic separated from HTTP layer",
            ),
            (
                "Dependency injection configured",
                "IoC container or constructor injection",
            ),
            (
                "Transaction management implemented",
                "ACID transactions with proper isolation levels",
            ),
            (
                "Event-driven patterns applied",
                "Domain events, event handlers, and pub/sub",
            ),
            (
                "Background job processing configured",
                "Message queue with worker pool for async tasks",
            ),
        ],
    )?;

    let s4 = add_template_sprint(
        conn,
        tpl,
        "Auth & Middleware",
        "Authentication, authorization, and cross-cutting concerns",
        3,
    )?;
    add_custom_sprint_section(
        conn,
        s4,
        "Authentication & Authorization",
        "Identity and access management",
        "#ef4444",
        &[
            (
                "JWT or session auth implemented",
                "Token generation, validation, and refresh flow",
            ),
            (
                "Role-based access control configured",
                "Middleware for route-level permission checks",
            ),
            (
                "API key authentication supported",
                "Key generation, rotation, and rate limit tiers",
            ),
            (
                "OAuth2/OIDC integration completed",
                "Third-party login with Google, GitHub, etc.",
            ),
            (
                "Rate limiting middleware added",
                "Token bucket or sliding window per client/IP",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s4,
        "Middleware & Cross-Cutting",
        "Logging, error handling, and request processing",
        "#8b5cf6",
        &[
            (
                "Request logging middleware",
                "Structured logs with correlation IDs and timing",
            ),
            (
                "Global error handler implemented",
                "Consistent error responses with proper status codes",
            ),
            (
                "CORS middleware configured",
                "Allowed origins, methods, headers, and credentials",
            ),
            (
                "Request ID and tracing headers",
                "X-Request-ID propagated through all services",
            ),
            (
                "Health check endpoints added",
                "/health, /ready, and /live endpoints for K8s",
            ),
        ],
    )?;

    let s5 = add_template_sprint(
        conn,
        tpl,
        "Database & Data Layer",
        "Persistence, caching, and data integrity",
        4,
    )?;
    add_template_sprint_sections(conn, s5, &[database])?;
    add_custom_sprint_section(
        conn,
        s5,
        "Caching Strategy",
        "Redis, CDN, and application-level caching",
        "#f59e0b",
        &[
            (
                "Cache layer configured",
                "Redis or Memcached with connection pooling",
            ),
            (
                "Cache invalidation strategy defined",
                "TTL, write-through, or event-driven invalidation",
            ),
            (
                "Query result caching implemented",
                "Frequently-read data cached with appropriate TTL",
            ),
            (
                "Cache warming strategy defined",
                "Pre-populate cache on deploy or schedule",
            ),
            (
                "Cache hit/miss metrics tracked",
                "Monitoring for cache effectiveness",
            ),
        ],
    )?;

    let s6 = add_template_sprint(
        conn,
        tpl,
        "Security Hardening",
        "API security and vulnerability mitigation",
        5,
    )?;
    add_template_sprint_sections(conn, s6, &[security])?;

    let s7 = add_template_sprint(
        conn,
        tpl,
        "Testing & QA",
        "API testing and quality assurance",
        6,
    )?;
    add_template_sprint_sections(conn, s7, &[testing])?;
    add_custom_sprint_section(
        conn,
        s7,
        "API Testing",
        "Contract, integration, and load testing",
        "#10b981",
        &[
            (
                "Contract tests for all endpoints",
                "Schema validation against OpenAPI spec",
            ),
            (
                "Integration tests with test database",
                "Full request/response cycle with real DB",
            ),
            (
                "Load testing completed",
                "k6 or Locust with target RPS and latency thresholds",
            ),
            (
                "API fuzzing performed",
                "Malformed inputs, boundary values, and injection attempts",
            ),
            (
                "Mock server for consumers",
                "WireMock or Prism for frontend team parallel development",
            ),
        ],
    )?;

    let s8 = add_template_sprint(
        conn,
        tpl,
        "Performance Optimization",
        "Query optimization and scaling",
        7,
    )?;
    add_template_sprint_sections(conn, s8, &[performance])?;

    let s9 = add_template_sprint(
        conn,
        tpl,
        "CI/CD & Deployment",
        "Automated pipeline and production launch",
        8,
    )?;
    add_template_sprint_sections(conn, s9, &[cicd, docs])?;

    let s10 = add_template_sprint(
        conn,
        tpl,
        "Monitoring & Operations",
        "Observability and production readiness",
        9,
    )?;
    add_template_sprint_sections(conn, s10, &[monitoring, docs])?;

    Ok(())
}
