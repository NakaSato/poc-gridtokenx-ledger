//! Core types for the compliance pallet

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Transaction data for compliance validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    /// Transaction ID
    pub id: String,
    /// Transaction type
    pub transaction_type: TransactionType,
    /// Transaction amount
    pub amount: f64,
    /// Source address
    pub from: String,
    /// Destination address
    pub to: String,
    /// Transaction timestamp
    pub timestamp: DateTime<Utc>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Transaction types for compliance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionType {
    /// Energy trading transaction
    EnergyTrade,
    /// Token transfer
    TokenTransfer,
    /// Governance vote
    GovernanceVote,
    /// Compliance report submission
    ComplianceReport,
    /// Oracle data update
    OracleUpdate,
    /// Cross-chain transaction
    CrossChain,
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    /// SEC compliance status
    pub sec_compliant: bool,
    /// ERC compliance status
    pub erc_compliant: bool,
    /// PDPA compliance status
    pub pdpa_compliant: bool,
    /// Last audit timestamp
    pub last_audit: DateTime<Utc>,
    /// Overall compliance score (0.0 to 1.0)
    pub compliance_score: f64,
}

/// Report types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReportType {
    /// Daily compliance report
    Daily,
    /// Weekly compliance report
    Weekly,
    /// Monthly compliance report
    Monthly,
    /// Quarterly compliance report
    Quarterly,
    /// Annual compliance report
    Annual,
    /// Audit report
    Audit,
    /// Incident report
    Incident,
    /// Custom report
    Custom(String),
}

/// Report data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    /// Report ID
    pub id: String,
    /// Report type
    pub report_type: ReportType,
    /// Report period start
    pub period_start: DateTime<Utc>,
    /// Report period end
    pub period_end: DateTime<Utc>,
    /// Report data
    pub data: serde_json::Value,
    /// Report generation timestamp
    pub generated_at: DateTime<Utc>,
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Report metadata
    pub metadata: ReportMetadata,
    /// SEC compliance section
    pub sec_section: SECReportSection,
    /// ERC compliance section
    pub erc_section: ERCReportSection,
    /// PDPA compliance section
    pub pdpa_section: PDPAReportSection,
    /// Summary
    pub summary: ReportSummary,
}

/// Report metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    /// Report ID
    pub id: String,
    /// Report type
    pub report_type: ReportType,
    /// Generation timestamp
    pub generated_at: DateTime<Utc>,
    /// Period covered
    pub period: ReportPeriod,
    /// Report version
    pub version: String,
}

/// Report period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportPeriod {
    /// Period start
    pub start: DateTime<Utc>,
    /// Period end
    pub end: DateTime<Utc>,
}

/// SEC report section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SECReportSection {
    /// Token registration status
    pub token_registration: RegistrationStatus,
    /// KYC compliance metrics
    pub kyc_metrics: KYCMetrics,
    /// AML monitoring results
    pub aml_results: AMLResults,
    /// Transaction monitoring summary
    pub transaction_monitoring: TransactionMonitoringSummary,
}

/// ERC report section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERCReportSection {
    /// Grid code compliance status
    pub grid_code_compliance: GridCodeCompliance,
    /// Energy trading license status
    pub license_status: LicenseStatus,
    /// Safety protocol compliance
    pub safety_compliance: SafetyCompliance,
    /// Grid connection standards
    pub grid_standards: GridStandards,
}

/// PDPA report section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PDPAReportSection {
    /// Data protection measures
    pub data_protection: DataProtectionMeasures,
    /// Consent management status
    pub consent_management: ConsentManagementStatus,
    /// Data retention compliance
    pub data_retention: DataRetentionCompliance,
    /// Breach incidents
    pub breach_incidents: Vec<BreachIncident>,
}

/// Report summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    /// Overall compliance score
    pub compliance_score: f64,
    /// Key findings
    pub key_findings: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Action items
    pub action_items: Vec<ActionItem>,
}

/// Configuration types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfigType {
    /// SEC configuration
    SEC,
    /// ERC configuration
    ERC,
    /// PDPA configuration
    PDPA,
    /// Audit configuration
    Audit,
    /// Reporting configuration
    Reporting,
}

/// Configuration data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigData {
    /// Configuration type
    pub config_type: ConfigType,
    /// Configuration values
    pub data: serde_json::Value,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Audit result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResult {
    /// Audit ID
    pub id: String,
    /// Audit timestamp
    pub timestamp: DateTime<Utc>,
    /// Audit scope
    pub scope: AuditScope,
    /// Audit findings
    pub findings: Vec<AuditFinding>,
    /// Compliance score
    pub compliance_score: f64,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Audit scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditScope {
    /// SEC compliance audit
    pub sec_compliance: bool,
    /// ERC compliance audit
    pub erc_compliance: bool,
    /// PDPA compliance audit
    pub pdpa_compliance: bool,
    /// Transaction validation audit
    pub transaction_validation: bool,
    /// Reporting accuracy audit
    pub reporting_accuracy: bool,
}

/// Audit finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFinding {
    /// Finding ID
    pub id: String,
    /// Finding type
    pub finding_type: FindingType,
    /// Severity level
    pub severity: SeverityLevel,
    /// Description
    pub description: String,
    /// Affected components
    pub affected_components: Vec<String>,
    /// Remediation steps
    pub remediation: Vec<String>,
}

/// Finding type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FindingType {
    /// Compliance violation
    ComplianceViolation,
    /// Security issue
    SecurityIssue,
    /// Process gap
    ProcessGap,
    /// Data quality issue
    DataQuality,
    /// Configuration issue
    Configuration,
}

/// Severity level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SeverityLevel {
    /// Critical issue
    Critical,
    /// High priority
    High,
    /// Medium priority
    Medium,
    /// Low priority
    Low,
    /// Information only
    Info,
}

// Additional supporting types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationStatus {
    pub grid_token: bool,
    pub watt_token: bool,
    pub registration_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KYCMetrics {
    pub total_users: u64,
    pub verified_users: u64,
    pub pending_verification: u64,
    pub verification_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AMLResults {
    pub total_transactions: u64,
    pub flagged_transactions: u64,
    pub alerts_generated: u64,
    pub false_positive_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMonitoringSummary {
    pub total_volume: f64,
    pub suspicious_activity: u64,
    pub threshold_breaches: u64,
    pub reporting_accuracy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridCodeCompliance {
    pub compliance_percentage: f64,
    pub violations: u64,
    pub last_assessment: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseStatus {
    pub valid: bool,
    pub expiration_date: Option<DateTime<Utc>>,
    pub renewal_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyCompliance {
    pub safety_score: f64,
    pub incidents: u64,
    pub last_inspection: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridStandards {
    pub compliance_level: String,
    pub standards_met: Vec<String>,
    pub gaps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProtectionMeasures {
    pub encryption_enabled: bool,
    pub access_controls: bool,
    pub audit_logging: bool,
    pub data_minimization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentManagementStatus {
    pub consent_rate: f64,
    pub active_consents: u64,
    pub withdrawn_consents: u64,
    pub consent_compliance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionCompliance {
    pub retention_policy_applied: bool,
    pub data_purged: u64,
    pub retention_violations: u64,
    pub compliance_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachIncident {
    pub incident_id: String,
    pub incident_date: DateTime<Utc>,
    pub incident_type: String,
    pub severity: SeverityLevel,
    pub affected_records: u64,
    pub resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionItem {
    pub id: String,
    pub description: String,
    pub priority: SeverityLevel,
    pub assigned_to: String,
    pub due_date: DateTime<Utc>,
    pub status: ActionItemStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionItemStatus {
    Open,
    InProgress,
    Completed,
    Deferred,
    Cancelled,
}
