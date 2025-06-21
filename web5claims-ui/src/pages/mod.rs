pub mod certificate_lookup;
pub mod home;
pub mod issuer;
pub mod not_found;
pub mod verifier;
pub mod verify_proof;
pub mod zkpassport;

pub use certificate_lookup::CertificateLookupPage;
pub use home::HomePage;
pub use issuer::IssuerPage;
pub use not_found::NotFoundPage;
pub use verifier::VerifierPage;
pub use verify_proof::VerifyProofPage;
pub use zkpassport::ZkPassportPage;
