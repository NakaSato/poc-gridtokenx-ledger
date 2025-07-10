//! Validation functions for the compliance pallet

use crate::types::*;
use crate::error::{Error, Result};
use chrono::{DateTime, Utc};

/// Validate transaction data
pub fn validate_transaction_data(transaction: &TransactionData) -> Result<()> {
    // Check transaction ID
    if transaction.id.is_empty() {
        return Err(Error::ValidationError("Transaction ID cannot be empty".to_string()));
    }

    // Check amount for relevant transaction types
    match transaction.transaction_type {
        TransactionType::EnergyTrade | TransactionType::TokenTransfer => {
            if transaction.amount <= 0.0 {
                return Err(Error::ValidationError("Transaction amount must be positive".to_string()));
            }
        }
        _ => {}
    }

    // Check addresses
    if transaction.from.is_empty() {
        return Err(Error::ValidationError("Source address cannot be empty".to_string()));
    }

    if transaction.to.is_empty() {
        return Err(Error::ValidationError("Destination address cannot be empty".to_string()));
    }

    // Check timestamp
    let now = Utc::now();
    if transaction.timestamp > now {
        return Err(Error::ValidationError("Transaction timestamp cannot be in the future".to_string()));
    }

    // Check if timestamp is too old (more than 24 hours)
    let max_age = chrono::Duration::hours(24);
    if now - transaction.timestamp > max_age {
        return Err(Error::ValidationError("Transaction timestamp is too old".to_string()));
    }

    Ok(())
}

/// Validate report data
pub fn validate_report_data(report: &ReportData) -> Result<()> {
    // Check report ID
    if report.id.is_empty() {
        return Err(Error::ValidationError("Report ID cannot be empty".to_string()));
    }

    // Check period validity
    if report.period_start >= report.period_end {
        return Err(Error::ValidationError("Report period start must be before end".to_string()));
    }

    // Check if period is not too long (max 1 year)
    let max_period = chrono::Duration::days(365);
    if report.period_end - report.period_start > max_period {
        return Err(Error::ValidationError("Report period cannot exceed 1 year".to_string()));
    }

    // Check generation timestamp
    let now = Utc::now();
    if report.generated_at > now {
        return Err(Error::ValidationError("Report generation timestamp cannot be in the future".to_string()));
    }

    // Validate report data size
    let data_size = serde_json::to_string(&report.data)
        .map_err(|e| Error::ValidationError(format!("Failed to serialize report data: {}", e)))?
        .len();
    
    if data_size > 1024 * 1024 { // 1MB limit
        return Err(Error::ValidationError("Report data size exceeds maximum limit".to_string()));
    }

    Ok(())
}

/// Validate configuration data
pub fn validate_config_data(config: &ConfigData) -> Result<()> {
    // Check if configuration data is valid JSON
    match serde_json::from_value::<serde_json::Value>(config.data.clone()) {
        Ok(_) => {}
        Err(e) => return Err(Error::ValidationError(format!("Invalid configuration JSON: {}", e))),
    }

    // Check timestamp
    let now = Utc::now();
    if config.updated_at > now {
        return Err(Error::ValidationError("Configuration update timestamp cannot be in the future".to_string()));
    }

    // Type-specific validation
    match config.config_type {
        ConfigType::SEC => validate_sec_config(&config.data)?,
        ConfigType::ERC => validate_erc_config(&config.data)?,
        ConfigType::PDPA => validate_pdpa_config(&config.data)?,
        ConfigType::Audit => validate_audit_config(&config.data)?,
        ConfigType::Reporting => validate_reporting_config(&config.data)?,
    }

    Ok(())
}

/// Validate SEC configuration
fn validate_sec_config(data: &serde_json::Value) -> Result<()> {
    // Check required fields
    let required_fields = ["token_registration_enabled", "kyc_required", "aml_monitoring"];
    for field in &required_fields {
        if !data.get(field).is_some() {
            return Err(Error::ValidationError(format!("Missing required SEC config field: {}", field)));
        }
    }

    // Validate transaction threshold
    if let Some(threshold) = data.get("transaction_threshold") {
        if let Some(threshold_val) = threshold.as_f64() {
            if threshold_val < 0.0 {
                return Err(Error::ValidationError("Transaction threshold must be non-negative".to_string()));
            }
        }
    }

    Ok(())
}

/// Validate ERC configuration
fn validate_erc_config(data: &serde_json::Value) -> Result<()> {
    // Check required fields
    let required_fields = ["grid_code_compliance", "license_required", "safety_protocols"];
    for field in &required_fields {
        if !data.get(field).is_some() {
            return Err(Error::ValidationError(format!("Missing required ERC config field: {}", field)));
        }
    }

    Ok(())
}

/// Validate PDPA configuration
fn validate_pdpa_config(data: &serde_json::Value) -> Result<()> {
    // Check required fields
    let required_fields = ["data_protection_enabled", "consent_management"];
    for field in &required_fields {
        if !data.get(field).is_some() {
            return Err(Error::ValidationError(format!("Missing required PDPA config field: {}", field)));
        }
    }

    // Validate data retention period
    if let Some(retention) = data.get("data_retention_period") {
        if let Some(retention_val) = retention.as_u64() {
            if retention_val == 0 {
                return Err(Error::ValidationError("Data retention period must be greater than 0".to_string()));
            }
        }
    }

    Ok(())
}

/// Validate audit configuration
fn validate_audit_config(data: &serde_json::Value) -> Result<()> {
    // Check audit interval
    if let Some(interval) = data.get("audit_interval") {
        if let Some(interval_val) = interval.as_u64() {
            if interval_val < 3600 { // Minimum 1 hour
                return Err(Error::ValidationError("Audit interval must be at least 1 hour".to_string()));
            }
        }
    }

    Ok(())
}

/// Validate reporting configuration
fn validate_reporting_config(data: &serde_json::Value) -> Result<()> {
    // Check reporting frequency
    if let Some(frequency) = data.get("reporting_frequency") {
        if let Some(frequency_val) = frequency.as_u64() {
            if frequency_val < 3600 { // Minimum 1 hour
                return Err(Error::ValidationError("Reporting frequency must be at least 1 hour".to_string()));
            }
        }
    }

    Ok(())
}

/// Validate audit scope
pub fn validate_audit_scope(scope: &AuditScope) -> Result<()> {
    // At least one scope must be selected
    if !scope.sec_compliance 
        && !scope.erc_compliance 
        && !scope.pdpa_compliance 
        && !scope.transaction_validation 
        && !scope.reporting_accuracy {
        return Err(Error::ValidationError("At least one audit scope must be selected".to_string()));
    }

    Ok(())
}

/// Validate compliance threshold
pub fn validate_compliance_threshold(threshold: f64) -> Result<()> {
    if threshold < 0.0 || threshold > 1.0 {
        return Err(Error::ValidationError("Compliance threshold must be between 0.0 and 1.0".to_string()));
    }
    Ok(())
}

/// Validate user permissions for compliance operations
pub fn validate_user_permissions(user_id: &str, operation: &str) -> Result<()> {
    // Placeholder for permission validation
    // In a real implementation, this would check against a permission system
    if user_id.is_empty() {
        return Err(Error::ValidationError("User ID cannot be empty".to_string()));
    }

    // Mock permission check - in reality this would query a permission system
    let allowed_operations = vec!["read", "write", "audit", "configure"];
    if !allowed_operations.contains(&operation) {
        return Err(Error::InsufficientPermissions);
    }

    Ok(())
}

/// Validate report generation parameters
pub fn validate_report_generation_params(
    report_type: &ReportType,
    start_date: &DateTime<Utc>,
    end_date: &DateTime<Utc>,
) -> Result<()> {
    // Check date range
    if start_date >= end_date {
        return Err(Error::ValidationError("Start date must be before end date".to_string()));
    }

    // Check if end date is not in the future
    let now = Utc::now();
    if *end_date > now {
        return Err(Error::ValidationError("End date cannot be in the future".to_string()));
    }

    // Validate report type specific constraints
    match report_type {
        ReportType::Daily => {
            let max_period = chrono::Duration::days(1);
            if *end_date - *start_date > max_period {
                return Err(Error::ValidationError("Daily report period cannot exceed 1 day".to_string()));
            }
        }
        ReportType::Weekly => {
            let max_period = chrono::Duration::weeks(1);
            if *end_date - *start_date > max_period {
                return Err(Error::ValidationError("Weekly report period cannot exceed 1 week".to_string()));
            }
        }
        ReportType::Monthly => {
            let max_period = chrono::Duration::days(31);
            if *end_date - *start_date > max_period {
                return Err(Error::ValidationError("Monthly report period cannot exceed 31 days".to_string()));
            }
        }
        ReportType::Quarterly => {
            let max_period = chrono::Duration::days(92);
            if *end_date - *start_date > max_period {
                return Err(Error::ValidationError("Quarterly report period cannot exceed 92 days".to_string()));
            }
        }
        ReportType::Annual => {
            let max_period = chrono::Duration::days(366);
            if *end_date - *start_date > max_period {
                return Err(Error::ValidationError("Annual report period cannot exceed 366 days".to_string()));
            }
        }
        ReportType::Audit | ReportType::Incident => {
            // More flexible for audit and incident reports
            let max_period = chrono::Duration::days(365);
            if *end_date - *start_date > max_period {
                return Err(Error::ValidationError("Report period cannot exceed 1 year".to_string()));
            }
        }
        ReportType::Custom(_) => {
            // Custom reports have more flexible constraints
            let max_period = chrono::Duration::days(365);
            if *end_date - *start_date > max_period {
                return Err(Error::ValidationError("Custom report period cannot exceed 1 year".to_string()));
            }
        }
    }

    Ok(())
}
