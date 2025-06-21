//! # Web5 Claims
//!
//! A zero-knowledge verifiable credential system for language learning achievements.
//!
//! This crate provides functionality to:
//! - Issue verifiable credentials for language learning certificates
//! - Generate zero-knowledge proofs about language proficiency without revealing sensitive data
//! - Verify ZK proofs to establish trust in claimed achievements
//!
//! ## Example Usage
//!
//! ```rust
//! use web5claims::{
//!     issuer::{CertificateIssuer, ProofRequest, ProofOptions},
//!     verifier::ZkProofVerifier,
//!     zk_proof::{ClaimType, CefrLevel},
//! };
//! use konnektoren_core::certificates::CertificateData;
//! use chrono::Utc;
//!
//! // Create a certificate
//! let certificate = CertificateData::new(
//!     "German_B2_Complete".to_string(),
//!     50,
//!     47,
//!     "Language Learner".to_string(),
//!     Utc::now(),
//! );
//!
//! // Create an issuer
//! let issuer = CertificateIssuer::new(
//!     "web5_claims_issuer".to_string(),
//!     "Web5 Claims Official".to_string(),
//! );
//!
//! // Generate a proof
//! let request = ProofRequest {
//!     certificate,
//!     claim_type: ClaimType::LanguageProficiency {
//!         language: "German".to_string(),
//!         min_level: CefrLevel::B2,
//!     },
//!     target_platform: "aleo".to_string(),
//!     options: ProofOptions::default(),
//! };
//!
//! let proof = issuer.generate_proof(request).unwrap();
//!
//! // Verify the proof
//! let verifier = ZkProofVerifier::new("web5_verifier".to_string());
//! let verification_result = verifier.verify_proof(&proof).unwrap();
//!
//! assert!(verification_result.is_valid);
//! assert!(verification_result.requirements_met);
//! ```

pub mod issuer;
pub mod verifier;
pub mod zk_proof;

// Re-export key types for convenience
pub use issuer::{CertificateIssuer, IssuerError, ProofOptions, ProofRequest};
pub use verifier::{VerificationResult, VerifierError, ZkProofVerifier};
pub use zk_proof::{CefrLevel, ClaimType, ProofData, ProofMetadata, PublicInputs, ZkProofClaim};

use chrono::Utc;
use konnektoren_core::certificates::CertificateData;

/// Create a sample certificate for testing and demonstration
pub fn create_sample_certificate() -> CertificateData {
    CertificateData::new(
        "German_B2_Complete".to_string(),
        50,
        47,
        "Test Student".to_string(),
        Utc::now(),
    )
}

/// Create a sample ZK proof for demonstration
pub fn create_sample_proof() -> Result<ZkProofClaim, IssuerError> {
    let issuer = CertificateIssuer::new("demo_issuer".to_string(), "Web5 Claims Demo".to_string());

    let certificate = create_sample_certificate();

    let request = ProofRequest {
        certificate,
        claim_type: ClaimType::LanguageProficiency {
            language: "German".to_string(),
            min_level: CefrLevel::B2,
        },
        target_platform: "aleo".to_string(),
        options: ProofOptions::default(),
    };

    issuer.generate_proof(request)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sample_certificate() {
        let cert = create_sample_certificate();
        assert_eq!(cert.game_path_name, "German_B2_Complete");
        assert_eq!(cert.total_challenges, 50);
        assert_eq!(cert.solved_challenges, 47);
        assert_eq!(cert.performance_percentage, 94);
    }

    #[test]
    fn test_create_sample_proof() {
        let proof = create_sample_proof().unwrap();
        assert!(!proof.proof_id.is_empty());
        assert!(proof.verify_integrity());

        // Verify the proof
        let verifier = ZkProofVerifier::new("test_verifier".to_string());
        let result = verifier.verify_proof(&proof).unwrap();
        assert!(result.is_valid);
        assert!(result.requirements_met);
    }

    #[test]
    fn test_end_to_end_workflow() {
        // 1. Create certificate
        let certificate = create_sample_certificate();

        // 2. Create issuer
        let issuer = CertificateIssuer::new(
            "e2e_issuer".to_string(),
            "End-to-End Test Issuer".to_string(),
        );

        // 3. Generate proof for language proficiency
        let language_proof_request = ProofRequest {
            certificate: certificate.clone(),
            claim_type: ClaimType::LanguageProficiency {
                language: "German".to_string(),
                min_level: CefrLevel::B1,
            },
            target_platform: "aleo".to_string(),
            options: ProofOptions::default(),
        };

        let language_proof = issuer.generate_proof(language_proof_request).unwrap();

        // 4. Generate proof for performance
        let performance_proof_request = ProofRequest {
            certificate: certificate.clone(),
            claim_type: ClaimType::PerformanceThreshold { min_percentage: 90 },
            target_platform: "stylus".to_string(),
            options: ProofOptions::default(),
        };

        let performance_proof = issuer.generate_proof(performance_proof_request).unwrap();

        // 5. Create verifier
        let verifier = ZkProofVerifier::new("e2e_verifier".to_string());

        // 6. Verify both proofs
        let language_result = verifier.verify_proof(&language_proof).unwrap();
        let performance_result = verifier.verify_proof(&performance_proof).unwrap();

        // 7. Assert all verifications passed
        assert!(language_result.is_valid && language_result.requirements_met);
        assert!(performance_result.is_valid && performance_result.requirements_met);

        // 8. Test combined proof
        let combined_request = ProofRequest {
            certificate,
            claim_type: ClaimType::Combined {
                criteria: vec![
                    ClaimType::LanguageProficiency {
                        language: "German".to_string(),
                        min_level: CefrLevel::B1,
                    },
                    ClaimType::PerformanceThreshold { min_percentage: 90 },
                ],
            },
            target_platform: "aleo".to_string(),
            options: ProofOptions::default(),
        };

        let combined_proof = issuer.generate_proof(combined_request).unwrap();
        let combined_result = verifier.verify_proof(&combined_proof).unwrap();

        assert!(combined_result.is_valid && combined_result.requirements_met);
    }
}
