use crate::zk_proof::{CefrLevel, ClaimType, ProofData, ProofMetadata, PublicInputs, ZkProofClaim};
use chrono::{DateTime, Utc};
use konnektoren_core::certificates::CertificateData;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IssuerError {
    #[error("Invalid certificate data: {0}")]
    InvalidCertificate(String),
    #[error("Insufficient performance: required {required}, got {actual}")]
    InsufficientPerformance { required: u8, actual: u8 },
    #[error("Invalid CEFR level: {0}")]
    InvalidCefrLevel(String),
    #[error("Proof generation failed: {0}")]
    ProofGenerationFailed(String),
    #[error("Invalid claim type for certificate")]
    InvalidClaimType,
}

/// Certificate issuer that can generate ZK proofs from language learning certificates
#[derive(Debug, Clone)]
pub struct CertificateIssuer {
    /// Issuer identifier
    pub issuer_id: String,
    /// Issuer name
    pub issuer_name: String,
    /// Supported platforms for proof generation
    pub supported_platforms: Vec<String>,
}

/// Request for generating a ZK proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofRequest {
    /// The certificate to generate proof from
    pub certificate: CertificateData,
    /// Type of claim to prove
    pub claim_type: ClaimType,
    /// Target platform (aleo, stylus, etc.)
    pub target_platform: String,
    /// Additional options
    pub options: ProofOptions,
}

/// Options for proof generation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProofOptions {
    /// Whether to include performance details
    pub include_performance_range: bool,
    /// Whether to include completion date
    pub include_completion_date: bool,
    /// Custom properties to include
    pub custom_properties: HashMap<String, String>,
}

impl CertificateIssuer {
    /// Create a new certificate issuer
    pub fn new(issuer_id: String, issuer_name: String) -> Self {
        Self {
            issuer_id,
            issuer_name,
            supported_platforms: vec!["aleo".to_string(), "stylus".to_string(), "test".to_string()],
        }
    }

    /// Generate a ZK proof from a certificate
    pub fn generate_proof(&self, request: ProofRequest) -> Result<ZkProofClaim, IssuerError> {
        // Validate the certificate
        self.validate_certificate(&request.certificate)?;

        // Validate the request
        self.validate_request(&request)?;

        // Generate the proof based on claim type
        match &request.claim_type {
            ClaimType::LanguageProficiency {
                language,
                min_level,
            } => self.generate_language_proficiency_proof(
                &request.certificate,
                language,
                min_level,
                &request,
            ),
            ClaimType::PerformanceThreshold { min_percentage } => {
                self.generate_performance_proof(&request.certificate, *min_percentage, &request)
            }
            ClaimType::CompletionDate { after_date } => {
                self.generate_completion_date_proof(&request.certificate, after_date, &request)
            }
            ClaimType::Combined { criteria } => {
                self.generate_combined_proof(&request.certificate, criteria, &request)
            }
        }
    }

    /// Validate a certificate for proof generation
    fn validate_certificate(&self, certificate: &CertificateData) -> Result<(), IssuerError> {
        if certificate.profile_name.is_empty() {
            return Err(IssuerError::InvalidCertificate(
                "Profile name cannot be empty".to_string(),
            ));
        }

        if certificate.game_path_name.is_empty() {
            return Err(IssuerError::InvalidCertificate(
                "Game path name cannot be empty".to_string(),
            ));
        }

        if certificate.total_challenges == 0 {
            return Err(IssuerError::InvalidCertificate(
                "Total challenges must be greater than 0".to_string(),
            ));
        }

        // Verify certificate signature if present
        if let Err(_) = certificate.verify() {
            return Err(IssuerError::InvalidCertificate(
                "Certificate signature verification failed".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate a proof request
    fn validate_request(&self, request: &ProofRequest) -> Result<(), IssuerError> {
        if !self.supported_platforms.contains(&request.target_platform) {
            return Err(IssuerError::InvalidClaimType);
        }
        Ok(())
    }

    /// Generate proof for language proficiency claim
    fn generate_language_proficiency_proof(
        &self,
        certificate: &CertificateData,
        language: &str,
        min_level: &CefrLevel,
        request: &ProofRequest,
    ) -> Result<ZkProofClaim, IssuerError> {
        // Extract language from certificate
        let cert_language = self.extract_language_from_certificate(certificate)?;
        if !cert_language.eq_ignore_ascii_case(language) {
            return Err(IssuerError::InvalidCertificate(format!(
                "Certificate language {} does not match requested language {}",
                cert_language, language
            )));
        }

        // Extract CEFR level from certificate
        let cert_level = CefrLevel::from_course_name(&certificate.game_path_name)
            .ok_or_else(|| IssuerError::InvalidCefrLevel(certificate.game_path_name.clone()))?;

        // Check if certificate meets minimum level requirement
        let meets_requirement = cert_level >= *min_level;

        // Create public inputs
        let mut requirements = HashMap::new();
        requirements.insert(
            "min_level".to_string(),
            serde_json::Value::String(min_level.to_string()),
        );
        requirements.insert(
            "language".to_string(),
            serde_json::Value::String(language.to_string()),
        );

        let public_inputs = PublicInputs {
            requirements,
            verification_result: meets_requirement,
            certificate_hash: self.get_certificate_hash(certificate),
        };

        // Generate simulated proof data
        let proof_data = self.generate_proof_data("language_proficiency_v1")?;

        // Create metadata
        let metadata = self.create_metadata(&request.target_platform, &request.options);

        Ok(ZkProofClaim::new(
            request.claim_type.clone(),
            public_inputs,
            proof_data,
            metadata,
        ))
    }

    /// Generate proof for performance threshold claim
    fn generate_performance_proof(
        &self,
        certificate: &CertificateData,
        min_percentage: u8,
        request: &ProofRequest,
    ) -> Result<ZkProofClaim, IssuerError> {
        let meets_requirement = certificate.performance_percentage >= min_percentage;

        if !meets_requirement {
            return Err(IssuerError::InsufficientPerformance {
                required: min_percentage,
                actual: certificate.performance_percentage,
            });
        }

        let mut requirements = HashMap::new();
        requirements.insert(
            "min_percentage".to_string(),
            serde_json::Value::Number(min_percentage.into()),
        );

        let public_inputs = PublicInputs {
            requirements,
            verification_result: meets_requirement,
            certificate_hash: self.get_certificate_hash(certificate),
        };

        let proof_data = self.generate_proof_data("performance_threshold_v1")?;
        let metadata = self.create_metadata(&request.target_platform, &request.options);

        Ok(ZkProofClaim::new(
            request.claim_type.clone(),
            public_inputs,
            proof_data,
            metadata,
        ))
    }

    /// Generate proof for completion date claim
    fn generate_completion_date_proof(
        &self,
        certificate: &CertificateData,
        after_date: &DateTime<Utc>,
        request: &ProofRequest,
    ) -> Result<ZkProofClaim, IssuerError> {
        let meets_requirement = certificate.date >= *after_date;

        let mut requirements = HashMap::new();
        requirements.insert(
            "after_date".to_string(),
            serde_json::Value::String(after_date.to_rfc3339()),
        );

        let public_inputs = PublicInputs {
            requirements,
            verification_result: meets_requirement,
            certificate_hash: self.get_certificate_hash(certificate),
        };

        let proof_data = self.generate_proof_data("completion_date_v1")?;
        let metadata = self.create_metadata(&request.target_platform, &request.options);

        Ok(ZkProofClaim::new(
            request.claim_type.clone(),
            public_inputs,
            proof_data,
            metadata,
        ))
    }

    /// Generate proof for combined claims
    fn generate_combined_proof(
        &self,
        certificate: &CertificateData,
        criteria: &[ClaimType],
        request: &ProofRequest,
    ) -> Result<ZkProofClaim, IssuerError> {
        // Validate each criterion
        let mut all_requirements = HashMap::new();
        let mut all_pass = true;

        for (i, criterion) in criteria.iter().enumerate() {
            let individual_request = ProofRequest {
                certificate: certificate.clone(),
                claim_type: criterion.clone(),
                target_platform: request.target_platform.clone(),
                options: request.options.clone(),
            };

            match self.generate_proof(individual_request) {
                Ok(proof) => {
                    all_pass &= proof.public_inputs.verification_result;
                    // Merge requirements with prefix
                    for (key, value) in proof.public_inputs.requirements {
                        all_requirements.insert(format!("criterion_{}_{}", i, key), value);
                    }
                }
                Err(_) => {
                    all_pass = false;
                }
            }
        }

        let public_inputs = PublicInputs {
            requirements: all_requirements,
            verification_result: all_pass,
            certificate_hash: self.get_certificate_hash(certificate),
        };

        let proof_data = self.generate_proof_data("combined_criteria_v1")?;
        let metadata = self.create_metadata(&request.target_platform, &request.options);

        Ok(ZkProofClaim::new(
            request.claim_type.clone(),
            public_inputs,
            proof_data,
            metadata,
        ))
    }

    /// Extract language from certificate path name
    fn extract_language_from_certificate(
        &self,
        certificate: &CertificateData,
    ) -> Result<String, IssuerError> {
        let parts: Vec<&str> = certificate.game_path_name.split('_').collect();
        if let Some(language) = parts.first() {
            Ok(language.to_string())
        } else {
            Err(IssuerError::InvalidCertificate(
                "Cannot extract language from certificate".to_string(),
            ))
        }
    }

    /// Generate simulated proof data
    fn generate_proof_data(&self, circuit_id: &str) -> Result<ProofData, IssuerError> {
        // In a real implementation, this would generate actual cryptographic proofs
        let proof_bytes = self.simulate_proof_generation(circuit_id);

        Ok(ProofData {
            proof_bytes,
            circuit_id: circuit_id.to_string(),
            vk_hash: Self::compute_verification_key_hash(circuit_id),
        })
    }

    /// Simulate proof generation (replace with real ZK proof generation)
    fn simulate_proof_generation(&self, circuit_id: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(circuit_id.as_bytes());
        hasher.update(self.issuer_id.as_bytes());
        hasher.update(Utc::now().timestamp().to_string().as_bytes());
        hasher.finalize().to_vec()
    }

    /// Compute verification key hash for a circuit (must match verifier's computation)
    pub fn compute_verification_key_hash(circuit_id: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"vk_");
        hasher.update(circuit_id.as_bytes());
        hasher.update(b"test"); // Use consistent platform for VK hash
        hex::encode(hasher.finalize())
    }

    /// Create proof metadata
    fn create_metadata(&self, platform: &str, options: &ProofOptions) -> ProofMetadata {
        let mut properties = HashMap::new();
        properties.insert("issuer_id".to_string(), self.issuer_id.clone());
        properties.insert("issuer_name".to_string(), self.issuer_name.clone());

        // Add custom properties from options
        for (key, value) in &options.custom_properties {
            properties.insert(key.clone(), value.clone());
        }

        ProofMetadata {
            version: "1.0.0".to_string(),
            platform: platform.to_string(),
            properties,
        }
    }

    /// Get hash of certificate for integrity verification
    fn get_certificate_hash(&self, certificate: &CertificateData) -> String {
        let mut hasher = Sha256::new();
        hasher.update(certificate.to_base64().as_bytes());
        hex::encode(hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_certificate() -> CertificateData {
        CertificateData::new(
            "German_B2_Complete".to_string(),
            50,
            47,
            "Test Student".to_string(),
            Utc::now(),
        )
    }

    fn create_test_issuer() -> CertificateIssuer {
        CertificateIssuer::new(
            "test_issuer_001".to_string(),
            "Web5 Claims Test Issuer".to_string(),
        )
    }

    #[test]
    fn test_certificate_validation() {
        let issuer = create_test_issuer();
        let certificate = create_test_certificate();

        assert!(issuer.validate_certificate(&certificate).is_ok());
    }

    #[test]
    fn test_invalid_certificate_validation() {
        let issuer = create_test_issuer();
        let mut certificate = create_test_certificate();
        certificate.profile_name = String::new();

        assert!(issuer.validate_certificate(&certificate).is_err());
    }

    #[test]
    fn test_language_proficiency_proof() {
        let issuer = create_test_issuer();
        let certificate = create_test_certificate();

        let request = ProofRequest {
            certificate,
            claim_type: ClaimType::LanguageProficiency {
                language: "German".to_string(),
                min_level: CefrLevel::B1,
            },
            target_platform: "test".to_string(),
            options: ProofOptions::default(),
        };

        let result = issuer.generate_proof(request);
        assert!(result.is_ok());

        let proof = result.unwrap();
        assert!(proof.public_inputs.verification_result);
        assert_eq!(proof.metadata.platform, "test");
    }

    #[test]
    fn test_performance_threshold_proof() {
        let issuer = create_test_issuer();
        let certificate = create_test_certificate();

        let request = ProofRequest {
            certificate,
            claim_type: ClaimType::PerformanceThreshold { min_percentage: 90 },
            target_platform: "aleo".to_string(),
            options: ProofOptions::default(),
        };

        let result = issuer.generate_proof(request);
        assert!(result.is_ok());

        let proof = result.unwrap();
        assert!(proof.public_inputs.verification_result);
    }

    #[test]
    fn test_insufficient_performance() {
        let issuer = create_test_issuer();
        let certificate = create_test_certificate();

        let request = ProofRequest {
            certificate,
            claim_type: ClaimType::PerformanceThreshold { min_percentage: 99 },
            target_platform: "test".to_string(),
            options: ProofOptions::default(),
        };

        let result = issuer.generate_proof(request);
        assert!(result.is_err());

        match result.unwrap_err() {
            IssuerError::InsufficientPerformance { required, actual } => {
                assert_eq!(required, 99);
                assert_eq!(actual, 94); // From create_test_certificate
            }
            _ => panic!("Expected InsufficientPerformance error"),
        }
    }

    #[test]
    fn test_completion_date_proof() {
        let issuer = create_test_issuer();
        let certificate = create_test_certificate();

        // Test with a date in the past (should pass)
        let past_date = Utc::now() - chrono::Duration::days(30);

        let request = ProofRequest {
            certificate,
            claim_type: ClaimType::CompletionDate {
                after_date: past_date,
            },
            target_platform: "stylus".to_string(),
            options: ProofOptions::default(),
        };

        let result = issuer.generate_proof(request);
        assert!(result.is_ok());

        let proof = result.unwrap();
        assert!(proof.public_inputs.verification_result);
    }

    #[test]
    fn test_combined_proof() {
        let issuer = create_test_issuer();
        let certificate = create_test_certificate();

        let criteria = vec![
            ClaimType::LanguageProficiency {
                language: "German".to_string(),
                min_level: CefrLevel::B1,
            },
            ClaimType::PerformanceThreshold { min_percentage: 90 },
        ];

        let request = ProofRequest {
            certificate,
            claim_type: ClaimType::Combined { criteria },
            target_platform: "aleo".to_string(),
            options: ProofOptions::default(),
        };

        let result = issuer.generate_proof(request);
        assert!(result.is_ok());

        let proof = result.unwrap();
        assert!(proof.public_inputs.verification_result);
    }

    #[test]
    fn test_proof_integrity() {
        let issuer = create_test_issuer();
        let certificate = create_test_certificate();

        let request = ProofRequest {
            certificate,
            claim_type: ClaimType::PerformanceThreshold { min_percentage: 80 },
            target_platform: "test".to_string(),
            options: ProofOptions::default(),
        };

        let proof = issuer.generate_proof(request).unwrap();
        assert!(proof.verify_integrity());
        assert!(!proof.get_proof_hash().is_empty());
    }

    #[test]
    fn test_vk_hash_consistency() {
        let circuit_id = "test_circuit_v1";
        let hash1 = CertificateIssuer::compute_verification_key_hash(circuit_id);
        let hash2 = CertificateIssuer::compute_verification_key_hash(circuit_id);
        assert_eq!(hash1, hash2);
        assert!(!hash1.is_empty());
    }
}
