# Seed Data

## Overview

DevStar ships with comprehensive, senior-dev-level seed data for 12 software project types. The data is organized into shared building blocks (sections and sprints) that are composed into templates.

## Shared Sections (10)

Reusable checklist blocks used across multiple templates. Each has 10 specific, actionable items.

| # | Name | Color | Description |
|---|------|-------|-------------|
| 1 | Project Planning & Scoping | `#6366f1` | Requirements gathering, architecture decisions, project setup |
| 2 | Security Fundamentals | `#ef4444` | Core security practices applied across all project types |
| 3 | Testing & QA | `#10b981` | Comprehensive testing strategy and quality assurance |
| 4 | CI/CD Pipeline | `#f59e0b` | Automated build, test, and deployment pipeline |
| 5 | Documentation | `#8b5cf6` | Technical and user documentation for the project |
| 6 | Code Quality & Review | `#06b6d4` | Code standards, review processes, maintainability |
| 7 | Performance Baseline | `#f97316` | Performance profiling, optimization, monitoring setup |
| 8 | Monitoring & Observability | `#ec4899` | Logging, metrics, tracing, incident response |
| 9 | Database & Data Management | `#14b8a6` | Schema design, migrations, backups, data integrity |
| 10 | Accessibility & UX | `#a855f7` | WCAG compliance, usability testing, inclusive design |

## Shared Sprints (8)

Reusable sprint templates that compose shared sections.

| # | Name | Sections Included |
|---|------|-------------------|
| 1 | Planning & Setup | Project Planning & Scoping, Code Quality & Review |
| 2 | Security & Quality | Security Fundamentals, Code Quality & Review |
| 3 | Testing & QA | Testing & QA |
| 4 | CI/CD & Deployment | CI/CD Pipeline, Documentation |
| 5 | Monitoring & Operations | Monitoring & Observability, Documentation |
| 6 | Performance & Optimization | Performance Baseline |
| 7 | Database & Data Management | Database & Data Management |
| 8 | Accessibility & UX | Accessibility & UX |

## Templates (12)

### 1. Full-Stack Web App (`#3b82f6`)
12 sprints covering frontend, backend, auth, security, testing, performance, accessibility, CI/CD, and launch.

### 2. Mobile App Development (`#8b5cf6`)
11 sprints covering platform setup, UI foundation, data layer, device features, security, testing, performance, and app store preparation.

### 3. Desktop Application (`#f59e0b`)
10 sprints covering architecture, core features, data & storage, security, auto-update system, testing, performance, and CI/CD.

### 4. Game Development (`#ef4444`)
10 sprints covering planning, core game loop, physics & collision, audio system, asset pipeline, security, testing, performance, and store submission.

### 5. Embedded Systems & IoT (`#14b8a6`)
10 sprints covering hardware setup, firmware foundation, sensor integration, communication protocols, power management, security, testing, safety certification, CI/CD, and monitoring.

### 6. API & Backend Development (`#06b6d4`)
10 sprints covering planning, API design, core implementation, auth & middleware, database & caching, security, testing, performance, CI/CD, and monitoring.

### 7. Data Science & AI (`#a855f7`)
10 sprints covering planning, data collection, EDA & feature engineering, model development, model serving, experiment tracking, security, monitoring & drift, testing, and CI/CD.

### 8. Cloud & Infrastructure (`#3b82f6`)
11 sprints covering planning, IaC, CI/CD, security, service mesh, auto-scaling, monitoring, backup & DR, cost optimization, testing, and documentation.

### 9. Systems Programming (`#64748b`)
10 sprints covering planning, core foundation, concurrency & scheduling, device drivers, security, testing, performance, debugging, CI/CD, and documentation.

### 10. Enterprise Systems (`#475569`)
11 sprints covering planning, domain modeling, core modules, integration layer, RBAC, security, database, testing, performance, CI/CD, and monitoring.

### 11. Security Software (`#dc2626`)
10 sprints covering planning, cryptographic foundation, core security engine, network security, vulnerability management, hardening, compliance, testing, CI/CD, and monitoring.

### 12. Tools & Libraries (`#7c3aed`)
10 sprints covering planning, core implementation, build system, testing, documentation, package publishing, performance, security, CI/CD, and community management.

## Link vs Copy Behavior

- **Linked items**: Reference the shared source. Changes to the shared section/sprint are reflected in all templates using them.
- **Copied items**: Independent copies. Changes to the source do not affect copies, and vice versa.
- **Project-specific items**: Created directly in a project sprint. Always independent.

## Adding Custom Seed Data

To add new templates or sections, modify the files in `src-tauri/src/db/seeds/`:

1. Add shared sections to `shared_sections.rs`
2. Add shared sprints to `shared_sprints.rs`
3. Add a new template file in `seeds/templates/`
4. Register it in `seeds/templates/mod.rs`
5. Call it from `seeds/mod.rs::seed_all()`
