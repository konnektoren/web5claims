pub const PROGRAM_ID: &str = "web5claims.aleo";

// Helper functions to convert data for Leo program
pub fn encode_language_to_field(language: &str) -> String {
    // Convert language string to field hash
    // In Leo, we need to represent strings as field elements
    match language.to_lowercase().as_str() {
        "german" => "1field",
        "spanish" => "2field",
        "french" => "3field",
        "italian" => "4field",
        "english" => "5field",
        "portuguese" => "6field",
        "dutch" => "7field",
        "russian" => "8field",
        "chinese" => "9field",
        "japanese" => "10field",
        _ => "0field", // Unknown language
    }
}

pub fn cefr_level_to_u8(level: &web5claims::CefrLevel) -> u8 {
    match level {
        web5claims::CefrLevel::A1 => 1,
        web5claims::CefrLevel::A2 => 2,
        web5claims::CefrLevel::B1 => 3,
        web5claims::CefrLevel::B2 => 4,
        web5claims::CefrLevel::C1 => 5,
        web5claims::CefrLevel::C2 => 6,
    }
}

pub fn get_current_timestamp() -> u32 {
    // Get current Unix timestamp as u32
    let now = chrono::Utc::now();
    now.timestamp() as u32
}

// Convert certificate data to Leo program inputs
pub fn certificate_to_leo_inputs(
    cert: &konnektoren_core::certificates::CertificateData,
    recipient_address: &str,
) -> Vec<String> {
    let language = cert.game_path_name.split('_').next().unwrap_or("unknown");
    let level = web5claims::CefrLevel::from_course_name(&cert.game_path_name)
        .map(|l| cefr_level_to_u8(&l))
        .unwrap_or(1);

    vec![
        recipient_address.to_string(),                // recipient
        encode_language_to_field(language),           // language
        format!("{}u8", level),                       // level
        format!("{}u8", cert.performance_percentage), // score
        format!("{}u16", cert.total_challenges),      // challenges_total
        format!("{}u16", cert.solved_challenges),     // challenges_solved
        format!("{}u32", get_current_timestamp()),    // issued_at
    ]
}

// Generate inputs for language proficiency proof
pub fn language_proof_inputs(language: &str, min_level: &web5claims::CefrLevel) -> Vec<String> {
    vec![
        encode_language_to_field(language),           // language_required
        format!("{}u8", cefr_level_to_u8(min_level)), // min_level
        format!("{}u32", get_current_timestamp()),    // current_time
    ]
}

// Generate inputs for performance proof
pub fn performance_proof_inputs(language: &str, min_score: u8) -> Vec<String> {
    vec![
        encode_language_to_field(language),        // language_required
        format!("{}u8", min_score),                // min_score
        format!("{}u32", get_current_timestamp()), // current_time
    ]
}

// Generate inputs for combined proof
pub fn combined_proof_inputs(
    language: &str,
    min_level: &web5claims::CefrLevel,
    min_score: u8,
) -> Vec<String> {
    vec![
        encode_language_to_field(language),           // language_required
        format!("{}u8", cefr_level_to_u8(min_level)), // min_level
        format!("{}u8", min_score),                   // min_score
        format!("{}u32", get_current_timestamp()),    // current_time
    ]
}
