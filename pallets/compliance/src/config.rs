//! Configuration types for the compliance pallet

use std::marker::PhantomData;

/// Configuration trait for the compliance pallet
pub trait Config {
    /// The maximum number of compliance reports that can be stored
    const MAX_REPORTS: u32 = 1000;
    
    /// The maximum size of a compliance report in bytes
    const MAX_REPORT_SIZE: u32 = 1024 * 1024; // 1MB
    
    /// The audit interval in seconds
    const AUDIT_INTERVAL: u64 = 86400; // 24 hours
    
    /// The reporting retention period in seconds
    const REPORT_RETENTION: u64 = 31536000; // 1 year
}

/// Default configuration implementation
#[derive(Debug, Clone)]
pub struct DefaultConfig;

impl Config for DefaultConfig {
    const MAX_REPORTS: u32 = 1000;
    const MAX_REPORT_SIZE: u32 = 1024 * 1024;
    const AUDIT_INTERVAL: u64 = 86400;
    const REPORT_RETENTION: u64 = 31536000;
}

/// Configuration for SEC compliance
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SECConfig {
    /// Token registration requirements
    pub token_registration_enabled: bool,
    /// KYC requirements
    pub kyc_required: bool,
    /// AML monitoring enabled
    pub aml_monitoring: bool,
    /// Transaction monitoring thresholds
    pub transaction_threshold: f64,
    /// Reporting frequency in seconds
    pub reporting_frequency: u64,
}

impl Default for SECConfig {
    fn default() -> Self {
        Self {
            token_registration_enabled: true,
            kyc_required: true,
            aml_monitoring: true,
            transaction_threshold: 10000.0, // 10,000 THB
            reporting_frequency: 86400, // Daily
        }
    }
}

/// Configuration for ERC compliance
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ERCConfig {
    /// Grid code compliance enabled
    pub grid_code_compliance: bool,
    /// Energy trading license required
    pub license_required: bool,
    /// Grid connection standards
    pub grid_connection_standards: bool,
    /// Safety protocols enabled
    pub safety_protocols: bool,
    /// Reporting frequency in seconds
    pub reporting_frequency: u64,
}

impl Default for ERCConfig {
    fn default() -> Self {
        Self {
            grid_code_compliance: true,
            license_required: true,
            grid_connection_standards: true,
            safety_protocols: true,
            reporting_frequency: 86400, // Daily
        }
    }
}

/// Configuration for PDPA compliance
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PDPAConfig {
    /// Data protection enabled
    pub data_protection_enabled: bool,
    /// Consent management required
    pub consent_management: bool,
    /// Data retention period in seconds
    pub data_retention_period: u64,
    /// Anonymization required
    pub anonymization_required: bool,
    /// Audit logging enabled
    pub audit_logging: bool,
}

impl Default for PDPAConfig {
    fn default() -> Self {
        Self {
            data_protection_enabled: true,
            consent_management: true,
            data_retention_period: 31536000, // 1 year
            anonymization_required: true,
            audit_logging: true,
        }
    }
}

/// Overall compliance configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComplianceConfig {
    /// SEC configuration
    pub sec: SECConfig,
    /// ERC configuration
    pub erc: ERCConfig,
    /// PDPA configuration
    pub pdpa: PDPAConfig,
    /// Global compliance settings
    pub global: GlobalConfig,
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            sec: SECConfig::default(),
            erc: ERCConfig::default(),
            pdpa: PDPAConfig::default(),
            global: GlobalConfig::default(),
        }
    }
}

/// Global compliance configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GlobalConfig {
    /// Compliance monitoring enabled
    pub monitoring_enabled: bool,
    /// Real-time alerts enabled
    pub real_time_alerts: bool,
    /// Automated reporting enabled
    pub automated_reporting: bool,
    /// Compliance score threshold
    pub compliance_threshold: f64,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            monitoring_enabled: true,
            real_time_alerts: true,
            automated_reporting: true,
            compliance_threshold: 0.8, // 80% compliance required
        }
    }
}
