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
    let _performance = section_map.get("performance").copied().unwrap_or(0);
    let monitoring = section_map.get("monitoring").copied().unwrap_or(0);

    let tpl = add_template(
        conn,
        "Security Software Development",
        "Encryption tools, firewalls, and cybersecurity suites",
        "#dc2626",
    )?;

    let s1 = add_template_sprint(
        conn,
        tpl,
        "Planning & Threat Modeling",
        "Security requirements and threat analysis",
        0,
    )?;
    add_template_sprint_sections(conn, s1, &[planning])?;
    add_custom_sprint_section(
        conn,
        s1,
        "Threat Modeling",
        "Security architecture and threat analysis",
        "#dc2626",
        &[
            (
                "Threat model created",
                "STRIDE or DREAD analysis for all system components",
            ),
            (
                "Attack surface mapped",
                "All entry points, trust boundaries, and data flows documented",
            ),
            (
                "Security requirements defined",
                "Confidentiality, integrity, and availability requirements",
            ),
            (
                "Compliance framework identified",
                "NIST, ISO 27001, SOC 2, or industry-specific standards",
            ),
            (
                "Security team and roles assigned",
                "Security champion, pen tester, and incident responder",
            ),
        ],
    )?;

    let s2 = add_template_sprint(
        conn,
        tpl,
        "Cryptographic Foundation",
        "Encryption, hashing, and key management",
        1,
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Cryptographic Implementation",
        "Crypto algorithms and protocols",
        "#ef4444",
        &[
            (
                "Cryptographic library selected",
                "libsodium, OpenSSL, BoringSSL, or platform-native crypto",
            ),
            (
                "Encryption at rest implemented",
                "AES-256-GCM or ChaCha20-Poly1305 for data encryption",
            ),
            (
                "Encryption in transit enforced",
                "TLS 1.3 with strong cipher suites and cert pinning",
            ),
            (
                "Hashing algorithms configured",
                "SHA-256/512 for integrity, Argon2 for password hashing",
            ),
            (
                "Digital signatures implemented",
                "Ed25519 or RSA for message and file signing",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Key Management",
        "Key generation, storage, and rotation",
        "#f97316",
        &[
            (
                "Key generation with secure RNG",
                "CSPRNG-based key generation with sufficient entropy",
            ),
            (
                "Key storage secured",
                "HSM, KMS, or secure enclave for key material",
            ),
            (
                "Key rotation policy implemented",
                "Automated key rotation with zero-downtime transition",
            ),
            (
                "Key revocation mechanism designed",
                "CRL, OCSP, or custom revocation list",
            ),
            (
                "Key escrow and recovery defined",
                "Secure key recovery process for organizational continuity",
            ),
        ],
    )?;

    let s3 = add_template_sprint(
        conn,
        tpl,
        "Core Security Engine",
        "Detection, analysis, and response engine",
        2,
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Detection Engine",
        "Threat detection and analysis",
        "#06b6d4",
        &[
            (
                "Signature-based detection implemented",
                "Pattern matching against known threat signatures",
            ),
            (
                "Anomaly detection configured",
                "Statistical or ML-based anomaly detection",
            ),
            (
                "Behavioral analysis engine built",
                "Baseline behavior profiling with deviation alerts",
            ),
            (
                "Log analysis pipeline created",
                "Centralized log ingestion with correlation rules",
            ),
            (
                "Threat intelligence feed integrated",
                "External threat feeds with IOC matching",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Response Engine",
        "Automated and manual incident response",
        "#14b8a6",
        &[
            (
                "Automated response actions defined",
                "Block IP, quarantine host, or disable account",
            ),
            (
                "Incident workflow engine implemented",
                "Triage, investigation, containment, and recovery",
            ),
            (
                "Forensic data collection automated",
                "Memory dumps, disk images, and network captures",
            ),
            (
                "Playbook automation configured",
                "SOAR-style automated response playbooks",
            ),
            (
                "Escalation procedures defined",
                "Tiered escalation with SLA-based response times",
            ),
        ],
    )?;

    let s4 = add_template_sprint(
        conn,
        tpl,
        "Network Security",
        "Firewall, IDS/IPS, and network monitoring",
        3,
    )?;
    add_custom_sprint_section(
        conn,
        s4,
        "Network Defense",
        "Firewall and intrusion detection",
        "#8b5cf6",
        &[
            (
                "Firewall rules engine implemented",
                "Stateful packet inspection with rule management",
            ),
            (
                "IDS/IPS configured",
                "Network and host-based intrusion detection/prevention",
            ),
            (
                "Network segmentation designed",
                "VLANs, micro-segmentation, and zero-trust zones",
            ),
            (
                "DNS security implemented",
                "DNS filtering, DNSSEC, and DNS tunneling detection",
            ),
            (
                "Network traffic analysis enabled",
                "NetFlow, pcap analysis, and protocol anomaly detection",
            ),
        ],
    )?;

    let s5 = add_template_sprint(
        conn,
        tpl,
        "Vulnerability Management",
        "Scanning, assessment, and remediation",
        4,
    )?;
    add_custom_sprint_section(
        conn,
        s5,
        "Vulnerability Scanning",
        "Automated vulnerability detection",
        "#f59e0b",
        &[
            (
                "Vulnerability scanner integrated",
                "Nessus, OpenVAS, or custom scanning engine",
            ),
            (
                "Dependency scanning automated",
                "SCA tools for third-party library vulnerabilities",
            ),
            (
                "Container image scanning configured",
                "Trivy, Clair, or similar for container vulnerabilities",
            ),
            (
                "Web application scanning enabled",
                "DAST scanning for OWASP Top 10 vulnerabilities",
            ),
            (
                "Vulnerability database maintained",
                "CVE tracking with severity scoring and patch status",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s5,
        "Penetration Testing",
        "Manual and automated security testing",
        "#ef4444",
        &[
            (
                "Penetration testing scope defined",
                "In-scope systems, rules of engagement, and timeline",
            ),
            (
                "Automated exploit testing configured",
                "Metasploit, Burp Suite, or custom exploit framework",
            ),
            (
                "Social engineering testing planned",
                "Phishing simulations and physical security tests",
            ),
            (
                "Red team exercises conducted",
                "Full-scope adversarial simulation with blue team response",
            ),
            (
                "Remediation tracking implemented",
                "Vulnerability lifecycle from discovery to verification",
            ),
        ],
    )?;

    let s6 = add_template_sprint(
        conn,
        tpl,
        "Security Hardening",
        "System and application hardening",
        5,
    )?;
    add_template_sprint_sections(conn, s6, &[security])?;

    let s7 = add_template_sprint(
        conn,
        tpl,
        "Compliance & Audit",
        "Regulatory compliance and audit trails",
        6,
    )?;
    add_custom_sprint_section(
        conn,
        s7,
        "Compliance Framework",
        "Regulatory compliance and certification",
        "#475569",
        &[
            (
                "Compliance checklist created",
                "NIST CSF, ISO 27001, PCI DSS, or HIPAA requirements",
            ),
            (
                "Audit logging implemented",
                "Immutable audit trail with tamper detection",
            ),
            (
                "Evidence collection automated",
                "Automated evidence gathering for compliance audits",
            ),
            (
                "Policy management system built",
                "Security policy creation, distribution, and acknowledgment",
            ),
            (
                "Compliance reporting dashboard",
                "Real-time compliance status with gap analysis",
            ),
        ],
    )?;

    let s8 = add_template_sprint(
        conn,
        tpl,
        "Testing & Validation",
        "Security testing and validation",
        7,
    )?;
    add_template_sprint_sections(conn, s8, &[testing])?;
    add_custom_sprint_section(
        conn,
        s8,
        "Security Testing",
        "Specialized security validation",
        "#10b981",
        &[
            (
                "Fuzz testing on all input parsers",
                "AFL, libFuzzer, or custom fuzzing harnesses",
            ),
            (
                "Cryptographic implementation verified",
                "Known-answer tests and side-channel analysis",
            ),
            (
                "Memory safety validated",
                "ASan, MSan, and TSan for memory error detection",
            ),
            (
                "Formal verification for critical components",
                "Model checking or theorem proving for crypto",
            ),
            (
                "Third-party security audit completed",
                "Independent security firm assessment and report",
            ),
        ],
    )?;

    let s9 = add_template_sprint(
        conn,
        tpl,
        "CI/CD & Deployment",
        "Secure build and deployment pipeline",
        8,
    )?;
    add_template_sprint_sections(conn, s9, &[cicd, docs])?;
    add_custom_sprint_section(
        conn,
        s9,
        "Secure Build Pipeline",
        "Supply chain security and build integrity",
        "#dc2626",
        &[
            (
                "Reproducible builds configured",
                "Deterministic builds with verification",
            ),
            (
                "SBOM generated for all releases",
                "Software Bill of Materials with dependency tracking",
            ),
            (
                "Code signing for all artifacts",
                "Binary signing with certificate verification",
            ),
            (
                "Supply chain security verified",
                "SLSA or similar supply chain level assessment",
            ),
            (
                "Build environment hardened",
                "Ephemeral build agents with minimal dependencies",
            ),
        ],
    )?;

    let s10 = add_template_sprint(
        conn,
        tpl,
        "Monitoring & Operations",
        "Security monitoring and incident response",
        9,
    )?;
    add_template_sprint_sections(conn, s10, &[monitoring, docs])?;

    Ok(())
}
