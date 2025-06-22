use serde::{Deserialize, Serialize};
use yew_router::prelude::*;

#[derive(Clone, Debug, Routable, PartialEq, Serialize, Deserialize)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/issuer")]
    Issuer,
    #[at("/lookup")]
    CertificateLookup,
    #[at("/verifier")]
    Verifier,
    #[at("/verify")]
    VerifyProof,
    #[at("/zkpassport")]
    ZkPassport,
    #[at("/zkpass")]
    ZkPass,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Route::Home => write!(f, "/"),
            Route::Issuer => write!(f, "/issuer"),
            Route::CertificateLookup => write!(f, "/lookup"),
            Route::Verifier => write!(f, "/verifier"),
            Route::VerifyProof => write!(f, "/verify"),
            Route::ZkPassport => write!(f, "/zkpassport"),
            Route::ZkPass => write!(f, "/zkpass"),
            Route::NotFound => write!(f, "/404"),
        }
    }
}

impl Route {
    pub fn title(&self) -> &'static str {
        match self {
            Route::Home => "Web5 Claims - ZK Language Certificates",
            Route::Issuer => "Certificate Issuer - Web5 Claims",
            Route::CertificateLookup => "Certificate Lookup - Web5 Claims",
            Route::Verifier => "Proof Verifier - Web5 Claims",
            Route::VerifyProof => "Verify Proof - Web5 Claims",
            Route::ZkPassport => "ZK Passport - Web5 Claims",
            Route::ZkPass => "ZK Identity Verification - Web5 Claims",
            Route::NotFound => "Page Not Found - Web5 Claims",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Route::Home => "Create and verify zero-knowledge language learning certificates",
            Route::Issuer => {
                "Generate certificates and ZK proofs for language learning achievements"
            }
            Route::CertificateLookup => {
                "Look up existing certificates by ID and generate ZK proofs"
            }
            Route::Verifier => "Verify zero-knowledge proofs without revealing private data",
            Route::VerifyProof => "Verifying a zero-knowledge proof link",
            Route::ZkPassport => "Identity verification using ZK Passport technology",
            Route::ZkPass => "Advanced identity verification using ZKPass passport scanning",
            Route::NotFound => "The requested page could not be found",
        }
    }

    // Add helper methods for ZKPass integration
    pub fn zkpass_external_url() -> String {
        // In production, this will be the GitHub Pages URL
        if cfg!(debug_assertions) {
            "http://localhost:8000/".to_string()
        } else {
            "/zkpass/".to_string()
        }
    }
}
