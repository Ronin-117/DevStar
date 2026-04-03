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
    let quality = section_map.get("quality").copied().unwrap_or(0);
    let performance = section_map.get("performance").copied().unwrap_or(0);
    let monitoring = section_map.get("monitoring").copied().unwrap_or(0);

    let tpl = add_template(
        conn,
        "Tools & Library Development",
        "Frameworks, compilers, CLIs, and developer utilities",
        "#7c3aed",
    )?;

    let s1 = add_template_sprint(
        conn,
        tpl,
        "Planning & Design",
        "API design and architecture decisions",
        0,
    )?;
    add_template_sprint_sections(conn, s1, &[planning, quality])?;
    add_custom_sprint_section(
        conn,
        s1,
        "API & Interface Design",
        "Public API and developer experience",
        "#7c3aed",
        &[
            (
                "Public API surface defined",
                "Explicit public vs internal API boundary",
            ),
            (
                "API ergonomics reviewed",
                "Intuitive naming, fluent interfaces, and minimal boilerplate",
            ),
            (
                "Backward compatibility policy established",
                "Semver strategy and deprecation timeline",
            ),
            (
                "Error design documented",
                "Error types, error messages, and recovery guidance",
            ),
            (
                "Developer experience plan created",
                "Quick-start guide, examples, and migration guides",
            ),
        ],
    )?;

    let s2 = add_template_sprint(
        conn,
        tpl,
        "Core Implementation",
        "Primary functionality and algorithms",
        1,
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Core Engine",
        "Primary functionality implementation",
        "#6366f1",
        &[
            (
                "Core algorithms implemented",
                "Primary data structures and algorithms optimized",
            ),
            (
                "Plugin/extension system designed",
                "Plugin API with versioning and sandboxing",
            ),
            (
                "Configuration system built",
                "Config file parsing, env vars, and CLI flag overrides",
            ),
            (
                "Cross-platform abstraction layer",
                "OS-specific code isolated behind platform traits",
            ),
            (
                "Performance-critical paths optimized",
                "Hot paths profiled and optimized early",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "CLI Interface",
        "Command-line interface and UX",
        "#8b5cf6",
        &[
            (
                "CLI argument parsing configured",
                "clap, argparse, or similar with subcommands",
            ),
            (
                "Help text and man pages generated",
                "Comprehensive help with examples for all commands",
            ),
            (
                "Output formatting implemented",
                "Human-readable, JSON, and machine-parseable output",
            ),
            (
                "Progress indicators and status messages",
                "Spinners, progress bars, and colored output",
            ),
            (
                "Shell completion scripts generated",
                "Bash, zsh, fish, and PowerShell completions",
            ),
        ],
    )?;

    let s3 = add_template_sprint(
        conn,
        tpl,
        "Build System & Tooling",
        "Build configuration and developer tooling",
        2,
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Build System",
        "Compilation, packaging, and distribution",
        "#f59e0b",
        &[
            (
                "Build system configured",
                "CMake, Cargo, Make, or language-specific build tool",
            ),
            (
                "Cross-compilation support added",
                "Multiple target architectures and platforms",
            ),
            (
                "Feature flags and conditional compilation",
                "Optional features with minimal dependency tree",
            ),
            (
                "Static and dynamic library builds",
                "Both .a/.so and .dylib/.dll outputs",
            ),
            (
                "Build caching configured",
                "ccache, sccache, or build system native caching",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Developer Tooling",
        "Linting, formatting, and IDE support",
        "#10b981",
        &[
            (
                "Linter and formatter configured",
                "Language-specific tools with strict rules",
            ),
            (
                "IDE integration provided",
                "Language server, syntax highlighting, and snippets",
            ),
            (
                "Pre-commit hooks installed",
                "Auto-format, lint, and test before commit",
            ),
            (
                "Development container configured",
                "DevContainer or similar for reproducible dev env",
            ),
            (
                "Benchmark suite created",
                "Criterion, pytest-benchmark, or similar for performance tracking",
            ),
        ],
    )?;

    let s4 = add_template_sprint(
        conn,
        tpl,
        "Testing & Quality",
        "Comprehensive testing strategy for libraries",
        3,
    )?;
    add_template_sprint_sections(conn, s4, &[testing])?;
    add_custom_sprint_section(
        conn,
        s4,
        "Library Testing",
        "Unit, property-based, and integration tests",
        "#10b981",
        &[
            (
                "Property-based tests written",
                "QuickCheck, Hypothesis, or similar for edge cases",
            ),
            (
                "Doctest or example-based tests",
                "Documentation examples that compile and run as tests",
            ),
            (
                "Fuzz testing on parsers and input",
                "AFL, libFuzzer, or cargo-fuzz for input validation",
            ),
            (
                "Integration tests with real dependencies",
                "End-to-end tests with actual file system or network",
            ),
            (
                "Cross-platform test matrix",
                "Tests run on all supported OS and architecture combinations",
            ),
        ],
    )?;

    let s5 = add_template_sprint(
        conn,
        tpl,
        "Documentation",
        "API docs, guides, and examples",
        4,
    )?;
    add_template_sprint_sections(conn, s5, &[docs])?;
    add_custom_sprint_section(
        conn,
        s5,
        "Developer Documentation",
        "Comprehensive docs for library consumers",
        "#a855f7",
        &[
            (
                "API reference documentation generated",
                "rustdoc, Javadoc, Sphinx, or similar auto-generated",
            ),
            (
                "Getting started guide written",
                "Installation, first use, and common patterns",
            ),
            (
                "Tutorial series created",
                "Step-by-step tutorials from basic to advanced usage",
            ),
            (
                "Migration guides for version upgrades",
                "Breaking changes with automated migration scripts",
            ),
            (
                "Architecture decision records maintained",
                "Key design decisions with context and rationale",
            ),
        ],
    )?;

    let s6 = add_template_sprint(
        conn,
        tpl,
        "Package Publishing",
        "Registry, versioning, and distribution",
        5,
    )?;
    add_custom_sprint_section(
        conn,
        s6,
        "Package Management",
        "Publishing and version management",
        "#06b6d4",
        &[
            (
                "Package registry configured",
                "npm, crates.io, PyPI, or private registry",
            ),
            (
                "Version bump automation",
                "Conventional commits with automated version bumping",
            ),
            (
                "Changelog generation automated",
                "CHANGELOG.md from commit history with categories",
            ),
            (
                "Release notes template created",
                "User-facing release notes with breaking change warnings",
            ),
            (
                "Package signing configured",
                "GPG or registry-native package signature verification",
            ),
        ],
    )?;

    let s7 = add_template_sprint(
        conn,
        tpl,
        "Performance Optimization",
        "Benchmarking and optimization",
        6,
    )?;
    add_template_sprint_sections(conn, s7, &[performance])?;
    add_custom_sprint_section(
        conn,
        s7,
        "Library Performance",
        "Optimization techniques for libraries",
        "#f97316",
        &[
            (
                "Zero-cost abstractions verified",
                "No runtime overhead for convenience APIs",
            ),
            (
                "Memory allocation minimized",
                "Arena allocation, string interning, or smallvec",
            ),
            (
                "SIMD optimization applied",
                "Auto-vectorization or explicit SIMD for hot loops",
            ),
            (
                "Compile-time computation maximized",
                "Const fn, const generics, or metaprogramming",
            ),
            (
                "Benchmark regression detection",
                "CI fails if benchmarks degrade beyond threshold",
            ),
        ],
    )?;

    let s8 = add_template_sprint(
        conn,
        tpl,
        "Security Review",
        "Dependency audit and vulnerability assessment",
        7,
    )?;
    add_template_sprint_sections(conn, s8, &[security])?;

    let s9 = add_template_sprint(
        conn,
        tpl,
        "CI/CD & Automation",
        "Automated testing and publishing",
        8,
    )?;
    add_template_sprint_sections(conn, s9, &[cicd, docs])?;

    let s10 = add_template_sprint(
        conn,
        tpl,
        "Community & Maintenance",
        "Contributor onboarding and long-term maintenance",
        9,
    )?;
    add_custom_sprint_section(
        conn,
        s10,
        "Community Management",
        "Open-source community and contribution",
        "#ec4899",
        &[
            (
                "Contributing guide published",
                "Code style, PR process, and issue templates",
            ),
            (
                "Code of conduct established",
                "Community guidelines and enforcement process",
            ),
            (
                "Issue triage process defined",
                "Label system, priority levels, and response SLAs",
            ),
            (
                "Release cadence established",
                "Regular release schedule with LTS support policy",
            ),
            (
                "Community channels set up",
                "Discord, Matrix, or forum for user support and discussion",
            ),
        ],
    )?;

    Ok(())
}
