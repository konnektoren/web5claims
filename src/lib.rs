use chrono::Utc;
use konnektoren_core::certificates::CertificateData;

pub fn create_sample_certificate() -> CertificateData {
    CertificateData::new(
        "German_B2_Complete".to_string(),
        50,
        47,
        "Test Student".to_string(),
        Utc::now(),
    )
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
}
