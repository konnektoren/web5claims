pub mod clipboard;

use konnektoren_core::certificates::CertificateData;

pub fn get_cefr_level_from_course(course_name: &str) -> u8 {
    if course_name.contains("A1") {
        1
    } else if course_name.contains("A2") {
        2
    } else if course_name.contains("B1") {
        3
    } else if course_name.contains("B2") {
        4
    } else if course_name.contains("C1") {
        5
    } else if course_name.contains("C2") {
        6
    } else {
        0
    }
}

pub fn format_course_name(course_name: &str) -> String {
    course_name
        .replace("_", " ")
        .replace("Complete", "")
        .replace("Basic", "- Basic")
        .replace("Elementary", "- Elementary")
        .replace("Intermediate", "- Intermediate")
        .replace("Advanced", "- Advanced")
        .trim()
        .to_string()
}

pub fn get_achievement_level(performance: u8) -> &'static str {
    match performance {
        95..=100 => "Excellent",
        90..=94 => "Very Good",
        80..=89 => "Good",
        70..=79 => "Satisfactory",
        60..=69 => "Adequate",
        _ => "Needs Improvement",
    }
}

pub fn is_certificate_valid_for_zk_proof(cert: &CertificateData) -> bool {
    // Check if certificate meets minimum requirements for ZK proof
    cert.performance_percentage >= 60
        && cert.solved_challenges > 0
        && !cert.profile_name.is_empty()
        && !cert.game_path_name.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_cefr_level_parsing() {
        assert_eq!(get_cefr_level_from_course("German_B2_Complete"), 4);
        assert_eq!(get_cefr_level_from_course("Spanish_A1_Basic"), 1);
        assert_eq!(get_cefr_level_from_course("French_C1_Advanced"), 5);
    }

    #[test]
    fn test_course_name_formatting() {
        assert_eq!(format_course_name("German_B2_Complete"), "German B2");
        assert_eq!(format_course_name("Spanish_A1_Basic"), "Spanish A1 - Basic");
    }

    #[test]
    fn test_certificate_validation() {
        let valid_cert = CertificateData::new(
            "German_B2_Complete".to_string(),
            50,
            40,
            "Test User".to_string(),
            Utc::now(),
        );

        assert!(is_certificate_valid_for_zk_proof(&valid_cert));

        let invalid_cert = CertificateData::new(
            "".to_string(),
            50,
            20, // Too low performance
            "".to_string(),
            Utc::now(),
        );

        assert!(!is_certificate_valid_for_zk_proof(&invalid_cert));
    }
}
