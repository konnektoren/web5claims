use konnektoren_core::certificates::CertificateData;
use web5claims::{
    CefrLevel, CertificateIssuer, ClaimType, ProofOptions, ProofRequest, VerificationResult,
    ZkProofClaim, ZkProofVerifier,
};
use yew::Callback;

pub struct ZkService {
    issuer: CertificateIssuer,
    verifier: ZkProofVerifier,
}

impl ZkService {
    pub fn new() -> Self {
        Self {
            issuer: CertificateIssuer::new(
                "web5_claims_ui_issuer".to_string(),
                "Web5 Claims UI Issuer".to_string(),
            ),
            verifier: ZkProofVerifier::new("web5_claims_ui_verifier".to_string()),
        }
    }

    pub fn generate_language_proficiency_proof(
        &self,
        certificate: CertificateData,
        language: String,
        min_level: CefrLevel,
        platform: String,
        on_success: Callback<ZkProofClaim>,
        on_error: Callback<String>,
    ) {
        log::info!("=== Language Proficiency Proof Generation ===");
        log::info!("Certificate Details:");
        log::info!("  - Profile: {}", certificate.profile_name);
        log::info!("  - Course: {}", certificate.game_path_name);
        log::info!("  - Performance: {}%", certificate.performance_percentage);
        log::info!(
            "  - Solved: {}/{}",
            certificate.solved_challenges,
            certificate.total_challenges
        );
        log::info!("  - Date: {}", certificate.date);
        log::info!("Claim Details:");
        log::info!("  - Language: {}", language);
        log::info!("  - Min Level: {:?}", min_level);
        log::info!("  - Platform: {}", platform);

        let request = ProofRequest {
            certificate,
            claim_type: ClaimType::LanguageProficiency {
                language,
                min_level,
            },
            target_platform: platform,
            options: ProofOptions::default(),
        };

        match self.issuer.generate_proof(request) {
            Ok(proof) => {
                log::info!(
                    "✅ Proof generated successfully with ID: {}",
                    proof.proof_id
                );
                on_success.emit(proof);
            }
            Err(e) => {
                log::error!("❌ Proof generation failed: {}", e);
                on_error.emit(format!("Proof generation failed: {}", e));
            }
        }
    }

    pub fn generate_performance_proof(
        &self,
        certificate: CertificateData,
        min_percentage: u8,
        platform: String,
        on_success: Callback<ZkProofClaim>,
        on_error: Callback<String>,
    ) {
        log::info!("=== Performance Proof Generation ===");
        log::info!("Certificate Details:");
        log::info!("  - Profile: {}", certificate.profile_name);
        log::info!("  - Course: {}", certificate.game_path_name);
        log::info!("  - Performance: {}%", certificate.performance_percentage);
        log::info!(
            "  - Solved: {}/{}",
            certificate.solved_challenges,
            certificate.total_challenges
        );
        log::info!("Claim Details:");
        log::info!("  - Min Percentage: {}%", min_percentage);
        log::info!("  - Platform: {}", platform);

        let request = ProofRequest {
            certificate,
            claim_type: ClaimType::PerformanceThreshold { min_percentage },
            target_platform: platform,
            options: ProofOptions::default(),
        };

        match self.issuer.generate_proof(request) {
            Ok(proof) => {
                log::info!(
                    "✅ Performance proof generated successfully with ID: {}",
                    proof.proof_id
                );
                on_success.emit(proof);
            }
            Err(e) => {
                log::error!("❌ Performance proof generation failed: {}", e);
                on_error.emit(format!("Performance proof generation failed: {}", e));
            }
        }
    }

    pub fn generate_combined_proof(
        &self,
        certificate: CertificateData,
        criteria: Vec<ClaimType>,
        platform: String,
        on_success: Callback<ZkProofClaim>,
        on_error: Callback<String>,
    ) {
        log::info!("Starting combined proof generation");
        log::info!("Certificate: {:?}", certificate.profile_name);
        log::info!("Criteria count: {}, Platform: {}", criteria.len(), platform);

        let request = ProofRequest {
            certificate,
            claim_type: ClaimType::Combined { criteria },
            target_platform: platform,
            options: ProofOptions::default(),
        };

        match self.issuer.generate_proof(request) {
            Ok(proof) => {
                log::info!(
                    "Combined proof generated successfully with ID: {}",
                    proof.proof_id
                );
                on_success.emit(proof);
            }
            Err(e) => {
                log::error!("Combined proof generation failed: {}", e);
                on_error.emit(format!("Combined proof generation failed: {}", e));
            }
        }
    }

    pub fn verify_proof(
        &self,
        proof: ZkProofClaim,
        on_success: Callback<VerificationResult>,
        on_error: Callback<String>,
    ) {
        match self.verifier.verify_proof(&proof) {
            Ok(result) => on_success.emit(result),
            Err(e) => on_error.emit(format!("Proof verification failed: {}", e)),
        }
    }
}

impl Default for ZkService {
    fn default() -> Self {
        Self::new()
    }
}
