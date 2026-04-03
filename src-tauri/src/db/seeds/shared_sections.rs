use super::super::types::AppError;
use rusqlite::Connection;

/// Add a shared section with its checklist items. Returns the section ID.
pub fn add_shared_section(
    conn: &Connection,
    name: &str,
    description: &str,
    color: &str,
    items: &[(&str, &str)],
) -> Result<i64, AppError> {
    conn.execute(
        "INSERT INTO shared_sections (name, description, color) VALUES (?1, ?2, ?3)",
        (name, description, color),
    )?;
    let section_id = conn.last_insert_rowid();
    for (i, (title, desc)) in items.iter().enumerate() {
        conn.execute(
            "INSERT INTO shared_section_items (section_id, title, description, sort_order) VALUES (?1, ?2, ?3, ?4)",
            (section_id, title, desc, i as i64),
        )?;
    }
    Ok(section_id)
}

pub fn seed(conn: &Connection) -> Result<Vec<(String, i64)>, AppError> {
    let mut sections = Vec::new();

    // 1. Project Planning & Scoping
    let id = add_shared_section(
        conn,
        "Project Planning & Scoping",
        "Requirements gathering, architecture decisions, and project setup",
        "#6366f1",
        &[
            (
                "Stakeholder interviews conducted",
                "Key requirements and constraints documented",
            ),
            (
                "Product requirements document written",
                "User stories with acceptance criteria defined",
            ),
            (
                "Technical architecture diagram created",
                "System components, data flow, and boundaries mapped",
            ),
            (
                "Tech stack decision documented",
                "Languages, frameworks, and libraries selected with rationale",
            ),
            (
                "Repository initialized with structure",
                "Monorepo or multi-repo layout, branch protection rules",
            ),
            (
                "Development environment configured",
                "IDE setup, linters, formatters, pre-commit hooks",
            ),
            (
                "Project management board set up",
                "Backlog created, sprint cadence defined, estimation method chosen",
            ),
            (
                "Risk assessment completed",
                "Technical risks, dependencies, and mitigation strategies identified",
            ),
            (
                "CI pipeline skeleton created",
                "Build, lint, and test stages configured on every push",
            ),
            (
                "Initial project timeline estimated",
                "Milestones, dependencies, and critical path identified",
            ),
        ],
    )?;
    sections.push(("planning".to_string(), id));

    // 2. Security Fundamentals
    let id = add_shared_section(
        conn,
        "Security Fundamentals",
        "Core security practices applied across all project types",
        "#ef4444",
        &[
            (
                "Threat modeling completed",
                "STRIDE or similar analysis for all system components",
            ),
            (
                "Authentication flow implemented",
                "JWT, OAuth2, or session-based auth with secure token storage",
            ),
            (
                "Input validation on all entry points",
                "Server-side validation with schema enforcement",
            ),
            (
                "Secrets management configured",
                "No hardcoded credentials; env vars or secret manager used",
            ),
            (
                "HTTPS/TLS enforced everywhere",
                "Certificate management, HSTS headers, no mixed content",
            ),
            (
                "Dependency vulnerability scan configured",
                "Automated scanning with Snyk, Dependabot, or similar",
            ),
            (
                "OWASP Top 10 checklist reviewed",
                "Each vulnerability category assessed and mitigated",
            ),
            (
                "Error messages sanitized",
                "No stack traces or internal details exposed to users",
            ),
            (
                "Rate limiting implemented",
                "API endpoints protected against brute force and abuse",
            ),
            (
                "Security headers configured",
                "CSP, X-Frame-Options, X-Content-Type-Options, Referrer-Policy",
            ),
        ],
    )?;
    sections.push(("security".to_string(), id));

    // 3. Testing & QA
    let id = add_shared_section(
        conn,
        "Testing & QA",
        "Comprehensive testing strategy and quality assurance",
        "#10b981",
        &[
            (
                "Unit test framework configured",
                "Assertion library, mocking framework, and test runner set up",
            ),
            (
                "Core logic unit tests written",
                "Critical business logic covered with >80% coverage",
            ),
            (
                "Integration test suite created",
                "API endpoints, database operations, and external services tested",
            ),
            (
                "E2E test pipeline established",
                "Critical user flows automated with Playwright/Cypress",
            ),
            (
                "Test data strategy defined",
                "Fixtures, factories, or seed data for consistent test runs",
            ),
            (
                "Mock services configured",
                "External API mocks for isolated testing",
            ),
            (
                "Code coverage gate enforced",
                "Minimum coverage threshold in CI pipeline",
            ),
            (
                "Regression test suite maintained",
                "Bug fixes accompanied by regression tests",
            ),
            (
                "Performance benchmarks established",
                "Baseline metrics for response times and throughput",
            ),
            (
                "QA sign-off process defined",
                "Checklist for release readiness and acceptance criteria",
            ),
        ],
    )?;
    sections.push(("testing".to_string(), id));

    // 4. CI/CD Pipeline
    let id = add_shared_section(
        conn,
        "CI/CD Pipeline",
        "Automated build, test, and deployment pipeline",
        "#f59e0b",
        &[
            (
                "Build pipeline automated",
                "Clean build from source with dependency resolution",
            ),
            (
                "Test automation in CI",
                "Unit, integration, and linting run on every PR",
            ),
            (
                "Artifact versioning strategy defined",
                "Semantic versioning with automated tag generation",
            ),
            (
                "Staging environment deployed",
                "Mirror of production for pre-release validation",
            ),
            (
                "Production deployment automated",
                "Zero-downtime deploy with health checks",
            ),
            (
                "Rollback strategy implemented",
                "One-click rollback with data migration reversal",
            ),
            (
                "Release notes automation configured",
                "Changelog generated from commit history",
            ),
            (
                "Environment parity ensured",
                "Dev, staging, and prod use same config structure",
            ),
            (
                "Secrets rotation automated",
                "Credentials rotated on schedule without downtime",
            ),
            (
                "Deployment notifications configured",
                "Slack/email alerts on deploy success or failure",
            ),
        ],
    )?;
    sections.push(("cicd".to_string(), id));

    // 5. Documentation
    let id = add_shared_section(
        conn,
        "Documentation",
        "Technical and user documentation for the project",
        "#8b5cf6",
        &[
            (
                "README with quick-start guide",
                "Setup, run, and test instructions for new developers",
            ),
            (
                "API documentation generated",
                "OpenAPI/Swagger spec with interactive explorer",
            ),
            (
                "Architecture decision records maintained",
                "Key technical decisions with context and consequences",
            ),
            (
                "Deployment guide written",
                "Step-by-step production deployment with troubleshooting",
            ),
            (
                "Contributing guide created",
                "Code style, PR process, and review expectations",
            ),
            (
                "Changelog maintained",
                "User-facing changes tracked per release",
            ),
            (
                "Code comments standard enforced",
                "Public APIs documented with examples",
            ),
            (
                "Onboarding guide for new team members",
                "First-week checklist with environment and access setup",
            ),
            (
                "Runbook for common operations",
                "Debugging steps, log locations, and escalation paths",
            ),
            (
                "Post-mortem template created",
                "Incident review format with action item tracking",
            ),
        ],
    )?;
    sections.push(("docs".to_string(), id));

    // 6. Code Quality & Review
    let id = add_shared_section(
        conn,
        "Code Quality & Review",
        "Code standards, review processes, and maintainability",
        "#06b6d4",
        &[
            (
                "Linter configured with project rules",
                "ESLint, Clippy, or language-specific linter with strict mode",
            ),
            (
                "Code formatter enforced",
                "Prettier, rustfmt, or equivalent with pre-commit hook",
            ),
            (
                "PR template created",
                "Description, testing notes, and checklist for reviewers",
            ),
            (
                "Code review checklist defined",
                "Security, performance, readability, and test coverage checks",
            ),
            (
                "Branch protection rules enabled",
                "Required reviews, status checks, and signed commits",
            ),
            (
                "Conventional commit format adopted",
                "Structured commit messages for changelog generation",
            ),
            (
                "Static analysis integrated",
                "SonarQube, CodeQL, or similar for code quality metrics",
            ),
            (
                "Complexity limits enforced",
                "Cyclomatic complexity thresholds with CI gate",
            ),
            (
                "Technical debt tracking system",
                "Debt items logged with priority and estimated effort",
            ),
            (
                "Pair review policy established",
                "Critical code paths require two reviewer approvals",
            ),
        ],
    )?;
    sections.push(("quality".to_string(), id));

    // 7. Performance Baseline
    let id = add_shared_section(
        conn,
        "Performance Baseline",
        "Performance profiling, optimization, and monitoring setup",
        "#f97316",
        &[
            (
                "Profiling tools configured",
                "CPU, memory, and I/O profiling in dev environment",
            ),
            (
                "Benchmark suite created",
                "Automated benchmarks for critical code paths",
            ),
            (
                "Memory profiling established",
                "Leak detection and memory usage baselines",
            ),
            (
                "Load test baseline recorded",
                "Requests/second and latency at expected traffic levels",
            ),
            (
                "Caching strategy defined",
                "Redis, CDN, or in-memory caching with invalidation rules",
            ),
            (
                "Database query optimization reviewed",
                "Slow query log enabled, indexes analyzed, N+1 queries fixed",
            ),
            (
                "Bundle/asset size audited",
                "Tree-shaking, code splitting, and compression verified",
            ),
            (
                "Performance budget set",
                "Maximum page weight, TTI, and LCP thresholds defined",
            ),
            (
                "Image optimization pipeline",
                "WebP/AVIF conversion, lazy loading, and responsive sizes",
            ),
            (
                "CDN strategy implemented",
                "Static assets served from edge with cache headers",
            ),
        ],
    )?;
    sections.push(("performance".to_string(), id));

    // 8. Monitoring & Observability
    let id = add_shared_section(
        conn,
        "Monitoring & Observability",
        "Logging, metrics, tracing, and incident response",
        "#ec4899",
        &[
            (
                "Structured logging framework integrated",
                "JSON logs with correlation IDs and log levels",
            ),
            (
                "Metrics collection configured",
                "Prometheus, Datadog, or similar for key metrics",
            ),
            (
                "Distributed tracing enabled",
                "OpenTelemetry or similar for cross-service request tracking",
            ),
            (
                "Alert rules defined",
                "Threshold-based and anomaly-based alerts with escalation",
            ),
            (
                "Dashboard created",
                "Real-time view of system health, errors, and performance",
            ),
            (
                "Error tracking integrated",
                "Sentry, Rollbar, or similar for exception aggregation",
            ),
            (
                "Uptime monitoring configured",
                "External health checks with multi-region probes",
            ),
            (
                "SLA/SLO targets defined",
                "Availability, latency, and error rate objectives documented",
            ),
            (
                "Incident response plan created",
                "On-call rotation, escalation matrix, and communication templates",
            ),
            (
                "Runbook automation set up",
                "Common remediation steps automated with self-healing",
            ),
        ],
    )?;
    sections.push(("monitoring".to_string(), id));

    // 9. Database & Data Management
    let id = add_shared_section(
        conn,
        "Database & Data Management",
        "Schema design, migrations, backups, and data integrity",
        "#14b8a6",
        &[
            (
                "Database schema designed and reviewed",
                "Normalized tables, foreign keys, and indexes planned",
            ),
            (
                "Migration system configured",
                "Versioned migrations with rollback support",
            ),
            (
                "Connection pooling configured",
                "PgBouncer, connection limits, and timeout settings",
            ),
            (
                "Backup strategy implemented",
                "Automated daily backups with point-in-time recovery",
            ),
            (
                "Data validation at application layer",
                "Schema validation before write operations",
            ),
            (
                "Index strategy documented",
                "Query patterns analyzed, composite indexes planned",
            ),
            (
                "Soft delete pattern implemented",
                "Deleted_at timestamps with unique partial indexes",
            ),
            (
                "Audit logging configured",
                "Create, update, delete events tracked with user context",
            ),
            (
                "Data retention policy defined",
                "Archival and deletion schedules for compliance",
            ),
            (
                "Database access restricted",
                "Least privilege roles, read replicas for queries",
            ),
        ],
    )?;
    sections.push(("database".to_string(), id));

    // 10. Accessibility & UX
    let id = add_shared_section(
        conn,
        "Accessibility & UX",
        "WCAG compliance, usability testing, and inclusive design",
        "#a855f7",
        &[
            (
                "WCAG 2.1 AA compliance checklist",
                "Color contrast, keyboard nav, screen reader support",
            ),
            (
                "ARIA labels on interactive elements",
                "Buttons, forms, and dynamic content properly labeled",
            ),
            (
                "Keyboard navigation tested",
                "All features accessible without mouse",
            ),
            (
                "Screen reader compatibility verified",
                "NVDA, VoiceOver, and JAWS tested",
            ),
            (
                "Focus management implemented",
                "Logical tab order, visible focus indicators, trap handling",
            ),
            (
                "Responsive design tested",
                "Mobile, tablet, and desktop breakpoints verified",
            ),
            (
                "Color contrast ratios validated",
                "All text meets 4.5:1 minimum ratio",
            ),
            (
                "Reduced motion support added",
                "Animations respect prefers-reduced-motion",
            ),
            (
                "Form error messages accessible",
                "Inline errors with aria-describedby and focus management",
            ),
            (
                "Usability testing conducted",
                "Real user testing with task completion metrics",
            ),
        ],
    )?;
    sections.push(("a11y".to_string(), id));

    Ok(sections)
}
