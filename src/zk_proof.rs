use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Represents a zero-knowledge proof claim about language learning achievements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ZkProofClaim {
    /// Unique identifier for this proof
    pub proof_id: String,
    /// The type of claim being made
    pub claim_type: ClaimType,
    /// Public inputs that can be verified
    pub public_inputs: PublicInputs,
    /// The actual ZK proof data (in a real implementation, this would be cryptographic proof)
    pub proof_data: ProofData,
    /// Timestamp when the proof was generated
    pub generated_at: DateTime<Utc>,
    /// Proof metadata
    pub metadata: ProofMetadata,
}

/// Types of claims that can be made with zero-knowledge proofs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClaimType {
    /// Prove language proficiency level without revealing exact scores
    LanguageProficiency {
        language: String,
        min_level: CefrLevel,
    },
    /// Prove minimum performance threshold was met
    PerformanceThreshold { min_percentage: u8 },
    /// Prove completion date is after a certain date
    CompletionDate { after_date: DateTime<Utc> },
    /// Prove multiple criteria simultaneously
    Combined { criteria: Vec<ClaimType> },
}

/// CEFR (Common European Framework of Reference) levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub enum CefrLevel {
    A1 = 1,
    A2 = 2,
    B1 = 3,
    B2 = 4,
    C1 = 5,
    C2 = 6,
}

/// Public inputs that are revealed as part of the proof
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicInputs {
    /// Minimum requirements that were checked
    pub requirements: HashMap<String, serde_json::Value>,
    /// Verification result (true if requirements are met)
    pub verification_result: bool,
    /// Hash of the certificate data (for integrity)
    pub certificate_hash: String,
}

/// The actual proof data (simplified for this demo)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofData {
    /// Simulated ZK proof bytes (in real implementation, this would be actual cryptographic proof)
    pub proof_bytes: Vec<u8>,
    /// Circuit identifier used for this proof
    pub circuit_id: String,
    /// Verification key hash
    pub vk_hash: String,
}

/// Additional metadata about the proof
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofMetadata {
    /// Version of the proof system used
    pub version: String,
    /// Target platform (e.g., "aleo", "stylus")
    pub platform: String,
    /// Additional properties
    pub properties: HashMap<String, String>,
}

impl ZkProofClaim {
    /// Create a new ZK proof claim
    pub fn new(
        claim_type: ClaimType,
        public_inputs: PublicInputs,
        proof_data: ProofData,
        metadata: ProofMetadata,
    ) -> Self {
        Self {
            proof_id: uuid::Uuid::new_v4().to_string(),
            claim_type,
            public_inputs,
            proof_data,
            generated_at: Utc::now(),
            metadata,
        }
    }

    /// Verify the integrity of this proof claim
    pub fn verify_integrity(&self) -> bool {
        // Basic integrity checks
        !self.proof_id.is_empty()
            && !self.public_inputs.certificate_hash.is_empty()
            && !self.proof_data.proof_bytes.is_empty()
            && !self.proof_data.circuit_id.is_empty()
    }

    /// Get a hash of this proof for uniqueness verification
    pub fn get_proof_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(self).unwrap_or_default().as_bytes());
        hex::encode(hasher.finalize())
    }
}

impl CefrLevel {
    /// Convert from string representation
    pub fn from_course_name(course_name: &str) -> Option<CefrLevel> {
        if course_name.contains("A1") {
            Some(CefrLevel::A1)
        } else if course_name.contains("A2") {
            Some(CefrLevel::A2)
        } else if course_name.contains("B1") {
            Some(CefrLevel::B1)
        } else if course_name.contains("B2") {
            Some(CefrLevel::B2)
        } else if course_name.contains("C1") {
            Some(CefrLevel::C1)
        } else if course_name.contains("C2") {
            Some(CefrLevel::C2)
        } else {
            None
        }
    }

    /// Convert to numeric value
    pub fn to_numeric(&self) -> u8 {
        match self {
            CefrLevel::A1 => 1,
            CefrLevel::A2 => 2,
            CefrLevel::B1 => 3,
            CefrLevel::B2 => 4,
            CefrLevel::C1 => 5,
            CefrLevel::C2 => 6,
        }
    }
}

impl std::fmt::Display for CefrLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CefrLevel::A1 => write!(f, "A1"),
            CefrLevel::A2 => write!(f, "A2"),
            CefrLevel::B1 => write!(f, "B1"),
            CefrLevel::B2 => write!(f, "B2"),
            CefrLevel::C1 => write!(f, "C1"),
            CefrLevel::C2 => write!(f, "C2"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_cefr_level_conversion() {
        assert_eq!(
            CefrLevel::from_course_name("German_B2_Complete"),
            Some(CefrLevel::B2)
        );
        assert_eq!(
            CefrLevel::from_course_name("Spanish_A1_Basic"),
            Some(CefrLevel::A1)
        );
        assert_eq!(CefrLevel::from_course_name("Invalid_Course"), None);
    }

    #[test]
    fn test_cefr_level_numeric() {
        assert_eq!(CefrLevel::B2.to_numeric(), 4);
        assert_eq!(CefrLevel::A1.to_numeric(), 1);
        assert_eq!(CefrLevel::C2.to_numeric(), 6);
    }

    #[test]
    fn test_cefr_level_ordering() {
        assert!(CefrLevel::B2 > CefrLevel::B1);
        assert!(CefrLevel::A1 < CefrLevel::A2);
        assert!(CefrLevel::C2 > CefrLevel::C1);
    }

    #[test]
    fn test_zk_proof_claim_creation() {
        let claim_type = ClaimType::LanguageProficiency {
            language: "German".to_string(),
            min_level: CefrLevel::B2,
        };

        let public_inputs = PublicInputs {
            requirements: HashMap::new(),
            verification_result: true,
            certificate_hash: "test_hash".to_string(),
        };

        let proof_data = ProofData {
            proof_bytes: vec![1, 2, 3, 4],
            circuit_id: "language_verification_v1".to_string(),
            vk_hash: "vk_hash_123".to_string(),
        };

        let metadata = ProofMetadata {
            version: "1.0.0".to_string(),
            platform: "aleo".to_string(),
            properties: HashMap::new(),
        };

        let claim = ZkProofClaim::new(claim_type, public_inputs, proof_data, metadata);

        assert!(!claim.proof_id.is_empty());
        assert!(claim.verify_integrity());
        assert!(!claim.get_proof_hash().is_empty());
    }

    #[test]
    fn test_proof_verification() {
        let mut claim = create_test_claim();
        assert!(claim.verify_integrity());

        // Test with empty proof_id
        claim.proof_id = String::new();
        assert!(!claim.verify_integrity());
    }

    fn create_test_claim() -> ZkProofClaim {
        ZkProofClaim::new(
            ClaimType::PerformanceThreshold { min_percentage: 90 },
            PublicInputs {
                requirements: HashMap::new(),
                verification_result: true,
                certificate_hash: "hash123".to_string(),
            },
            ProofData {
                proof_bytes: vec![1, 2, 3],
                circuit_id: "test_circuit".to_string(),
                vk_hash: "vk123".to_string(),
            },
            ProofMetadata {
                version: "1.0.0".to_string(),
                platform: "test".to_string(),
                properties: HashMap::new(),
            },
        )
    }
}
