use konnektoren_core::certificates::CertificateData;

#[derive(Clone, Default, PartialEq)]
pub struct AppState {
    pub certificate_data: Option<CertificateData>,
    pub zk_proof: Option<String>,
    pub verification_result: Option<bool>,
}
