use base64::{engine::general_purpose, Engine as _};
use web5claims::ZkProofClaim;

/// Generate a shareable verification link for a proof using query parameters
pub fn generate_verify_link(proof: &ZkProofClaim) -> Result<String, String> {
    // Serialize proof to JSON
    let json =
        serde_json::to_string(proof).map_err(|e| format!("Failed to serialize proof: {}", e))?;

    // Compress and encode to base64 for URL safety
    let compressed = compress_string(&json)?;
    let encoded = general_purpose::URL_SAFE_NO_PAD.encode(compressed);

    // Get current origin
    let window = web_sys::window().ok_or("No window available")?;
    let location = window.location();
    let origin = location.origin().map_err(|_| "Could not get origin")?;

    // Create verification URL with query parameter
    let verify_url = format!("{}/#/verify?proof={}", origin, encoded);

    Ok(verify_url)
}

/// Decode a proof from URL query parameter
pub fn decode_proof_from_query() -> Result<Option<ZkProofClaim>, String> {
    log::info!("Starting to decode proof from query");

    let window = web_sys::window().ok_or("No window available")?;
    let location = window.location();

    // Get the hash part of the URL (after #)
    let hash = location.hash().map_err(|_| "Could not get hash")?;
    log::info!("Hash: {}", hash);

    // Parse the hash to extract query parameters
    // Hash format: #/verify?proof=...
    if let Some(query_start) = hash.find('?') {
        let query_part = &hash[query_start + 1..];
        log::info!("Query part: {}", query_part);

        let url_params = web_sys::UrlSearchParams::new_with_str(query_part)
            .map_err(|_| "Failed to parse URL parameters")?;

        if let Some(encoded_proof) = url_params.get("proof") {
            log::info!("Found proof parameter, length: {}", encoded_proof.len());
            let proof = decode_proof_from_url(&encoded_proof)?;
            Ok(Some(proof))
        } else {
            log::warn!("No 'proof' parameter found in query");
            Ok(None)
        }
    } else {
        log::warn!("No query parameters found in hash");
        Ok(None)
    }
}

/// Decode a proof from URL-encoded data
pub fn decode_proof_from_url(encoded_data: &str) -> Result<ZkProofClaim, String> {
    log::info!(
        "Decoding proof from URL data, length: {}",
        encoded_data.len()
    );

    // Decode from base64
    let decoded_bytes = general_purpose::URL_SAFE_NO_PAD
        .decode(encoded_data)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;

    log::info!("Decoded bytes length: {}", decoded_bytes.len());

    // Decompress
    let json_str = decompress_bytes(&decoded_bytes)?;
    log::info!("Decompressed JSON length: {}", json_str.len());

    // Parse JSON
    let proof: ZkProofClaim = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse proof JSON: {}", e))?;

    log::info!("Successfully parsed proof with ID: {}", proof.proof_id);
    Ok(proof)
}

/// Simple compression using deflate
fn compress_string(input: &str) -> Result<Vec<u8>, String> {
    use flate2::write::DeflateEncoder;
    use flate2::Compression;
    use std::io::Write;

    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder
        .write_all(input.as_bytes())
        .map_err(|e| format!("Compression failed: {}", e))?;
    encoder
        .finish()
        .map_err(|e| format!("Compression finish failed: {}", e))
}

/// Simple decompression using deflate
fn decompress_bytes(input: &[u8]) -> Result<String, String> {
    use flate2::read::DeflateDecoder;
    use std::io::Read;

    let mut decoder = DeflateDecoder::new(input);
    let mut result = String::new();
    decoder
        .read_to_string(&mut result)
        .map_err(|e| format!("Decompression failed: {}", e))?;
    Ok(result)
}

/// Get the current page URL
pub fn get_current_url() -> Option<String> {
    web_sys::window()?.location().href().ok()
}

/// Copy text to clipboard
pub fn copy_link_to_clipboard(link: &str) -> Result<(), String> {
    if let Some(window) = web_sys::window() {
        let clipboard = window.navigator().clipboard();
        let _promise = clipboard.write_text(link);
        Ok(())
    } else {
        Err("No window available".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use web5claims::{CefrLevel, ClaimType, ProofData, ProofMetadata, PublicInputs};

    fn create_test_proof() -> ZkProofClaim {
        ZkProofClaim::new(
            ClaimType::LanguageProficiency {
                language: "German".to_string(),
                min_level: CefrLevel::B2,
            },
            PublicInputs {
                requirements: HashMap::new(),
                verification_result: true,
                certificate_hash: "test_hash".to_string(),
            },
            ProofData {
                proof_bytes: vec![1, 2, 3, 4],
                circuit_id: "test_circuit".to_string(),
                vk_hash: "test_vk".to_string(),
            },
            ProofMetadata {
                version: "1.0.0".to_string(),
                platform: "test".to_string(),
                properties: HashMap::new(),
            },
        )
    }

    #[test]
    fn test_compression_decompression() {
        let test_string = "Hello, World! This is a test string for compression.";
        let compressed = compress_string(test_string).unwrap();
        let decompressed = decompress_bytes(&compressed).unwrap();
        assert_eq!(test_string, decompressed);
    }

    #[test]
    fn test_proof_encoding_decoding() {
        let original_proof = create_test_proof();

        // Serialize to JSON
        let json = serde_json::to_string(&original_proof).unwrap();

        // Compress and encode
        let compressed = compress_string(&json).unwrap();
        let encoded = general_purpose::URL_SAFE_NO_PAD.encode(compressed);

        // Decode back
        let decoded_proof = decode_proof_from_url(&encoded).unwrap();

        // Should match original
        assert_eq!(original_proof.proof_id, decoded_proof.proof_id);
        assert_eq!(original_proof.claim_type, decoded_proof.claim_type);
    }
}
