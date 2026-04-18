use super::{
    add_custom_sprint_section, add_template, add_template_sprint, add_template_sprint_sections,
};
use super::super::super::types::AppError;
use rusqlite::Connection;

pub fn seed(
    conn: &Connection,
    section_map: &std::collections::HashMap<String, i64>,
    _sprint_map: &std::collections::HashMap<String, i64>,
) -> Result<(), AppError> {
    let planning = section_map.get("planning").copied().unwrap_or(0);
    let security = section_map.get("security").copied().unwrap_or(0);
    let testing = section_map.get("testing").copied().unwrap_or(0);
    let cicd = section_map.get("cicd").copied().unwrap_or(0);
    let docs = section_map.get("docs").copied().unwrap_or(0);
    let _quality = section_map.get("quality").copied().unwrap_or(0);
    let performance = section_map.get("performance").copied().unwrap_or(0);
    let monitoring = section_map.get("monitoring").copied().unwrap_or(0);
    let database = section_map.get("database").copied().unwrap_or(0);

    let tpl = add_template(
        conn,
        "Enterprise Systems Development",
        "ERP, CRM, and large-scale internal corporate systems",
        "#475569",
    )?;

    let s1 = add_template_sprint(
        conn,
        tpl,
        "Planning & Requirements",
        "Business analysis and system architecture",
        0,
    )?;
    add_template_sprint_sections(conn, s1, &[planning])?;
    add_custom_sprint_section(
        conn,
        s1,
        "Enterprise Architecture",
        "System design for large-scale corporate systems",
        "#475569",
        &[
            (
                "Business process mapping completed",
                "As-is and to-be process flows with stakeholders",
            ),
            (
                "Enterprise architecture framework chosen",
                "TOGAF, Zachman, or custom enterprise architecture",
            ),
            (
                "Integration points identified",
                "Existing systems, third-party APIs, and data sources",
            ),
            (
                "Scalability requirements defined",
                "User counts, transaction volumes, and growth projections",
            ),
            (
                "Compliance requirements documented",
                "SOX, GDPR, HIPAA, or industry-specific regulations",
            ),
        ],
    )?;

    let s2 = add_template_sprint(
        conn,
        tpl,
        "Domain Modeling",
        "DDD, entities, and business rules",
        1,
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Domain Design",
        "Bounded contexts and domain models",
        "#64748b",
        &[
            (
                "Bounded contexts identified",
                "Clear boundaries between business domains",
            ),
            (
                "Aggregate roots defined",
                "Consistency boundaries with transactional invariants",
            ),
            (
                "Domain events modeled",
                "Business events with event sourcing capability",
            ),
            (
                "Value objects and entities designed",
                "Immutable value objects with identity entities",
            ),
            (
                "Ubiquitous language established",
                "Shared terminology between business and tech teams",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Data Architecture",
        "Enterprise data modeling and governance",
        "#06b6d4",
        &[
            (
                "Enterprise data model designed",
                "Conceptual, logical, and physical data models",
            ),
            (
                "Master data management strategy",
                "Golden record definition and data stewardship",
            ),
            (
                "Data governance framework established",
                "Data ownership, quality rules, and lifecycle policies",
            ),
            (
                "Multi-tenant architecture designed",
                "Tenant isolation with shared or dedicated resources",
            ),
            (
                "Data migration strategy planned",
                "Legacy data extraction, transformation, and loading",
            ),
        ],
    )?;

    let s3 = add_template_sprint(
        conn,
        tpl,
        "Core Module Development",
        "Primary business modules and workflows",
        2,
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Workflow Engine",
        "Business process automation and approval flows",
        "#8b5cf6",
        &[
            (
                "Workflow definition language chosen",
                "BPMN, state machines, or custom DSL",
            ),
            (
                "Approval chain implemented",
                "Multi-level approvals with escalation rules",
            ),
            (
                "Task assignment engine built",
                "Role-based, queue-based, or round-robin assignment",
            ),
            (
                "SLA tracking configured",
                "Time-based alerts and escalation for overdue tasks",
            ),
            (
                "Workflow audit trail maintained",
                "Complete history of state changes and actions",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Reporting Engine",
        "Dashboards, analytics, and report generation",
        "#a855f7",
        &[
            (
                "Report builder framework implemented",
                "Drag-and-drop report designer or template-based",
            ),
            (
                "Dashboard with real-time metrics",
                "KPI widgets, charts, and drill-down capabilities",
            ),
            (
                "Scheduled report generation",
                "Automated PDF/Excel reports with email distribution",
            ),
            (
                "Data export functionality",
                "CSV, Excel, PDF, and API export for all data views",
            ),
            (
                "Custom query builder",
                "User-friendly interface for ad-hoc data queries",
            ),
        ],
    )?;

    let s4 = add_template_sprint(
        conn,
        tpl,
        "Integration Layer",
        "APIs, webhooks, and system connectors",
        3,
    )?;
    add_custom_sprint_section(
        conn,
        s4,
        "External Integrations",
        "Third-party system connectivity",
        "#f59e0b",
        &[
            (
                "API gateway configured",
                "Centralized API management with rate limiting and auth",
            ),
            (
                "Webhook system implemented",
                "Event-driven notifications to external systems",
            ),
            (
                "Message queue integration",
                "RabbitMQ, Kafka, or SQS for async communication",
            ),
            (
                "ETL pipelines built",
                "Data extraction, transformation, and loading from external sources",
            ),
            (
                "Connector framework designed",
                "Pluggable connector architecture for new integrations",
            ),
        ],
    )?;

    let s5 = add_template_sprint(
        conn,
        tpl,
        "Role-Based Access Control",
        "Users, roles, permissions, and audit",
        4,
    )?;
    add_custom_sprint_section(
        conn,
        s5,
        "Identity & Access",
        "Enterprise-grade access management",
        "#ef4444",
        &[
            (
                "SSO/SAML integration completed",
                "Active Directory, Okta, or Azure AD integration",
            ),
            (
                "Fine-grained permissions implemented",
                "Resource-level, field-level, and action-level permissions",
            ),
            (
                "Role hierarchy defined",
                "Inheritance, delegation, and temporary role assignment",
            ),
            (
                "Audit logging for all access",
                "Who accessed what, when, and from where",
            ),
            (
                "Segregation of duties enforced",
                "Conflicting role detection and prevention",
            ),
        ],
    )?;

    let s6 = add_template_sprint(
        conn,
        tpl,
        "Security & Compliance",
        "Enterprise security and regulatory compliance",
        5,
    )?;
    add_template_sprint_sections(conn, s6, &[security])?;

    let s7 = add_template_sprint(
        conn,
        tpl,
        "Database & Data Layer",
        "Enterprise database architecture",
        6,
    )?;
    add_template_sprint_sections(conn, s7, &[database])?;

    let s8 = add_template_sprint(conn, tpl, "Testing & QA", "Enterprise testing strategy", 7)?;
    add_template_sprint_sections(conn, s8, &[testing])?;
    add_custom_sprint_section(
        conn,
        s8,
        "Enterprise Testing",
        "UAT, integration, and performance testing",
        "#10b981",
        &[
            (
                "User acceptance testing coordinated",
                "Business user testing with sign-off process",
            ),
            (
                "Integration testing with all systems",
                "End-to-end testing across all connected systems",
            ),
            (
                "Load testing at enterprise scale",
                "Concurrent user simulation at peak load levels",
            ),
            (
                "Data migration testing completed",
                "Legacy data validated against new system",
            ),
            (
                "Regression test suite maintained",
                "Automated regression for all critical business flows",
            ),
        ],
    )?;

    let s9 = add_template_sprint(
        conn,
        tpl,
        "Performance Optimization",
        "Scaling and performance tuning",
        8,
    )?;
    add_template_sprint_sections(conn, s9, &[performance])?;

    let s10 = add_template_sprint(
        conn,
        tpl,
        "CI/CD & Deployment",
        "Enterprise release management",
        9,
    )?;
    add_template_sprint_sections(conn, s10, &[cicd, docs])?;
    add_custom_sprint_section(
        conn,
        s10,
        "Release Management",
        "Enterprise deployment and change control",
        "#3b82f6",
        &[
            (
                "Change advisory board process defined",
                "CAB review, approval, and scheduling for releases",
            ),
            (
                "Blue-green or canary deployment configured",
                "Zero-downtime deployment with instant rollback",
            ),
            (
                "Feature flag system implemented",
                "Gradual rollout with user-segment targeting",
            ),
            (
                "Release communication plan created",
                "Stakeholder notifications, downtime windows, and FAQs",
            ),
            (
                "Post-deployment validation automated",
                "Smoke tests and health checks after each release",
            ),
        ],
    )?;

    let s11 = add_template_sprint(
        conn,
        tpl,
        "Monitoring & Operations",
        "Production monitoring and support",
        10,
    )?;
    add_template_sprint_sections(conn, s11, &[monitoring, docs])?;

    Ok(())
}
