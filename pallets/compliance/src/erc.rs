//! ERC (Energy Regulatory Commission) compliance module

use crate::types::*;
use crate::error::{Error, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ERC compliance manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERCCompliance {
    /// Grid code compliance
    pub grid_code: GridCodeCompliance,
    /// Energy trading license management
    pub licensing: LicensingSystem,
    /// Safety protocol compliance
    pub safety: SafetyProtocolSystem,
    /// Grid connection standards
    pub grid_standards: GridStandardsSystem,
    /// Configuration
    pub config: ERCConfig,
}

/// Grid code compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridCodeCompliance {
    /// Compliance status
    pub status: ComplianceStatus,
    /// Last assessment date
    pub last_assessment: DateTime<Utc>,
    /// Compliance score
    pub compliance_score: f64,
    /// Violations
    pub violations: Vec<GridCodeViolation>,
}

/// Grid code violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridCodeViolation {
    /// Violation ID
    pub id: String,
    /// Violation type
    pub violation_type: ViolationType,
    /// Description
    pub description: String,
    /// Severity
    pub severity: SeverityLevel,
    /// Date detected
    pub detected_at: DateTime<Utc>,
    /// Resolution status
    pub resolved: bool,
}

/// Violation type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ViolationType {
    /// Voltage regulation
    VoltageRegulation,
    /// Frequency control
    FrequencyControl,
    /// Power quality
    PowerQuality,
    /// Grid stability
    GridStability,
    /// Interconnection standards
    InterconnectionStandards,
}

/// Licensing system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensingSystem {
    /// Active licenses
    pub active_licenses: HashMap<String, EnergyLicense>,
    /// Pending applications
    pub pending_applications: Vec<LicenseApplication>,
    /// License history
    pub license_history: Vec<LicenseEvent>,
}

/// Energy license
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyLicense {
    /// License ID
    pub id: String,
    /// License type
    pub license_type: LicenseType,
    /// Holder information
    pub holder: LicenseHolder,
    /// Issue date
    pub issued_at: DateTime<Utc>,
    /// Expiration date
    pub expires_at: DateTime<Utc>,
    /// Status
    pub status: LicenseStatus,
    /// Conditions
    pub conditions: Vec<LicenseCondition>,
}

/// License type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LicenseType {
    /// Generation license
    Generation,
    /// Distribution license
    Distribution,
    /// Retail license
    Retail,
    /// Trading license
    Trading,
    /// Brokerage license
    Brokerage,
}

/// License holder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseHolder {
    /// Name
    pub name: String,
    /// Registration number
    pub registration_number: String,
    /// Address
    pub address: String,
    /// Contact information
    pub contact: ContactInfo,
}

/// License status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LicenseStatus {
    /// Active
    Active,
    /// Suspended
    Suspended,
    /// Revoked
    Revoked,
    /// Expired
    Expired,
    /// Pending renewal
    PendingRenewal,
}

/// License condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseCondition {
    /// Condition ID
    pub id: String,
    /// Description
    pub description: String,
    /// Compliance required
    pub compliance_required: bool,
    /// Monitoring frequency
    pub monitoring_frequency: MonitoringFrequency,
}

/// Monitoring frequency
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MonitoringFrequency {
    /// Real-time
    RealTime,
    /// Hourly
    Hourly,
    /// Daily
    Daily,
    /// Weekly
    Weekly,
    /// Monthly
    Monthly,
    /// Quarterly
    Quarterly,
    /// Annually
    Annually,
}

/// License application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseApplication {
    /// Application ID
    pub id: String,
    /// Applicant information
    pub applicant: LicenseHolder,
    /// Requested license type
    pub license_type: LicenseType,
    /// Application date
    pub applied_at: DateTime<Utc>,
    /// Status
    pub status: ApplicationStatus,
    /// Documents
    pub documents: Vec<Document>,
}

/// Application status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApplicationStatus {
    /// Submitted
    Submitted,
    /// Under review
    UnderReview,
    /// Additional information required
    AdditionalInfoRequired,
    /// Approved
    Approved,
    /// Rejected
    Rejected,
    /// Withdrawn
    Withdrawn,
}

/// License event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseEvent {
    /// Event ID
    pub id: String,
    /// License ID
    pub license_id: String,
    /// Event type
    pub event_type: LicenseEventType,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Details
    pub details: String,
}

/// License event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LicenseEventType {
    /// Issued
    Issued,
    /// Renewed
    Renewed,
    /// Suspended
    Suspended,
    /// Revoked
    Revoked,
    /// Expired
    Expired,
    /// Condition added
    ConditionAdded,
    /// Condition removed
    ConditionRemoved,
}

/// Safety protocol system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyProtocolSystem {
    /// Safety protocols
    pub protocols: Vec<SafetyProtocol>,
    /// Safety incidents
    pub incidents: Vec<SafetyIncident>,
    /// Safety assessments
    pub assessments: Vec<SafetyAssessment>,
}

/// Safety protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyProtocol {
    /// Protocol ID
    pub id: String,
    /// Protocol name
    pub name: String,
    /// Protocol type
    pub protocol_type: SafetyProtocolType,
    /// Requirements
    pub requirements: Vec<SafetyRequirement>,
    /// Compliance level
    pub compliance_level: SafetyComplianceLevel,
}

/// Safety protocol type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SafetyProtocolType {
    /// Electrical safety
    ElectricalSafety,
    /// Fire safety
    FireSafety,
    /// Environmental safety
    EnvironmentalSafety,
    /// Personnel safety
    PersonnelSafety,
    /// Equipment safety
    EquipmentSafety,
}

/// Safety requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyRequirement {
    /// Requirement ID
    pub id: String,
    /// Description
    pub description: String,
    /// Mandatory
    pub mandatory: bool,
    /// Compliance status
    pub compliance_status: ComplianceStatus,
}

/// Safety compliance level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SafetyComplianceLevel {
    /// Full compliance
    FullCompliance,
    /// Partial compliance
    PartialCompliance,
    /// Non-compliance
    NonCompliance,
}

/// Safety incident
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyIncident {
    /// Incident ID
    pub id: String,
    /// Incident type
    pub incident_type: SafetyIncidentType,
    /// Severity
    pub severity: SeverityLevel,
    /// Date occurred
    pub occurred_at: DateTime<Utc>,
    /// Location
    pub location: String,
    /// Description
    pub description: String,
    /// Investigation status
    pub investigation_status: InvestigationStatus,
}

/// Safety incident type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SafetyIncidentType {
    /// Electrical fault
    ElectricalFault,
    /// Fire
    Fire,
    /// Equipment failure
    EquipmentFailure,
    /// Personnel injury
    PersonnelInjury,
    /// Environmental impact
    EnvironmentalImpact,
}

/// Investigation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InvestigationStatus {
    /// Pending
    Pending,
    /// In progress
    InProgress,
    /// Completed
    Completed,
    /// Closed
    Closed,
}

/// Safety assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyAssessment {
    /// Assessment ID
    pub id: String,
    /// Assessment type
    pub assessment_type: AssessmentType,
    /// Assessed entity
    pub assessed_entity: String,
    /// Assessment date
    pub assessed_at: DateTime<Utc>,
    /// Assessor
    pub assessor: String,
    /// Results
    pub results: AssessmentResults,
}

/// Assessment type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AssessmentType {
    /// Initial assessment
    Initial,
    /// Periodic assessment
    Periodic,
    /// Incident-triggered assessment
    IncidentTriggered,
    /// Compliance assessment
    Compliance,
}

/// Assessment results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentResults {
    /// Overall score
    pub overall_score: f64,
    /// Findings
    pub findings: Vec<AssessmentFinding>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Required actions
    pub required_actions: Vec<RequiredAction>,
}

/// Assessment finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentFinding {
    /// Finding ID
    pub id: String,
    /// Finding type
    pub finding_type: FindingType,
    /// Description
    pub description: String,
    /// Severity
    pub severity: SeverityLevel,
    /// Area of concern
    pub area: String,
}

/// Required action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredAction {
    /// Action ID
    pub id: String,
    /// Description
    pub description: String,
    /// Priority
    pub priority: SeverityLevel,
    /// Due date
    pub due_date: DateTime<Utc>,
    /// Responsible party
    pub responsible_party: String,
}

/// Grid standards system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridStandardsSystem {
    /// Standards
    pub standards: Vec<GridStandard>,
    /// Compliance assessments
    pub assessments: Vec<StandardsAssessment>,
    /// Certificates
    pub certificates: Vec<StandardsCertificate>,
}

/// Grid standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridStandard {
    /// Standard ID
    pub id: String,
    /// Standard name
    pub name: String,
    /// Standard type
    pub standard_type: StandardType,
    /// Requirements
    pub requirements: Vec<StandardRequirement>,
    /// Compliance level
    pub compliance_level: StandardComplianceLevel,
}

/// Standard type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StandardType {
    /// Technical standard
    Technical,
    /// Performance standard
    Performance,
    /// Safety standard
    Safety,
    /// Environmental standard
    Environmental,
    /// Quality standard
    Quality,
}

/// Standard requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardRequirement {
    /// Requirement ID
    pub id: String,
    /// Description
    pub description: String,
    /// Metric
    pub metric: String,
    /// Target value
    pub target_value: f64,
    /// Tolerance
    pub tolerance: f64,
    /// Measurement frequency
    pub measurement_frequency: MonitoringFrequency,
}

/// Standard compliance level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StandardComplianceLevel {
    /// Exceeds standards
    Exceeds,
    /// Meets standards
    Meets,
    /// Below standards
    Below,
    /// Non-compliant
    NonCompliant,
}

/// Standards assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardsAssessment {
    /// Assessment ID
    pub id: String,
    /// Standard ID
    pub standard_id: String,
    /// Assessed entity
    pub assessed_entity: String,
    /// Assessment date
    pub assessed_at: DateTime<Utc>,
    /// Results
    pub results: StandardsAssessmentResults,
}

/// Standards assessment results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardsAssessmentResults {
    /// Compliance level
    pub compliance_level: StandardComplianceLevel,
    /// Score
    pub score: f64,
    /// Measurements
    pub measurements: Vec<StandardMeasurement>,
    /// Gaps
    pub gaps: Vec<ComplianceGap>,
}

/// Standard measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardMeasurement {
    /// Measurement ID
    pub id: String,
    /// Requirement ID
    pub requirement_id: String,
    /// Measured value
    pub measured_value: f64,
    /// Target value
    pub target_value: f64,
    /// Compliance status
    pub compliance_status: ComplianceStatus,
    /// Measurement date
    pub measured_at: DateTime<Utc>,
}

/// Compliance gap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceGap {
    /// Gap ID
    pub id: String,
    /// Requirement ID
    pub requirement_id: String,
    /// Description
    pub description: String,
    /// Severity
    pub severity: SeverityLevel,
    /// Recommended action
    pub recommended_action: String,
}

/// Standards certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardsCertificate {
    /// Certificate ID
    pub id: String,
    /// Standard ID
    pub standard_id: String,
    /// Certificate holder
    pub holder: String,
    /// Issue date
    pub issued_at: DateTime<Utc>,
    /// Expiration date
    pub expires_at: DateTime<Utc>,
    /// Status
    pub status: CertificateStatus,
}

/// Certificate status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificateStatus {
    /// Valid
    Valid,
    /// Expired
    Expired,
    /// Suspended
    Suspended,
    /// Revoked
    Revoked,
}

/// ERC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERCConfig {
    /// Grid code compliance enabled
    pub grid_code_compliance: bool,
    /// License requirements enabled
    pub license_requirements: bool,
    /// Safety protocols enabled
    pub safety_protocols: bool,
    /// Grid standards enabled
    pub grid_standards: bool,
    /// Reporting frequency
    pub reporting_frequency: u64,
}

impl Default for ERCConfig {
    fn default() -> Self {
        Self {
            grid_code_compliance: true,
            license_requirements: true,
            safety_protocols: true,
            grid_standards: true,
            reporting_frequency: 86400, // Daily
        }
    }
}

impl ERCCompliance {
    /// Create new ERC compliance instance
    pub fn new() -> Self {
        Self {
            grid_code: GridCodeCompliance {
                status: ComplianceStatus::UnderReview,
                last_assessment: Utc::now(),
                compliance_score: 0.0,
                violations: Vec::new(),
            },
            licensing: LicensingSystem {
                active_licenses: HashMap::new(),
                pending_applications: Vec::new(),
                license_history: Vec::new(),
            },
            safety: SafetyProtocolSystem {
                protocols: Vec::new(),
                incidents: Vec::new(),
                assessments: Vec::new(),
            },
            grid_standards: GridStandardsSystem {
                standards: Vec::new(),
                assessments: Vec::new(),
                certificates: Vec::new(),
            },
            config: ERCConfig::default(),
        }
    }

    /// Initialize ERC compliance
    pub fn initialize(&mut self) -> Result<()> {
        // Initialize grid code compliance
        self.initialize_grid_code_compliance()?;
        
        // Initialize safety protocols
        self.initialize_safety_protocols()?;
        
        // Initialize grid standards
        self.initialize_grid_standards()?;
        
        Ok(())
    }

    /// Initialize grid code compliance
    fn initialize_grid_code_compliance(&mut self) -> Result<()> {
        self.grid_code.status = ComplianceStatus::UnderReview;
        self.grid_code.last_assessment = Utc::now();
        self.grid_code.compliance_score = 0.0;
        Ok(())
    }

    /// Initialize safety protocols
    fn initialize_safety_protocols(&mut self) -> Result<()> {
        // Add default safety protocols
        let electrical_safety = SafetyProtocol {
            id: "electrical_safety".to_string(),
            name: "Electrical Safety Protocol".to_string(),
            protocol_type: SafetyProtocolType::ElectricalSafety,
            requirements: Vec::new(),
            compliance_level: SafetyComplianceLevel::PartialCompliance,
        };
        
        self.safety.protocols.push(electrical_safety);
        Ok(())
    }

    /// Initialize grid standards
    fn initialize_grid_standards(&mut self) -> Result<()> {
        // Add default grid standards
        let voltage_standard = GridStandard {
            id: "voltage_standard".to_string(),
            name: "Voltage Regulation Standard".to_string(),
            standard_type: StandardType::Technical,
            requirements: Vec::new(),
            compliance_level: StandardComplianceLevel::Meets,
        };
        
        self.grid_standards.standards.push(voltage_standard);
        Ok(())
    }

    /// Validate transaction for ERC compliance
    pub fn validate_transaction(&self, transaction: &TransactionData) -> Result<()> {
        // Check if transaction type requires ERC compliance
        match transaction.transaction_type {
            TransactionType::EnergyTrade => {
                // Check grid code compliance
                if self.config.grid_code_compliance {
                    self.validate_grid_code_compliance(transaction)?;
                }
                
                // Check licensing requirements
                if self.config.license_requirements {
                    self.validate_licensing_requirements(transaction)?;
                }
                
                // Check safety protocols
                if self.config.safety_protocols {
                    self.validate_safety_protocols(transaction)?;
                }
                
                // Check grid standards
                if self.config.grid_standards {
                    self.validate_grid_standards(transaction)?;
                }
            }
            _ => {} // Other transaction types may not require ERC compliance
        }
        
        Ok(())
    }

    /// Validate grid code compliance
    fn validate_grid_code_compliance(&self, _transaction: &TransactionData) -> Result<()> {
        if self.grid_code.status != ComplianceStatus::Compliant {
            return Err(Error::ERCComplianceViolation(
                "Grid code compliance not met".to_string()
            ));
        }
        Ok(())
    }

    /// Validate licensing requirements
    fn validate_licensing_requirements(&self, transaction: &TransactionData) -> Result<()> {
        // Check if source address has required licenses
        let has_trading_license = self.licensing.active_licenses.values()
            .any(|license| license.license_type == LicenseType::Trading && 
                         license.status == LicenseStatus::Active);
        
        if !has_trading_license {
            return Err(Error::ERCComplianceViolation(
                "Trading license required".to_string()
            ));
        }
        
        Ok(())
    }

    /// Validate safety protocols
    fn validate_safety_protocols(&self, _transaction: &TransactionData) -> Result<()> {
        // Check if all mandatory safety protocols are compliant
        for protocol in &self.safety.protocols {
            if protocol.compliance_level == SafetyComplianceLevel::NonCompliance {
                return Err(Error::ERCComplianceViolation(
                    format!("Safety protocol {} not compliant", protocol.name)
                ));
            }
        }
        Ok(())
    }

    /// Validate grid standards
    fn validate_grid_standards(&self, _transaction: &TransactionData) -> Result<()> {
        // Check if all grid standards are met
        for standard in &self.grid_standards.standards {
            if standard.compliance_level == StandardComplianceLevel::NonCompliant {
                return Err(Error::ERCComplianceViolation(
                    format!("Grid standard {} not met", standard.name)
                ));
            }
        }
        Ok(())
    }

    /// Update configuration
    pub fn update_config(&mut self, config_data: ConfigData) -> Result<()> {
        let new_config: ERCConfig = serde_json::from_value(config_data.data)?;
        self.config = new_config;
        Ok(())
    }

    /// Check if ERC compliance is met
    pub fn is_compliant(&self) -> bool {
        self.grid_code.status == ComplianceStatus::Compliant &&
        self.config.grid_code_compliance &&
        self.config.license_requirements &&
        self.config.safety_protocols &&
        self.config.grid_standards
    }
}

impl Default for ERCCompliance {
    fn default() -> Self {
        Self::new()
    }
}

/// Compliance status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplianceStatus {
    /// Compliant
    Compliant,
    /// Under review
    UnderReview,
    /// Non-compliant
    NonCompliant,
    /// Pending
    Pending,
}

/// Contact information (reused from SEC module)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    /// Email
    pub email: String,
    /// Phone
    pub phone: String,
    /// Website
    pub website: Option<String>,
}

/// Document (reused from SEC module)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Document ID
    pub id: String,
    /// Document type
    pub doc_type: String,
    /// Document name
    pub name: String,
    /// Upload date
    pub uploaded_at: DateTime<Utc>,
    /// Document hash
    pub hash: String,
}
