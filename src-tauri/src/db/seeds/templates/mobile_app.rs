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
    let _database = section_map.get("database").copied().unwrap_or(0);

    let _planning_setup = sprint_map.get("planning_setup").copied().unwrap_or(0);
    let _security_quality = sprint_map.get("security_quality").copied().unwrap_or(0);
    let _testing_qa = sprint_map.get("testing_qa").copied().unwrap_or(0);
    let _cicd_deploy = sprint_map.get("cicd_deploy").copied().unwrap_or(0);
    let _monitoring_ops = sprint_map.get("monitoring_ops").copied().unwrap_or(0);
    let _perf_sprint = sprint_map.get("performance").copied().unwrap_or(0);
    let _db_sprint = sprint_map.get("database").copied().unwrap_or(0);

    // === Mobile App Development ===
    let tpl = add_template(
        conn,
        "Mobile App Development",
        "React Native / Flutter / Swift / Kotlin — cross-platform and native mobile checklist",
        "#8b5cf6",
    )?;

    // Sprint 1: Planning & Setup (shared)
    let s1 = add_template_sprint(
        conn,
        tpl,
        "Planning & Setup",
        "Project kickoff and dev environment",
        0,
    )?;
    add_template_sprint_sections(conn, s1, &[planning, quality])?;

    // Sprint 2: Platform Setup & Architecture
    let s2 = add_template_sprint(
        conn,
        tpl,
        "Platform Setup & Architecture",
        "Native or cross-platform project initialization",
        1,
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Project Initialization",
        "Platform-specific setup and configuration",
        "#8b5cf6",
        &[
            (
                "Native or cross-platform framework chosen",
                "React Native, Flutter, Swift, or Kotlin with rationale",
            ),
            (
                "Project scaffolded with CLI",
                "Clean project structure with proper naming conventions",
            ),
            (
                "Build system configured",
                "Gradle/Xcode/Fastlane with build variants",
            ),
            (
                "Environment config separated",
                "Dev, staging, and prod config files or env vars",
            ),
            (
                "Code signing setup",
                "Development certificates and provisioning profiles",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "App Architecture",
        "Navigation, state, and layer separation",
        "#6366f1",
        &[
            (
                "Navigation structure defined",
                "Stack, tab, and drawer navigation with deep linking support",
            ),
            (
                "State management solution chosen",
                "Redux, Riverpod, Provider, or native state",
            ),
            (
                "MVVM or Clean Architecture applied",
                "Presentation, domain, and data layers separated",
            ),
            (
                "Dependency injection configured",
                "GetIt, Dagger, or native DI framework",
            ),
            (
                "App lifecycle handling implemented",
                "Foreground, background, and termination states",
            ),
        ],
    )?;

    // Sprint 3: UI Foundation
    let s3 = add_template_sprint(
        conn,
        tpl,
        "UI Foundation",
        "Design system, components, and responsive layouts",
        2,
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Design System",
        "Reusable UI components and theming",
        "#ec4899",
        &[
            (
                "Design tokens defined",
                "Colors, typography, spacing, and border radius variables",
            ),
            (
                "Core component library built",
                "Buttons, inputs, cards, modals, and lists",
            ),
            (
                "Dark mode support implemented",
                "System theme detection with light/dark themes",
            ),
            (
                "Responsive layout system configured",
                "Safe areas, notch handling, and screen size breakpoints",
            ),
            (
                "Animation library integrated",
                "Lottie, Reanimated, or native animation framework",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Screen Implementation",
        "Core app screens and user flows",
        "#f472b6",
        &[
            (
                "Onboarding flow built",
                "Welcome screens, permissions requests, and account setup",
            ),
            (
                "Home/dashboard screen implemented",
                "Primary content view with navigation",
            ),
            (
                "Detail screens created",
                "Item detail, profile, and settings screens",
            ),
            (
                "Form screens with validation",
                "Input forms with keyboard handling and validation",
            ),
            (
                "Loading and error states designed",
                "Skeleton screens, error views, and retry actions",
            ),
        ],
    )?;

    // Sprint 4: Data Layer & API Integration
    let s4 = add_template_sprint(
        conn,
        tpl,
        "Data Layer & API",
        "Network layer, local storage, and offline support",
        3,
    )?;
    add_custom_sprint_section(
        conn,
        s4,
        "Network Layer",
        "API client and data synchronization",
        "#06b6d4",
        &[
            (
                "HTTP client configured",
                "Dio, Axios, or native with interceptors and timeouts",
            ),
            (
                "API service layer created",
                "Typed API calls with request/response models",
            ),
            (
                "Error handling for network failures",
                "Retry logic, offline detection, and user feedback",
            ),
            (
                "Token refresh mechanism implemented",
                "Automatic token refresh on 401 responses",
            ),
            (
                "Request/response caching configured",
                "HTTP cache headers or custom caching strategy",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s4,
        "Local Storage & Offline",
        "Persistent data and offline-first support",
        "#14b8a6",
        &[
            (
                "Local database configured",
                "SQLite, Realm, or Hive for persistent storage",
            ),
            (
                "Offline-first sync strategy defined",
                "Queue-based sync with conflict resolution",
            ),
            (
                "Secure storage for tokens",
                "Keychain (iOS) or Keystore (Android) for sensitive data",
            ),
            (
                "Cache invalidation rules defined",
                "TTL-based or event-driven cache clearing",
            ),
            (
                "Offline UI states implemented",
                "Cached content display with sync status indicators",
            ),
        ],
    )?;

    // Sprint 5: Device Features & Permissions
    let s5 = add_template_sprint(
        conn,
        tpl,
        "Device Features",
        "Native device capabilities and permissions",
        4,
    )?;
    add_custom_sprint_section(
        conn,
        s5,
        "Device Permissions",
        "Runtime permissions and user consent",
        "#f59e0b",
        &[
            (
                "Camera permission flow implemented",
                "Request, rationale, and settings redirect",
            ),
            (
                "Location permission configured",
                "Coarse/fine location with background permission handling",
            ),
            (
                "Notification permission requested",
                "Push notification consent with settings link",
            ),
            (
                "Storage/photo library access configured",
                "Read/write permissions with scoped storage",
            ),
            (
                "Permission denied UI created",
                "Graceful degradation when permissions are denied",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s5,
        "Native Features",
        "Platform-specific capabilities",
        "#f97316",
        &[
            (
                "Push notifications configured",
                "FCM (Android) and APNs (iOS) with topic/segment support",
            ),
            (
                "Background tasks implemented",
                "Periodic sync, location updates, or data processing",
            ),
            (
                "Deep linking configured",
                "Universal links (iOS) and App Links (Android)",
            ),
            (
                "Biometric authentication added",
                "Face ID, Touch ID, or fingerprint with fallback",
            ),
            (
                "In-app purchases or subscriptions",
                "StoreKit (iOS) or Billing Library (Android) integration",
            ),
        ],
    )?;

    // Sprint 6: Security Hardening (shared)
    let s6 = add_template_sprint(
        conn,
        tpl,
        "Security Hardening",
        "Mobile-specific security measures",
        5,
    )?;
    add_template_sprint_sections(conn, s6, &[security])?;
    add_custom_sprint_section(
        conn,
        s6,
        "Mobile Security",
        "App-level security and data protection",
        "#ef4444",
        &[
            (
                "Certificate pinning configured",
                "Prevent MITM attacks with pinned certificates",
            ),
            (
                "App integrity checks added",
                "Root/jailbreak detection and tamper detection",
            ),
            (
                "Data encryption at rest",
                "Encrypted database and file storage",
            ),
            (
                "Secure communication enforced",
                "TLS 1.2+ with certificate validation",
            ),
            (
                "Sensitive data cleared from memory",
                "No sensitive data in logs, screenshots, or clipboard",
            ),
        ],
    )?;

    // Sprint 7: Testing & QA (shared)
    let s7 = add_template_sprint(
        conn,
        tpl,
        "Testing & QA",
        "Mobile-specific testing strategy",
        6,
    )?;
    add_template_sprint_sections(conn, s7, &[testing])?;
    add_custom_sprint_section(
        conn,
        s7,
        "Mobile Testing",
        "Device-specific and UI testing",
        "#10b981",
        &[
            (
                "Widget/unit tests written",
                "Individual component and business logic tests",
            ),
            (
                "Integration tests for user flows",
                "Multi-screen navigation and state transitions",
            ),
            (
                "Device compatibility tested",
                "Multiple screen sizes, OS versions, and manufacturers",
            ),
            (
                "Network condition testing",
                "Slow network, offline, and reconnect scenarios",
            ),
            (
                "Memory leak detection configured",
                "LeakCanary (Android) or Instruments (iOS)",
            ),
        ],
    )?;

    // Sprint 8: Performance Optimization
    let s8 = add_template_sprint(
        conn,
        tpl,
        "Performance Optimization",
        "App speed, memory, and battery efficiency",
        7,
    )?;
    add_template_sprint_sections(conn, s8, &[performance])?;
    add_custom_sprint_section(
        conn,
        s8,
        "Mobile Performance",
        "App-specific optimization techniques",
        "#f97316",
        &[
            (
                "List rendering optimized",
                "Virtualized lists with recycled views/cells",
            ),
            (
                "Image loading optimized",
                "Cached, resized, and lazy-loaded images",
            ),
            (
                "App startup time measured and optimized",
                "Cold start under 2 seconds target",
            ),
            (
                "Battery usage profiled",
                "Background task optimization and wake lock management",
            ),
            (
                "Memory usage profiled",
                "Heap analysis, image cache limits, and leak fixes",
            ),
        ],
    )?;

    // Sprint 9: App Store Preparation
    let s9 = add_template_sprint(
        conn,
        tpl,
        "App Store Preparation",
        "Store listing, assets, and submission",
        8,
    )?;
    add_custom_sprint_section(
        conn,
        s9,
        "Store Assets & Metadata",
        "App store listing preparation",
        "#8b5cf6",
        &[
            (
                "App icons created for all sizes",
                "iOS app icons and Android adaptive icons",
            ),
            (
                "Screenshots captured for all devices",
                "Phone, tablet, and landscape screenshots",
            ),
            (
                "App store listing written",
                "Title, description, keywords, and category selected",
            ),
            (
                "Privacy policy URL configured",
                "GDPR/CCPA compliant privacy policy hosted",
            ),
            (
                "App rating and review strategy defined",
                "In-app review prompts with timing logic",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s9,
        "Build & Distribution",
        "Release builds and distribution channels",
        "#6366f1",
        &[
            (
                "Release build configured",
                "ProGuard/R8, code shrinking, and resource optimization",
            ),
            (
                "App bundle/APK signing completed",
                "Release keystore with proper key management",
            ),
            (
                "TestFlight or internal testing set up",
                "Beta distribution to testers before public release",
            ),
            (
                "Crash reporting integrated",
                "Firebase Crashlytics or Sentry for production errors",
            ),
            (
                "Analytics configured",
                "User behavior tracking with privacy-compliant setup",
            ),
        ],
    )?;

    // Sprint 10: CI/CD & Deployment (shared)
    let s10 = add_template_sprint(
        conn,
        tpl,
        "CI/CD & Deployment",
        "Automated build and release pipeline",
        9,
    )?;
    add_template_sprint_sections(conn, s10, &[cicd, docs])?;

    // Sprint 11: Monitoring & Operations (shared)
    let s11 = add_template_sprint(
        conn,
        tpl,
        "Monitoring & Operations",
        "Production monitoring and incident response",
        10,
    )?;
    add_template_sprint_sections(conn, s11, &[monitoring, docs])?;

    Ok(())
}
