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
    let a11y = section_map.get("a11y").copied().unwrap_or(0);

    let _planning_setup = sprint_map.get("planning_setup").copied().unwrap_or(0);
    let _security_quality = sprint_map.get("security_quality").copied().unwrap_or(0);
    let _testing_qa = sprint_map.get("testing_qa").copied().unwrap_or(0);
    let _cicd_deploy = sprint_map.get("cicd_deploy").copied().unwrap_or(0);
    let _monitoring_ops = sprint_map.get("monitoring_ops").copied().unwrap_or(0);
    let _perf_sprint = sprint_map.get("performance").copied().unwrap_or(0);
    let _db_sprint = sprint_map.get("database").copied().unwrap_or(0);
    let _a11y_sprint = sprint_map.get("a11y_ux").copied().unwrap_or(0);

    // === Full-Stack Web App ===
    let tpl = add_template(
        conn,
        "Full-Stack Web App",
        "React/Next.js + Node/Python backend — complete web application checklist",
        "#3b82f6",
    )?;

    // Sprint 1: Planning & Setup (shared)
    let s1 = add_template_sprint(
        conn,
        tpl,
        "Planning & Setup",
        "Project kickoff, requirements, and dev environment",
        0,
    )?;
    add_template_sprint_sections(conn, s1, &[planning, quality])?;

    // Sprint 2: Architecture & Design
    let s2 = add_template_sprint(
        conn,
        tpl,
        "Architecture & Design",
        "System architecture, UI/UX design, and tech decisions",
        1,
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Frontend Architecture",
        "Component structure, state management, and routing",
        "#6366f1",
        &[
            (
                "Component hierarchy designed",
                "Atomic design or feature-based folder structure",
            ),
            (
                "State management solution chosen",
                "Redux, Zustand, Context API, or server state library",
            ),
            (
                "Routing structure defined",
                "File-based or config-based routing with nested routes",
            ),
            (
                "Design system or component library selected",
                "Tailwind, shadcn, Material UI, or custom",
            ),
            (
                "API client layer created",
                "Axios/fetch wrapper with interceptors and error handling",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Backend Architecture",
        "API design, service layer, and data models",
        "#8b5cf6",
        &[
            (
                "API contract defined",
                "OpenAPI/Swagger spec with request/response schemas",
            ),
            (
                "Service layer structure planned",
                "Controllers, services, repositories pattern",
            ),
            (
                "Data models designed",
                "Entity relationships, DTOs, and validation schemas",
            ),
            (
                "Authentication strategy chosen",
                "JWT, sessions, OAuth2, or API keys",
            ),
            (
                "Error handling strategy defined",
                "Global error handler, error codes, and response format",
            ),
        ],
    )?;

    // Sprint 3: Database Setup
    let s3 = add_template_sprint(
        conn,
        tpl,
        "Database Setup",
        "Schema, migrations, and data layer",
        2,
    )?;
    add_template_sprint_sections(conn, s3, &[database])?;
    add_custom_sprint_section(
        conn,
        s3,
        "ORM & Data Access",
        "Database abstraction and query patterns",
        "#14b8a6",
        &[
            (
                "ORM or query builder configured",
                "Prisma, Drizzle, TypeORM, or raw SQL with migrations",
            ),
            (
                "Seed data scripts created",
                "Initial data for development and testing environments",
            ),
            (
                "Repository pattern implemented",
                "Data access layer with typed queries",
            ),
            (
                "Transaction management configured",
                "Rollback on failure, isolation levels set",
            ),
            (
                "Database connection tested",
                "Health check endpoint verifying DB connectivity",
            ),
        ],
    )?;

    // Sprint 4: Authentication & Authorization
    let s4 = add_template_sprint(
        conn,
        tpl,
        "Auth & Authorization",
        "User authentication, roles, and permissions",
        3,
    )?;
    add_custom_sprint_section(
        conn,
        s4,
        "Authentication Flow",
        "Login, registration, and session management",
        "#ef4444",
        &[
            (
                "Registration endpoint implemented",
                "Email/password or OAuth with validation",
            ),
            (
                "Login endpoint with token generation",
                "JWT or session-based with refresh token support",
            ),
            (
                "Password hashing configured",
                "bcrypt, argon2, or similar with proper salt rounds",
            ),
            (
                "Email verification flow added",
                "Token-based email confirmation with expiry",
            ),
            (
                "Password reset flow implemented",
                "Secure token generation with time-limited links",
            ),
            (
                "Session management configured",
                "Token expiry, refresh rotation, and logout invalidation",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s4,
        "Authorization & Roles",
        "Access control and permission management",
        "#f97316",
        &[
            (
                "Role-based access control implemented",
                "Admin, user, moderator roles with permissions",
            ),
            (
                "Route guards configured",
                "Protected routes with redirect logic",
            ),
            (
                "API middleware for auth checks",
                "Token validation and role verification on endpoints",
            ),
            (
                "Resource-level permissions defined",
                "CRUD permissions per resource type",
            ),
            (
                "Audit logging for auth events",
                "Login attempts, role changes, and permission grants logged",
            ),
        ],
    )?;

    // Sprint 5: Core Features Development
    let s5 = add_template_sprint(
        conn,
        tpl,
        "Core Features",
        "Main application functionality and business logic",
        4,
    )?;
    add_custom_sprint_section(
        conn,
        s5,
        "Backend API Endpoints",
        "RESTful or GraphQL API implementation",
        "#06b6d4",
        &[
            (
                "CRUD endpoints for primary resources",
                "Create, read, update, delete with proper HTTP methods",
            ),
            (
                "Pagination and filtering implemented",
                "Cursor or offset pagination with query filters",
            ),
            (
                "Input validation on all endpoints",
                "Zod, Joi, or Pydantic schema validation",
            ),
            (
                "File upload handling configured",
                "Multipart form data with size limits and type checks",
            ),
            (
                "API versioning strategy applied",
                "URL path, header, or query parameter versioning",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s5,
        "Frontend Pages & Components",
        "UI implementation and user flows",
        "#3b82f6",
        &[
            (
                "Core page layouts built",
                "Dashboard, list, detail, and form pages",
            ),
            (
                "Data fetching with loading states",
                "React Query, SWR, or similar with skeletons",
            ),
            (
                "Form handling with validation",
                "React Hook Form or similar with server-side sync",
            ),
            (
                "Error boundaries implemented",
                "Graceful error UI with retry options",
            ),
            (
                "Empty states designed",
                "Meaningful empty state UI with call-to-action",
            ),
        ],
    )?;

    // Sprint 6: Security Hardening (shared)
    let s6 = add_template_sprint(
        conn,
        tpl,
        "Security Hardening",
        "Security review and vulnerability mitigation",
        5,
    )?;
    add_template_sprint_sections(conn, s6, &[security, quality])?;
    add_custom_sprint_section(
        conn,
        s6,
        "Web Security",
        "Browser and HTTP security measures",
        "#ef4444",
        &[
            (
                "CSP headers configured",
                "Content-Security-Policy with strict directives",
            ),
            (
                "CSRF protection enabled",
                "SameSite cookies or CSRF tokens for state-changing requests",
            ),
            (
                "XSS prevention verified",
                "Output encoding, no innerHTML, sanitization on rich text",
            ),
            (
                "CORS policy configured",
                "Explicit allowed origins, methods, and headers",
            ),
            (
                "Cookie security flags set",
                "HttpOnly, Secure, SameSite attributes on all cookies",
            ),
        ],
    )?;

    // Sprint 7: Testing & QA (shared)
    let s7 = add_template_sprint(
        conn,
        tpl,
        "Testing & QA",
        "Comprehensive testing across all layers",
        6,
    )?;
    add_template_sprint_sections(conn, s7, &[testing])?;
    add_custom_sprint_section(
        conn,
        s7,
        "Frontend Testing",
        "Component and integration tests for UI",
        "#10b981",
        &[
            (
                "Component unit tests written",
                "Individual component rendering and interaction tests",
            ),
            (
                "Integration tests for user flows",
                "Multi-component interaction and state changes",
            ),
            (
                "Visual regression tests configured",
                "Percy, Chromatic, or similar for UI diffing",
            ),
            (
                "Accessibility tests automated",
                "axe-core or similar in test pipeline",
            ),
            (
                "Mock API server configured",
                "MSW or similar for consistent test data",
            ),
        ],
    )?;

    // Sprint 8: Performance Optimization
    let s8 = add_template_sprint(
        conn,
        tpl,
        "Performance Optimization",
        "Frontend and backend performance tuning",
        7,
    )?;
    add_template_sprint_sections(conn, s8, &[performance])?;
    add_custom_sprint_section(
        conn,
        s8,
        "Frontend Performance",
        "Client-side optimization techniques",
        "#f97316",
        &[
            (
                "Code splitting implemented",
                "Route-level and component-level lazy loading",
            ),
            (
                "Bundle analysis completed",
                "webpack-bundle-analyzer or similar reviewed",
            ),
            (
                "Image optimization pipeline active",
                "Next.js Image, sharp, or CDN optimization",
            ),
            (
                "Core Web Vitals measured",
                "LCP, FID, CLS metrics within thresholds",
            ),
            (
                "Server-side rendering or SSG configured",
                "SSR for dynamic content, SSG for static pages",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s8,
        "Backend Performance",
        "Server-side optimization techniques",
        "#f59e0b",
        &[
            (
                "Database query optimization reviewed",
                "Slow query log analyzed, indexes added",
            ),
            (
                "Response caching implemented",
                "Redis or in-memory cache for frequent queries",
            ),
            (
                "Connection pooling verified",
                "Database and HTTP connection pools configured",
            ),
            (
                "Compression enabled",
                "gzip or brotli compression on API responses",
            ),
            (
                "N+1 query problems eliminated",
                "Eager loading or join queries for related data",
            ),
        ],
    )?;

    // Sprint 9: Accessibility & UX (shared)
    let s9 = add_template_sprint(
        conn,
        tpl,
        "Accessibility & UX",
        "WCAG compliance and usability improvements",
        8,
    )?;
    add_template_sprint_sections(conn, s9, &[a11y])?;

    // Sprint 10: CI/CD & Deployment (shared)
    let s10 = add_template_sprint(
        conn,
        tpl,
        "CI/CD & Deployment",
        "Automated pipeline and production launch",
        9,
    )?;
    add_template_sprint_sections(conn, s10, &[cicd, docs])?;

    // Sprint 11: Monitoring & Operations (shared)
    let s11 = add_template_sprint(
        conn,
        tpl,
        "Monitoring & Operations",
        "Observability and production readiness",
        10,
    )?;
    add_template_sprint_sections(conn, s11, &[monitoring, docs])?;

    // Sprint 12: Launch & Handoff
    let s12 = add_template_sprint(
        conn,
        tpl,
        "Launch & Handoff",
        "Production launch and knowledge transfer",
        11,
    )?;
    add_custom_sprint_section(
        conn,
        s12,
        "Production Readiness",
        "Final checks before going live",
        "#10b981",
        &[
            (
                "Production environment verified",
                "All services healthy, configs correct, secrets set",
            ),
            (
                "Domain and SSL configured",
                "DNS records, CDN setup, HTTPS certificate active",
            ),
            (
                "Database migration to production run",
                "Schema changes applied with rollback plan",
            ),
            (
                "Smoke tests passed in production",
                "Critical flows verified in live environment",
            ),
            (
                "Rollback plan tested",
                "Rollback procedure validated with test deployment",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s12,
        "Knowledge Transfer",
        "Documentation and team handoff",
        "#8b5cf6",
        &[
            (
                "Architecture walkthrough completed",
                "System design presented to operations team",
            ),
            (
                "Runbook delivered",
                "Common issues, debugging steps, and escalation paths documented",
            ),
            (
                "On-call rotation established",
                "PagerDuty or similar with escalation policies",
            ),
            (
                "Post-launch monitoring period defined",
                "Hypercare period with enhanced monitoring",
            ),
            (
                "Retrospective scheduled",
                "Lessons learned and improvement items captured",
            ),
        ],
    )?;

    Ok(())
}
