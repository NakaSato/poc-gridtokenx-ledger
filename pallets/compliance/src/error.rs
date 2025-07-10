//! Error types for the compliance pallet

use serde::{Deserialize, Serialize};

/// Compliance pallet errors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
    /// Invalid transaction data
    InvalidTransactionData,
    /// SEC compliance violation
    SECComplianceViolation(String),
    /// ERC compliance violation
    ERCComplianceViolation(String),
    /// PDPA compliance violation
    PDPAComplianceViolation(String),
    /// Audit system error
    AuditSystemError(String),
    /// Reporting system error
    ReportingSystemError(String),
    /// Configuration error
    ConfigurationError(String),
    /// Validation error
    ValidationError(String),
    /// Insufficient permissions
    InsufficientPermissions,
    /// Data not found
    DataNotFound,
    /// Invalid report type
    InvalidReportType,
    /// Report generation failed
    ReportGenerationFailed,
    /// Compliance threshold not met
    ComplianceThresholdNotMet,
    /// Maximum reports exceeded
    MaxReportsExceeded,
    /// Report size too large
    ReportSizeTooLarge,
    /// Invalid configuration
    InvalidConfiguration,
    /// Database error
    DatabaseError(String),
    /// Network error
    NetworkError(String),
    /// Serialization error
    SerializationError(String),
    /// Unknown error
    Unknown(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidTransactionData => write!(f, "Invalid transaction data"),
            Error::SECComplianceViolation(msg) => write!(f, "SEC compliance violation: {}", msg),
            Error::ERCComplianceViolation(msg) => write!(f, "ERC compliance violation: {}", msg),
            Error::PDPAComplianceViolation(msg) => write!(f, "PDPA compliance violation: {}", msg),
            Error::AuditSystemError(msg) => write!(f, "Audit system error: {}", msg),
            Error::ReportingSystemError(msg) => write!(f, "Reporting system error: {}", msg),
            Error::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            Error::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Error::InsufficientPermissions => write!(f, "Insufficient permissions"),
            Error::DataNotFound => write!(f, "Data not found"),
            Error::InvalidReportType => write!(f, "Invalid report type"),
            Error::ReportGenerationFailed => write!(f, "Report generation failed"),
            Error::ComplianceThresholdNotMet => write!(f, "Compliance threshold not met"),
            Error::MaxReportsExceeded => write!(f, "Maximum reports exceeded"),
            Error::ReportSizeTooLarge => write!(f, "Report size too large"),
            Error::InvalidConfiguration => write!(f, "Invalid configuration"),
            Error::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Error::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Error::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            Error::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Result type for compliance operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error conversion utilities
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerializationError(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::NetworkError(err.to_string())
    }
}

/// Error categories for better error handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorCategory {
    /// Validation errors
    Validation,
    /// Compliance errors
    Compliance,
    /// System errors
    System,
    /// Configuration errors
    Configuration,
    /// Permission errors
    Permission,
    /// Data errors
    Data,
}

impl Error {
    /// Get the category of the error
    pub fn category(&self) -> ErrorCategory {
        match self {
            Error::InvalidTransactionData | Error::ValidationError(_) => ErrorCategory::Validation,
            Error::SECComplianceViolation(_)
            | Error::ERCComplianceViolation(_)
            | Error::PDPAComplianceViolation(_)
            | Error::ComplianceThresholdNotMet => ErrorCategory::Compliance,
            Error::AuditSystemError(_)
            | Error::ReportingSystemError(_)
            | Error::DatabaseError(_)
            | Error::NetworkError(_)
            | Error::SerializationError(_)
            | Error::Unknown(_) => ErrorCategory::System,
            Error::ConfigurationError(_) | Error::InvalidConfiguration => {
                ErrorCategory::Configuration
            }
            Error::InsufficientPermissions => ErrorCategory::Permission,
            Error::DataNotFound
            | Error::InvalidReportType
            | Error::ReportGenerationFailed
            | Error::MaxReportsExceeded
            | Error::ReportSizeTooLarge => ErrorCategory::Data,
        }
    }

    /// Check if the error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            Error::InvalidTransactionData
            | Error::ValidationError(_)
            | Error::InsufficientPermissions
            | Error::DataNotFound
            | Error::InvalidReportType
            | Error::InvalidConfiguration => false,
            Error::SECComplianceViolation(_)
            | Error::ERCComplianceViolation(_)
            | Error::PDPAComplianceViolation(_)
            | Error::AuditSystemError(_)
            | Error::ReportingSystemError(_)
            | Error::ConfigurationError(_)
            | Error::ReportGenerationFailed
            | Error::ComplianceThresholdNotMet
            | Error::MaxReportsExceeded
            | Error::ReportSizeTooLarge
            | Error::DatabaseError(_)
            | Error::NetworkError(_)
            | Error::SerializationError(_)
            | Error::Unknown(_) => true,
        }
    }
}
