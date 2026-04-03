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
    let database = section_map.get("database").copied().unwrap_or(0);

    let tpl = add_template(
        conn,
        "Data Science & AI Development",
        "Machine learning, analytics, and data pipeline development",
        "#a855f7",
    )?;

    let s1 = add_template_sprint(
        conn,
        tpl,
        "Planning & Setup",
        "Project scope and environment",
        0,
    )?;
    add_template_sprint_sections(conn, s1, &[planning, quality])?;

    let s2 = add_template_sprint(
        conn,
        tpl,
        "Data Collection & Ingestion",
        "Data sources, pipelines, and storage",
        1,
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Data Sources",
        "Identification and access to data",
        "#a855f7",
        &[
            (
                "Data sources identified and documented",
                "Internal databases, APIs, third-party datasets",
            ),
            (
                "Data access credentials secured",
                "Service accounts, API keys, and OAuth tokens stored safely",
            ),
            (
                "Data ingestion pipeline designed",
                "Batch or streaming with error handling and retries",
            ),
            (
                "Data quality checks implemented",
                "Schema validation, null checks, and anomaly detection",
            ),
            (
                "Data catalog established",
                "Metadata registry with lineage tracking",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Data Storage",
        "Data lake, warehouse, and versioning",
        "#8b5cf6",
        &[
            (
                "Raw data storage configured",
                "Data lake with partitioned storage (S3, GCS, or HDFS)",
            ),
            (
                "Processed data warehouse set up",
                "Columnar storage (Parquet, Delta Lake) with schema",
            ),
            (
                "Data versioning implemented",
                "DVC, LakeFS, or similar for dataset versioning",
            ),
            (
                "Data transformation pipeline built",
                "dbt, Spark, or Pandas transformations with testing",
            ),
            (
                "Backup and disaster recovery configured",
                "Cross-region replication and point-in-time recovery",
            ),
        ],
    )?;

    let s3 = add_template_sprint(
        conn,
        tpl,
        "Data Cleaning & Exploration",
        "EDA, preprocessing, and feature engineering",
        2,
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Exploratory Data Analysis",
        "Understanding data distributions and patterns",
        "#ec4899",
        &[
            (
                "Statistical summary computed",
                "Mean, median, std, quartiles for all features",
            ),
            (
                "Missing data analysis completed",
                "Missingness patterns identified and handling strategy chosen",
            ),
            (
                "Outlier detection performed",
                "IQR, Z-score, or isolation forest with documentation",
            ),
            (
                "Correlation analysis completed",
                "Feature-feature and feature-target correlations mapped",
            ),
            (
                "Visualization dashboard created",
                "Distribution plots, scatter matrices, and time series",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Feature Engineering",
        "Feature creation, selection, and transformation",
        "#f472b6",
        &[
            (
                "Feature extraction pipeline built",
                "Domain-specific features from raw data",
            ),
            (
                "Feature scaling and normalization applied",
                "StandardScaler, MinMax, or robust scaling",
            ),
            (
                "Categorical encoding strategy chosen",
                "One-hot, target, or embedding encoding",
            ),
            (
                "Feature selection performed",
                "Mutual information, RFE, or SHAP-based selection",
            ),
            (
                "Feature store configured",
                "Centralized feature registry with versioning",
            ),
        ],
    )?;

    let s4 = add_template_sprint(
        conn,
        tpl,
        "Model Development",
        "Algorithm selection, training, and validation",
        3,
    )?;
    add_custom_sprint_section(
        conn,
        s4,
        "Model Training",
        "Algorithm implementation and hyperparameter tuning",
        "#6366f1",
        &[
            (
                "Baseline model established",
                "Simple model (linear, random) for performance comparison",
            ),
            (
                "Multiple algorithms evaluated",
                "At least 3 different model families tested",
            ),
            (
                "Cross-validation strategy defined",
                "K-fold, time-series split, or stratified split",
            ),
            (
                "Hyperparameter optimization completed",
                "Grid search, random search, or Bayesian optimization",
            ),
            (
                "Ensemble methods explored",
                "Bagging, boosting, or stacking if beneficial",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s4,
        "Model Validation",
        "Evaluation metrics and bias analysis",
        "#10b981",
        &[
            (
                "Evaluation metrics defined",
                "Accuracy, precision, recall, F1, ROC-AUC, or domain-specific",
            ),
            (
                "Train/validation/test split enforced",
                "No data leakage between splits",
            ),
            (
                "Bias and fairness analysis performed",
                "Demographic parity, equalized odds across groups",
            ),
            (
                "Error analysis completed",
                "Confusion matrix, misclassification patterns documented",
            ),
            (
                "Model calibration verified",
                "Probability outputs calibrated with Platt or isotonic",
            ),
        ],
    )?;

    let s5 = add_template_sprint(
        conn,
        tpl,
        "Model Serving & Deployment",
        "API, batch inference, and monitoring",
        4,
    )?;
    add_custom_sprint_section(
        conn,
        s5,
        "Model Serving",
        "Production inference infrastructure",
        "#06b6d4",
        &[
            (
                "Model serialization format chosen",
                "ONNX, PMML, or framework-specific format",
            ),
            (
                "Inference API implemented",
                "REST or gRPC endpoint with request validation",
            ),
            (
                "Batch inference pipeline configured",
                "Scheduled or event-triggered batch predictions",
            ),
            (
                "Model registry established",
                "Versioned model storage with staging/production tags",
            ),
            (
                "A/B testing framework set up",
                "Champion/challenger deployment with traffic splitting",
            ),
        ],
    )?;

    let s6 = add_template_sprint(
        conn,
        tpl,
        "Experiment Tracking",
        "Reproducibility and experiment management",
        5,
    )?;
    add_custom_sprint_section(
        conn,
        s6,
        "Experiment Management",
        "Tracking, logging, and reproducibility",
        "#f59e0b",
        &[
            (
                "Experiment tracking tool configured",
                "MLflow, Weights & Biases, or similar",
            ),
            (
                "All experiments logged with parameters",
                "Hyperparameters, data versions, and metrics tracked",
            ),
            (
                "Reproducibility ensured",
                "Fixed random seeds, environment pinning, and data snapshots",
            ),
            (
                "Model cards created",
                "Documentation for each model with use cases and limitations",
            ),
            (
                "Experiment comparison dashboard built",
                "Side-by-side metric comparison across runs",
            ),
        ],
    )?;

    let s7 = add_template_sprint(
        conn,
        tpl,
        "Security & Compliance",
        "Data privacy and model security",
        6,
    )?;
    add_template_sprint_sections(conn, s7, &[security])?;
    add_custom_sprint_section(
        conn,
        s7,
        "ML Security",
        "Model-specific security measures",
        "#ef4444",
        &[
            (
                "Adversarial attack testing performed",
                "FGSM, PGD, or similar attacks evaluated",
            ),
            (
                "Model inversion attack mitigation",
                "Differential privacy or output perturbation",
            ),
            (
                "Training data privacy ensured",
                "PII removal, anonymization, or synthetic data",
            ),
            (
                "Model access control configured",
                "API authentication and rate limiting for inference",
            ),
            (
                "Data compliance verified",
                "GDPR, HIPAA, or industry-specific data regulations",
            ),
        ],
    )?;

    let s8 = add_template_sprint(
        conn,
        tpl,
        "Monitoring & Drift Detection",
        "Production model monitoring",
        7,
    )?;
    add_template_sprint_sections(conn, s8, &[monitoring])?;
    add_custom_sprint_section(
        conn,
        s8,
        "Model Monitoring",
        "Drift detection and performance tracking",
        "#ec4899",
        &[
            (
                "Data drift detection configured",
                "Statistical tests for feature distribution changes",
            ),
            (
                "Concept drift monitoring implemented",
                "Performance degradation detection over time",
            ),
            (
                "Prediction logging enabled",
                "All predictions logged with input features for audit",
            ),
            (
                "Model retraining pipeline automated",
                "Triggered by drift detection or schedule",
            ),
            (
                "Alerting for model degradation",
                "Threshold-based alerts on accuracy or latency",
            ),
        ],
    )?;

    let s9 = add_template_sprint(
        conn,
        tpl,
        "Testing & QA",
        "ML-specific testing strategies",
        8,
    )?;
    add_template_sprint_sections(conn, s9, &[testing])?;

    let s10 = add_template_sprint(
        conn,
        tpl,
        "CI/CD & Deployment",
        "MLOps pipeline automation",
        9,
    )?;
    add_template_sprint_sections(conn, s10, &[cicd, docs])?;

    Ok(())
}
