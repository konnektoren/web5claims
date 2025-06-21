use konnektoren_core::certificates::CertificateData;
use web5claims::{VerificationResult, ZkProofClaim};

#[derive(Clone, Default, PartialEq)]
pub struct AppState {
    pub certificate_data: Option<CertificateData>,
    pub zk_proof: Option<ZkProofClaim>,
    pub verification_result: Option<VerificationResult>,
    pub is_generating_proof: bool,
    pub is_verifying_proof: bool,
    pub error_message: Option<String>,
}

impl AppState {
    pub fn set_certificate(&mut self, certificate: CertificateData) {
        self.certificate_data = Some(certificate);
        // Clear previous proof data when new certificate is set
        self.zk_proof = None;
        self.verification_result = None;
        self.error_message = None;
    }

    pub fn set_zk_proof(&mut self, proof: ZkProofClaim) {
        self.zk_proof = Some(proof);
        // Clear previous verification when new proof is generated
        self.verification_result = None;
        self.error_message = None;
    }

    pub fn set_verification_result(&mut self, result: VerificationResult) {
        self.verification_result = Some(result);
        self.error_message = None;
    }

    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
        self.is_generating_proof = false;
        self.is_verifying_proof = false;
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }
}
