//! # Compliance Pallet
//!
//! This pallet provides comprehensive compliance management for Thailand's energy trading blockchain.
//! It handles SEC, ERC, and PDPA compliance requirements with automated reporting and audit capabilities.

#![cfg_attr(not(feature = "std"), no_std)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export commonly used types
pub use types::*;

/// Pallet configuration
pub mod config;
/// Core compliance types
pub mod types;
/// SEC compliance implementation
pub mod sec;
/// ERC compliance implementation
pub mod erc;
/// PDPA compliance implementation
pub mod pdpa;
/// Audit system
pub mod audit;
/// Reporting system
pub mod reporting;
/// Validation logic
pub mod validation;
/// Error handling
pub mod error;

pub use config::Config;
pub use error::Error;

/// The main compliance pallet struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompliancePallet<T: Config> {
    /// SEC compliance manager
    pub sec_compliance: sec::SECCompliance,
    /// ERC compliance manager
    pub erc_compliance: erc::ERCCompliance,
    /// PDPA compliance manager
    pub pdpa_compliance: pdpa::PDPACompliance,
    /// Audit system
    pub audit_system: audit::AuditSystem,
    /// Reporting system
    pub reporting_system: reporting::ReportingSystem,
    /// Phantom data for config
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> CompliancePallet<T> {
    /// Create a new compliance pallet instance
    pub fn new() -> Self {
        Self {
            sec_compliance: sec::SECCompliance::new(),
            erc_compliance: erc::ERCCompliance::new(),
            pdpa_compliance: pdpa::PDPACompliance::new(),
            audit_system: audit::AuditSystem::new(),
            reporting_system: reporting::ReportingSystem::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Initialize compliance framework
    pub fn initialize(&mut self) -> Result<(), Error> {
        self.sec_compliance.initialize()?;
        self.erc_compliance.initialize()?;
        self.pdpa_compliance.initialize()?;
        self.audit_system.initialize()?;
        self.reporting_system.initialize()?;
        Ok(())
    }

    /// Validate transaction compliance
    pub fn validate_transaction(&self, transaction: &types::TransactionData) -> Result<(), Error> {
        // Validate against all compliance frameworks
        self.sec_compliance.validate_transaction(transaction)?;
        self.erc_compliance.validate_transaction(transaction)?;
        self.pdpa_compliance.validate_transaction(transaction)?;
        
        // Log for audit
        self.audit_system.log_transaction(transaction);
        
        Ok(())
    }

    /// Get compliance status
    pub fn get_status(&self) -> types::ComplianceStatus {
        types::ComplianceStatus {
            sec_compliant: self.sec_compliance.is_compliant(),
            erc_compliant: self.erc_compliance.is_compliant(),
            pdpa_compliant: self.pdpa_compliance.is_compliant(),
            last_audit: self.audit_system.last_audit_time(),
            compliance_score: self.calculate_compliance_score(),
        }
    }

    /// Calculate overall compliance score
    fn calculate_compliance_score(&self) -> f64 {
        let mut score = 0.0;
        let mut total_weight = 0.0;

        // SEC compliance weight: 40%
        if self.sec_compliance.is_compliant() {
            score += 0.4;
        }
        total_weight += 0.4;

        // ERC compliance weight: 35%
        if self.erc_compliance.is_compliant() {
            score += 0.35;
        }
        total_weight += 0.35;

        // PDPA compliance weight: 25%
        if self.pdpa_compliance.is_compliant() {
            score += 0.25;
        }
        total_weight += 0.25;

        if total_weight > 0.0 {
            score / total_weight
        } else {
            0.0
        }
    }

    /// Generate compliance report
    pub fn generate_report(&self, report_type: types::ReportType) -> Result<types::ComplianceReport, Error> {
        self.reporting_system.generate_report(
            report_type,
            &self.get_status(),
            &self.audit_system.get_audit_data(),
        )
    }
}

impl<T: Config> Default for CompliancePallet<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Compliance pallet dispatchable functions
impl<T: Config> CompliancePallet<T> {
    /// Submit compliance report
    pub fn submit_report(
        &mut self,
        report_type: types::ReportType,
        data: types::ReportData,
    ) -> Result<(), Error> {
        // Validate report data
        validation::validate_report_data(&data)?;
        
        // Submit to reporting system
        self.reporting_system.submit_report(report_type, data)?;
        
        // Log submission
        self.audit_system.log_report_submission(report_type);
        
        Ok(())
    }

    /// Update compliance configuration
    pub fn update_config(
        &mut self,
        config_type: types::ConfigType,
        config_data: types::ConfigData,
    ) -> Result<(), Error> {
        match config_type {
            types::ConfigType::SEC => {
                self.sec_compliance.update_config(config_data)?;
            }
            types::ConfigType::ERC => {
                self.erc_compliance.update_config(config_data)?;
            }
            types::ConfigType::PDPA => {
                self.pdpa_compliance.update_config(config_data)?;
            }
            types::ConfigType::Audit => {
                self.audit_system.update_config(config_data)?;
            }
            types::ConfigType::Reporting => {
                self.reporting_system.update_config(config_data)?;
            }
        }
        Ok(())
    }

    /// Perform compliance audit
    pub fn perform_audit(&mut self) -> Result<types::AuditResult, Error> {
        let audit_result = self.audit_system.perform_comprehensive_audit(&types::AuditScope {
            sec_compliance: true,
            erc_compliance: true,
            pdpa_compliance: true,
            transaction_validation: true,
            reporting_accuracy: true,
        })?;

        // Generate audit report
        let _report = self.generate_report(types::ReportType::Audit)?;

        Ok(audit_result)
    }
}
