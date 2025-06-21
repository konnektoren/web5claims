use konnektoren_core::certificates::CertificateData;
use wasm_bindgen_futures::spawn_local;
use web5claims::{
    CefrLevel, CertificateIssuer, ClaimType, IssuerError, ProofOptions, ProofRequest,
    VerificationResult, VerifierError, ZkProofClaim, ZkProofVerifier,
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
        let issuer = self.issuer.clone();

        spawn_local(async move {
            let request = ProofRequest {
                certificate,
                claim_type: ClaimType::LanguageProficiency {
                    language,
                    min_level,
                },
                target_platform: platform,
                options: ProofOptions::default(),
            };

            match issuer.generate_proof(request) {
                Ok(proof) => on_success.emit(proof),
                Err(e) => on_error.emit(format!("Proof generation failed: {}", e)),
            }
        });
    }

    pub fn generate_performance_proof(
        &self,
        certificate: CertificateData,
        min_percentage: u8,
        platform: String,
        on_success: Callback<ZkProofClaim>,
        on_error: Callback<String>,
    ) {
        let issuer = self.issuer.clone();

        spawn_local(async move {
            let request = ProofRequest {
                certificate,
                claim_type: ClaimType::PerformanceThreshold { min_percentage },
                target_platform: platform,
                options: ProofOptions::default(),
            };

            match issuer.generate_proof(request) {
                Ok(proof) => on_success.emit(proof),
                Err(e) => on_error.emit(format!("Proof generation failed: {}", e)),
            }
        });
    }

    pub fn generate_combined_proof(
        &self,
        certificate: CertificateData,
        criteria: Vec<ClaimType>,
        platform: String,
        on_success: Callback<ZkProofClaim>,
        on_error: Callback<String>,
    ) {
        let issuer = self.issuer.clone();

        spawn_local(async move {
            let request = ProofRequest {
                certificate,
                claim_type: ClaimType::Combined { criteria },
                target_platform: platform,
                options: ProofOptions::default(),
            };

            match issuer.generate_proof(request) {
                Ok(proof) => on_success.emit(proof),
                Err(e) => on_error.emit(format!("Proof generation failed: {}", e)),
            }
        });
    }

    pub fn verify_proof(
        &self,
        proof: ZkProofClaim,
        on_success: Callback<VerificationResult>,
        on_error: Callback<String>,
    ) {
        let verifier = self.verifier.clone();

        spawn_local(async move {
            match verifier.verify_proof(&proof) {
                Ok(result) => on_success.emit(result),
                Err(e) => on_error.emit(format!("Proof verification failed: {}", e)),
            }
        });
    }
}

impl Default for ZkService {
    fn default() -> Self {
        Self::new()
    }
}
