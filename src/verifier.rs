use crate::issuer::CertificateIssuer;
use crate::zk_proof::{ClaimType, ZkProofClaim};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerifierError {
    #[error("Invalid proof structure: {0}")]
    InvalidProof(String),
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
    #[error("Unsupported platform: {0}")]
    UnsupportedPlatform(String),
    #[error("Proof integrity check failed")]
    IntegrityCheckFailed,
    #[error("Circuit verification failed: {0}")]
    CircuitVerificationFailed(String),
}

/// Result of proof verification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationResult {
    /// Whether the proof is valid
    pub is_valid: bool,
    /// Whether the claimed requirements are met
    pub requirements_met: bool,
    /// Verification details
    pub details: VerificationDetails,
    /// Any warnings or additional information
    pub warnings: Vec<String>,
}

/// Detailed information about the verification process
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationDetails {
    /// Platform used for verification
    pub platform: String,
    /// Circuit ID that was verified
    pub circuit_id: String,
    /// Verification timestamp
    pub verified_at: chrono::DateTime<chrono::Utc>,
    /// Public inputs that were verified
    pub verified_inputs: HashMap<String, serde_json::Value>,
    /// Additional verification metadata
    pub metadata: HashMap<String, String>,
}

/// Zero-knowledge proof verifier
#[derive(Debug, Clone)]
pub struct ZkProofVerifier {
    /// Verifier identifier
    pub verifier_id: String,
    /// Supported platforms for verification
    pub supported_platforms: Vec<String>,
    /// Trusted circuit registry
    pub trusted_circuits: HashMap<String, CircuitInfo>,
}

/// Information about a trusted circuit
#[derive(Debug, Clone)]
pub struct CircuitInfo {
    /// Circuit identifier
    pub circuit_id: String,
    /// Circuit version
    pub version: String,
    /// Expected verification key hash
    pub vk_hash: String,
    /// Circuit description
    pub description: String,
}

impl ZkProofVerifier {
    /// Create a new verifier with default trusted circuits
    pub fn new(verifier_id: String) -> Self {
        let mut trusted_circuits = HashMap::new();

        // Add default trusted circuits using the same VK hash computation as issuer
        trusted_circuits.insert(
            "language_proficiency_v1".to_string(),
            CircuitInfo {
                circuit_id: "language_proficiency_v1".to_string(),
                version: "1.0.0".to_string(),
                vk_hash: CertificateIssuer::compute_verification_key_hash(
                    "language_proficiency_v1",
                ),
                description: "Verifies language proficiency level claims".to_string(),
            },
        );

        trusted_circuits.insert(
            "performance_threshold_v1".to_string(),
            CircuitInfo {
                circuit_id: "performance_threshold_v1".to_string(),
                version: "1.0.0".to_string(),
                vk_hash: CertificateIssuer::compute_verification_key_hash(
                    "performance_threshold_v1",
                ),
                description: "Verifies performance threshold claims".to_string(),
            },
        );

        trusted_circuits.insert(
            "completion_date_v1".to_string(),
            CircuitInfo {
                circuit_id: "completion_date_v1".to_string(),
                version: "1.0.0".to_string(),
                vk_hash: CertificateIssuer::compute_verification_key_hash("completion_date_v1"),
                description: "Verifies completion date claims".to_string(),
            },
        );

        trusted_circuits.insert(
            "combined_criteria_v1".to_string(),
            CircuitInfo {
                circuit_id: "combined_criteria_v1".to_string(),
                version: "1.0.0".to_string(),
                vk_hash: CertificateIssuer::compute_verification_key_hash("combined_criteria_v1"),
                description: "Verifies combined criteria claims".to_string(),
            },
        );

        Self {
            verifier_id,
            supported_platforms: vec!["aleo".to_string(), "stylus".to_string(), "test".to_string()],
            trusted_circuits,
        }
    }

    /// Verify a zero-knowledge proof claim
    pub fn verify_proof(&self, proof: &ZkProofClaim) -> Result<VerificationResult, VerifierError> {
        // Step 1: Basic integrity checks
        if !proof.verify_integrity() {
            return Err(VerifierError::IntegrityCheckFailed);
        }

        // Step 2: Platform support check
        if !self.supported_platforms.contains(&proof.metadata.platform) {
            return Err(VerifierError::UnsupportedPlatform(
                proof.metadata.platform.clone(),
            ));
        }

        // Step 3: Circuit verification
        self.verify_circuit(&proof.proof_data.circuit_id, &proof.proof_data.vk_hash)?;

        // Step 4: Proof verification based on claim type
        let verification_result = match &proof.claim_type {
            ClaimType::LanguageProficiency {
                language,
                min_level,
            } => self.verify_language_proficiency_claim(proof, language, min_level)?,
            ClaimType::PerformanceThreshold { min_percentage } => {
                self.verify_performance_threshold_claim(proof, *min_percentage)?
            }
            ClaimType::CompletionDate { after_date } => {
                self.verify_completion_date_claim(proof, after_date)?
            }
            ClaimType::Combined { criteria } => self.verify_combined_claim(proof, criteria)?,
        };

        Ok(verification_result)
    }

    /// Verify circuit integrity and trust
    fn verify_circuit(
        &self,
        circuit_id: &str,
        provided_vk_hash: &str,
    ) -> Result<(), VerifierError> {
        let circuit_info = self.trusted_circuits.get(circuit_id).ok_or_else(|| {
            VerifierError::CircuitVerificationFailed(format!("Unknown circuit: {}", circuit_id))
        })?;

        // Note: In a real implementation, we would need to verify the VK hash properly
        // For this demo, we'll do a simplified check
        if circuit_info.vk_hash != provided_vk_hash {
            return Err(VerifierError::CircuitVerificationFailed(format!(
                "Verification key hash mismatch for circuit '{}': expected '{}', got '{}'",
                circuit_id, circuit_info.vk_hash, provided_vk_hash
            )));
        }

        Ok(())
    }

    /// Verify language proficiency claim
    fn verify_language_proficiency_claim(
        &self,
        proof: &ZkProofClaim,
        _language: &str,
        _min_level: &crate::zk_proof::CefrLevel,
    ) -> Result<VerificationResult, VerifierError> {
        // In a real implementation, this would verify the actual cryptographic proof
        // For this demo, we simulate the verification process

        let is_valid = self.simulate_proof_verification(&proof.proof_data.proof_bytes);
        let requirements_met = proof.public_inputs.verification_result;

        let details = VerificationDetails {
            platform: proof.metadata.platform.clone(),
            circuit_id: proof.proof_data.circuit_id.clone(),
            verified_at: chrono::Utc::now(),
            verified_inputs: proof.public_inputs.requirements.clone(),
            metadata: proof.metadata.properties.clone(),
        };

        let warnings = if !requirements_met {
            vec!["Language proficiency requirements not met".to_string()]
        } else {
            vec![]
        };

        Ok(VerificationResult {
            is_valid,
            requirements_met,
            details,
            warnings,
        })
    }

    /// Verify performance threshold claim
    fn verify_performance_threshold_claim(
        &self,
        proof: &ZkProofClaim,
        _min_percentage: u8,
    ) -> Result<VerificationResult, VerifierError> {
        let is_valid = self.simulate_proof_verification(&proof.proof_data.proof_bytes);
        let requirements_met = proof.public_inputs.verification_result;

        let details = VerificationDetails {
            platform: proof.metadata.platform.clone(),
            circuit_id: proof.proof_data.circuit_id.clone(),
            verified_at: chrono::Utc::now(),
            verified_inputs: proof.public_inputs.requirements.clone(),
            metadata: proof.metadata.properties.clone(),
        };

        let warnings = if !requirements_met {
            vec!["Performance threshold not met".to_string()]
        } else {
            vec![]
        };

        Ok(VerificationResult {
            is_valid,
            requirements_met,
            details,
            warnings,
        })
    }

    /// Verify completion date claim
    fn verify_completion_date_claim(
        &self,
        proof: &ZkProofClaim,
        _after_date: &chrono::DateTime<chrono::Utc>,
    ) -> Result<VerificationResult, VerifierError> {
        let is_valid = self.simulate_proof_verification(&proof.proof_data.proof_bytes);
        let requirements_met = proof.public_inputs.verification_result;

        let details = VerificationDetails {
            platform: proof.metadata.platform.clone(),
            circuit_id: proof.proof_data.circuit_id.clone(),
            verified_at: chrono::Utc::now(),
            verified_inputs: proof.public_inputs.requirements.clone(),
            metadata: proof.metadata.properties.clone(),
        };

        let warnings = if !requirements_met {
            vec!["Completion date requirements not met".to_string()]
        } else {
            vec![]
        };

        Ok(VerificationResult {
            is_valid,
            requirements_met,
            details,
            warnings,
        })
    }

    /// Verify combined claim
    fn verify_combined_claim(
        &self,
        proof: &ZkProofClaim,
        _criteria: &[ClaimType],
    ) -> Result<VerificationResult, VerifierError> {
        let is_valid = self.simulate_proof_verification(&proof.proof_data.proof_bytes);
        let requirements_met = proof.public_inputs.verification_result;

        let details = VerificationDetails {
            platform: proof.metadata.platform.clone(),
            circuit_id: proof.proof_data.circuit_id.clone(),
            verified_at: chrono::Utc::now(),
            verified_inputs: proof.public_inputs.requirements.clone(),
            metadata: proof.metadata.properties.clone(),
        };

        let warnings = if !requirements_met {
            vec!["Combined criteria requirements not met".to_string()]
        } else {
            vec![]
        };

        Ok(VerificationResult {
            is_valid,
            requirements_met,
            details,
            warnings,
        })
    }

    /// Simulate proof verification (replace with real cryptographic verification)
    fn simulate_proof_verification(&self, proof_bytes: &[u8]) -> bool {
        // In a real implementation, this would perform actual ZK proof verification
        // For this demo, we simulate by checking if proof bytes are non-empty and have valid structure
        !proof_bytes.is_empty() && proof_bytes.len() >= 32
    }

    /// Compute expected verification key hash for a circuit (deprecated - use CertificateIssuer method)
    #[deprecated(note = "Use CertificateIssuer::compute_verification_key_hash instead")]
    fn compute_expected_vk_hash(circuit_id: &str) -> String {
        CertificateIssuer::compute_verification_key_hash(circuit_id)
    }

    /// Add a trusted circuit to the verifier
    pub fn add_trusted_circuit(&mut self, circuit_info: CircuitInfo) {
        self.trusted_circuits
            .insert(circuit_info.circuit_id.clone(), circuit_info);
    }

    /// Check if a circuit is trusted
    pub fn is_circuit_trusted(&self, circuit_id: &str) -> bool {
        self.trusted_circuits.contains_key(circuit_id)
    }

    /// Get information about a trusted circuit
    pub fn get_circuit_info(&self, circuit_id: &str) -> Option<&CircuitInfo> {
        self.trusted_circuits.get(circuit_id)
    }

    /// List all trusted circuit IDs
    pub fn list_trusted_circuits(&self) -> Vec<String> {
        self.trusted_circuits.keys().cloned().collect()
    }

    /// Get verification statistics
    pub fn get_verification_stats(&self) -> VerificationStats {
        VerificationStats {
            supported_platforms: self.supported_platforms.len(),
            trusted_circuits: self.trusted_circuits.len(),
            verifier_id: self.verifier_id.clone(),
        }
    }
}

/// Statistics about the verifier's capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationStats {
    /// Number of supported platforms
    pub supported_platforms: usize,
    /// Number of trusted circuits
    pub trusted_circuits: usize,
    /// Verifier identifier
    pub verifier_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::issuer::{CertificateIssuer, ProofOptions, ProofRequest};
    use crate::zk_proof::{CefrLevel, ClaimType};
    use chrono::Utc;
    use konnektoren_core::certificates::CertificateData;

    fn create_test_verifier() -> ZkProofVerifier {
        ZkProofVerifier::new("test_verifier_001".to_string())
    }

    fn create_test_proof() -> ZkProofClaim {
        let issuer = CertificateIssuer::new("test_issuer".to_string(), "Test Issuer".to_string());

        let certificate = CertificateData::new(
            "German_B2_Complete".to_string(),
            50,
            47,
            "Test Student".to_string(),
            Utc::now(),
        );

        let request = ProofRequest {
            certificate,
            claim_type: ClaimType::LanguageProficiency {
                language: "German".to_string(),
                min_level: CefrLevel::B1,
            },
            target_platform: "test".to_string(),
            options: ProofOptions::default(),
        };

        issuer.generate_proof(request).unwrap()
    }

    #[test]
    fn test_verifier_creation() {
        let verifier = create_test_verifier();
        assert!(!verifier.verifier_id.is_empty());
        assert!(!verifier.trusted_circuits.is_empty());
        assert!(verifier.is_circuit_trusted("language_proficiency_v1"));
    }

    #[test]
    fn test_proof_verification_success() {
        let verifier = create_test_verifier();
        let proof = create_test_proof();

        let result = verifier.verify_proof(&proof);
        assert!(result.is_ok());

        let verification_result = result.unwrap();
        assert!(verification_result.is_valid);
        assert!(verification_result.requirements_met);
        assert!(verification_result.warnings.is_empty());
    }

    #[test]
    fn test_unsupported_platform() {
        let verifier = create_test_verifier();
        let mut proof = create_test_proof();
        proof.metadata.platform = "unsupported_platform".to_string();

        let result = verifier.verify_proof(&proof);
        assert!(result.is_err());

        match result.unwrap_err() {
            VerifierError::UnsupportedPlatform(platform) => {
                assert_eq!(platform, "unsupported_platform");
            }
            _ => panic!("Expected UnsupportedPlatform error"),
        }
    }

    #[test]
    fn test_circuit_verification_failure() {
        let verifier = create_test_verifier();
        let mut proof = create_test_proof();
        proof.proof_data.vk_hash = "invalid_hash".to_string();

        let result = verifier.verify_proof(&proof);
        assert!(result.is_err());

        match result.unwrap_err() {
            VerifierError::CircuitVerificationFailed(_) => {
                // Expected
            }
            _ => panic!("Expected CircuitVerificationFailed error"),
        }
    }

    #[test]
    fn test_unknown_circuit() {
        let verifier = create_test_verifier();
        let mut proof = create_test_proof();
        proof.proof_data.circuit_id = "unknown_circuit".to_string();

        let result = verifier.verify_proof(&proof);
        assert!(result.is_err());

        match result.unwrap_err() {
            VerifierError::CircuitVerificationFailed(msg) => {
                assert!(msg.contains("Unknown circuit"));
            }
            _ => panic!("Expected CircuitVerificationFailed error"),
        }
    }

    #[test]
    fn test_performance_threshold_verification() {
        let issuer = CertificateIssuer::new("test_issuer".to_string(), "Test Issuer".to_string());

        let certificate = CertificateData::new(
            "Spanish_B2_Complete".to_string(),
            50,
            45, // 90% performance
            "Test Student".to_string(),
            Utc::now(),
        );

        let request = ProofRequest {
            certificate,
            claim_type: ClaimType::PerformanceThreshold { min_percentage: 85 },
            target_platform: "test".to_string(),
            options: ProofOptions::default(),
        };

        let proof = issuer.generate_proof(request).unwrap();
        let verifier = create_test_verifier();

        let result = verifier.verify_proof(&proof).unwrap();
        assert!(result.is_valid);
        assert!(result.requirements_met);
    }

    #[test]
    fn test_add_trusted_circuit() {
        let mut verifier = create_test_verifier();

        let new_circuit = CircuitInfo {
            circuit_id: "custom_circuit_v1".to_string(),
            version: "1.0.0".to_string(),
            vk_hash: "custom_hash".to_string(),
            description: "Custom circuit for testing".to_string(),
        };

        verifier.add_trusted_circuit(new_circuit);
        assert!(verifier.is_circuit_trusted("custom_circuit_v1"));
    }

    #[test]
    fn test_circuit_info_retrieval() {
        let verifier = create_test_verifier();

        let circuit_info = verifier.get_circuit_info("language_proficiency_v1");
        assert!(circuit_info.is_some());

        let info = circuit_info.unwrap();
        assert_eq!(info.circuit_id, "language_proficiency_v1");
        assert_eq!(info.version, "1.0.0");
        assert!(!info.vk_hash.is_empty());
        assert!(!info.description.is_empty());
    }

    #[test]
    fn test_list_trusted_circuits() {
        let verifier = create_test_verifier();
        let circuits = verifier.list_trusted_circuits();

        assert!(circuits.contains(&"language_proficiency_v1".to_string()));
        assert!(circuits.contains(&"performance_threshold_v1".to_string()));
        assert!(circuits.contains(&"completion_date_v1".to_string()));
        assert!(circuits.contains(&"combined_criteria_v1".to_string()));
    }

    #[test]
    fn test_verification_stats() {
        let verifier = create_test_verifier();
        let stats = verifier.get_verification_stats();

        assert_eq!(stats.verifier_id, "test_verifier_001");
        assert_eq!(stats.supported_platforms, 3); // aleo, stylus, test
        assert_eq!(stats.trusted_circuits, 4); // 4 default circuits
    }

    #[test]
    fn test_vk_hash_consistency() {
        let verifier1 = create_test_verifier();
        let verifier2 = create_test_verifier();

        let circuit1 = verifier1
            .get_circuit_info("language_proficiency_v1")
            .unwrap();
        let circuit2 = verifier2
            .get_circuit_info("language_proficiency_v1")
            .unwrap();

        assert_eq!(circuit1.vk_hash, circuit2.vk_hash);
    }

    #[test]
    fn test_end_to_end_verification() {
        // Create issuer and generate proof
        let issuer = CertificateIssuer::new("e2e_issuer".to_string(), "E2E Test".to_string());
        let certificate = CertificateData::new(
            "French_C1_Advanced".to_string(),
            60,
            58,
            "Advanced Student".to_string(),
            Utc::now(),
        );

        let request = ProofRequest {
            certificate,
            claim_type: ClaimType::Combined {
                criteria: vec![
                    ClaimType::LanguageProficiency {
                        language: "French".to_string(),
                        min_level: CefrLevel::B2,
                    },
                    ClaimType::PerformanceThreshold { min_percentage: 90 },
                ],
            },
            target_platform: "aleo".to_string(),
            options: ProofOptions::default(),
        };

        let proof = issuer.generate_proof(request).unwrap();

        // Verify with verifier
        let verifier = create_test_verifier();
        let verification_result = verifier.verify_proof(&proof).unwrap();

        assert!(verification_result.is_valid);
        assert!(verification_result.requirements_met);
        assert_eq!(verification_result.details.platform, "aleo");
        assert_eq!(
            verification_result.details.circuit_id,
            "combined_criteria_v1"
        );
    }
}
