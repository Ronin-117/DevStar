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
    let monitoring = section_map.get("monitoring").copied().unwrap_or(0);
    let _quality = section_map.get("quality").copied().unwrap_or(0);
    let _performance = section_map.get("performance").copied().unwrap_or(0);
    let _docs = section_map.get("docs").copied().unwrap_or(0);
    let _database = section_map.get("database").copied().unwrap_or(0);


    let tpl = add_template(
        conn,
        "Cloud & Infrastructure Development",
        "Cloud-native services, DevOps tooling, and infrastructure automation",
        "#3b82f6",
    )?;

    let s1 = add_template_sprint(
        conn,
        tpl,
        "Planning & Architecture",
        "Cloud strategy and infrastructure design",
        0,
    )?;
    add_template_sprint_sections(conn, s1, &[planning])?;
    add_custom_sprint_section(
        conn,
        s1,
        "Cloud Architecture",
        "Multi-service design and deployment strategy",
        "#3b82f6",
        &[
            (
                "Cloud provider selected",
                "AWS, GCP, Azure, or multi-cloud with rationale documented",
            ),
            (
                "Service topology designed",
                "VPC, subnets, security groups, and routing topology",
            ),
            (
                "Compute strategy defined",
                "Containers, serverless, VMs, or managed services",
            ),
            (
                "Networking architecture planned",
                "Load balancers, API gateways, and service mesh",
            ),
            (
                "Disaster recovery strategy defined",
                "Multi-region failover with RTO/RPO targets",
            ),
        ],
    )?;

    let s2 = add_template_sprint(
        conn,
        tpl,
        "Infrastructure as Code",
        "Terraform, Pulumi, or CloudFormation",
        1,
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "IaC Foundation",
        "Infrastructure definition and state management",
        "#06b6d4",
        &[
            (
                "IaC tool selected and configured",
                "Terraform, Pulumi, CDK, or CloudFormation",
            ),
            (
                "State management configured",
                "Remote state with locking and versioning",
            ),
            (
                "Module structure designed",
                "Reusable modules for common infrastructure patterns",
            ),
            (
                "Environment separation implemented",
                "Dev, staging, prod with isolated resources",
            ),
            (
                "Secrets management integrated",
                "Vault, AWS Secrets Manager, or similar",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Container Infrastructure",
        "Kubernetes, ECS, or container orchestration",
        "#14b8a6",
        &[
            (
                "Container registry configured",
                "ECR, GCR, or ACR with image scanning",
            ),
            (
                "Orchestration platform provisioned",
                "EKS, GKE, AKS, or ECS cluster setup",
            ),
            (
                "Ingress controller configured",
                "Nginx, Traefik, or cloud-native ingress",
            ),
            (
                "Storage classes defined",
                "Persistent volumes with appropriate storage backends",
            ),
            (
                "Resource quotas and limits set",
                "CPU, memory, and storage quotas per namespace",
            ),
        ],
    )?;

    let s3 = add_template_sprint(
        conn,
        tpl,
        "CI/CD Pipeline",
        "Automated build, test, and deploy",
        2,
    )?;
    add_template_sprint_sections(conn, s3, &[cicd])?;

    let s4 = add_template_sprint(
        conn,
        tpl,
        "Security & Compliance",
        "Cloud security posture and compliance",
        3,
    )?;
    add_template_sprint_sections(conn, s4, &[security])?;
    add_custom_sprint_section(
        conn,
        s4,
        "Cloud Security",
        "Infrastructure-level security measures",
        "#ef4444",
        &[
            (
                "IAM policies with least privilege",
                "Role-based access with scoped permissions",
            ),
            (
                "Network security groups configured",
                "Ingress/egress rules with deny-by-default",
            ),
            (
                "Encryption at rest enabled",
                "KMS-managed keys for all storage services",
            ),
            (
                "Encryption in transit enforced",
                "TLS 1.2+ for all service communication",
            ),
            (
                "Cloud security posture management",
                "CSPM tool for continuous compliance monitoring",
            ),
        ],
    )?;

    let s5 = add_template_sprint(
        conn,
        tpl,
        "Service Mesh & Networking",
        "Inter-service communication and traffic management",
        4,
    )?;
    add_custom_sprint_section(
        conn,
        s5,
        "Service Mesh",
        "Istio, Linkerd, or cloud-native mesh",
        "#8b5cf6",
        &[
            (
                "Service mesh deployed",
                "Istio, Linkerd, or cloud-native service mesh",
            ),
            (
                "mTLS between services enabled",
                "Automatic certificate rotation and verification",
            ),
            (
                "Traffic management configured",
                "Canary, blue-green, and A/B testing routing",
            ),
            (
                "Circuit breakers implemented",
                "Retry policies, timeouts, and fallback handlers",
            ),
            (
                "Service discovery configured",
                "DNS-based or registry-based service resolution",
            ),
        ],
    )?;

    let s6 = add_template_sprint(
        conn,
        tpl,
        "Auto-Scaling & Load Balancing",
        "Elastic scaling and traffic distribution",
        5,
    )?;
    add_custom_sprint_section(
        conn,
        s6,
        "Scaling Strategy",
        "Horizontal and vertical scaling",
        "#f59e0b",
        &[
            (
                "HPA configured for workloads",
                "CPU, memory, or custom metric-based scaling",
            ),
            (
                "Cluster autoscaler enabled",
                "Node pool scaling based on pending pods",
            ),
            (
                "Load balancer health checks configured",
                "Liveness and readiness probes with thresholds",
            ),
            (
                "Connection draining implemented",
                "Graceful shutdown with in-flight request completion",
            ),
            (
                "Scaling limits defined",
                "Min/max replicas and scale-up/down cooldown periods",
            ),
        ],
    )?;

    let s7 = add_template_sprint(
        conn,
        tpl,
        "Monitoring & Observability",
        "Full-stack observability",
        6,
    )?;
    add_template_sprint_sections(conn, s7, &[monitoring])?;
    add_custom_sprint_section(
        conn,
        s7,
        "Infrastructure Monitoring",
        "Resource and cluster-level monitoring",
        "#ec4899",
        &[
            (
                "Node and cluster metrics collected",
                "CPU, memory, disk, and network per node",
            ),
            (
                "Pod and container metrics tracked",
                "Per-container resource usage and restart counts",
            ),
            (
                "Custom business metrics defined",
                "Application-specific KPIs exported to monitoring",
            ),
            (
                "Log aggregation pipeline built",
                "Fluentd, Fluent Bit, or similar to central store",
            ),
            (
                "Distributed tracing enabled",
                "OpenTelemetry with Jaeger, Tempo, or X-Ray",
            ),
        ],
    )?;

    let s8 = add_template_sprint(
        conn,
        tpl,
        "Backup & Disaster Recovery",
        "Data protection and recovery procedures",
        7,
    )?;
    add_custom_sprint_section(
        conn,
        s8,
        "Backup Strategy",
        "Automated backups and recovery testing",
        "#14b8a6",
        &[
            (
                "Database backups automated",
                "Daily full backups with hourly incrementals",
            ),
            (
                "Infrastructure state backed up",
                "Terraform state, K8s manifests, and configs",
            ),
            (
                "Cross-region replication configured",
                "Critical data replicated to secondary region",
            ),
            (
                "Recovery procedures documented",
                "Step-by-step runbook for each failure scenario",
            ),
            (
                "Disaster recovery drill conducted",
                "Full failover test with measured RTO/RPO",
            ),
        ],
    )?;

    let s9 = add_template_sprint(
        conn,
        tpl,
        "Cost Optimization",
        "Resource efficiency and cost management",
        8,
    )?;
    add_custom_sprint_section(
        conn,
        s9,
        "Cost Management",
        "Cloud cost monitoring and optimization",
        "#f97316",
        &[
            (
                "Cost allocation tags applied",
                "All resources tagged with team, project, and env",
            ),
            (
                "Budget alerts configured",
                "Threshold-based alerts at 50%, 80%, and 100% of budget",
            ),
            (
                "Unused resources identified and removed",
                "Idle instances, unattached volumes, old snapshots",
            ),
            (
                "Reserved instances or committed use evaluated",
                "Cost savings analysis for predictable workloads",
            ),
            (
                "Right-sizing completed",
                "Instance types matched to actual resource utilization",
            ),
        ],
    )?;

    let s10 = add_template_sprint(
        conn,
        tpl,
        "Testing & QA",
        "Infrastructure testing and validation",
        9,
    )?;
    add_template_sprint_sections(conn, s10, &[testing])?;

    let s11 = add_template_sprint(
        conn,
        tpl,
        "Documentation & Handoff",
        "Runbooks, architecture docs, and team training",
        10,
    )?;
    add_template_sprint_sections(conn, s11, &[docs])?;

    Ok(())
}
