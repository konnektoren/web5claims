use gloo::utils::document;
use konnektoren_core::certificates::CertificateData;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDocument, HtmlTextAreaElement};

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

/// Copy text to clipboard with fallback support
pub async fn copy_to_clipboard(text: &str) -> Result<(), String> {
    if let Some(window) = web_sys::window() {
        // Try modern clipboard API first
        let clipboard = window.navigator().clipboard();
        let promise = clipboard.write_text(text);
        match wasm_bindgen_futures::JsFuture::from(promise).await {
            Ok(_) => return Ok(()),
            Err(_) => {
                // Fall through to legacy method
            }
        }
    }

    // Legacy fallback method
    copy_to_clipboard_legacy(text)
}

/// Legacy clipboard copy using execCommand
pub fn copy_to_clipboard_legacy(text: &str) -> Result<(), String> {
    let document = document();

    // Create temporary textarea
    let textarea = document
        .create_element("textarea")
        .map_err(|_| "Failed to create textarea element")?;

    let textarea: HtmlTextAreaElement = textarea
        .dyn_into()
        .map_err(|_| "Failed to cast to textarea")?;

    // Set up textarea - style() returns CssStyleDeclaration directly, not a Result
    textarea.set_value(text);
    let style = textarea.style();
    let _ = style.set_property("position", "fixed");
    let _ = style.set_property("left", "-9999px");
    let _ = style.set_property("top", "-9999px");
    let _ = style.set_property("opacity", "0");

    // Add to DOM
    if let Some(body) = document.body() {
        body.append_child(&textarea)
            .map_err(|_| "Failed to append textarea to body")?;

        // Select and copy
        textarea.select();

        // Set selection range
        if textarea.set_selection_range(0, text.len() as u32).is_ok() {
            // Try to get HtmlDocument for execCommand
            if let Ok(html_doc) = document.dyn_into::<HtmlDocument>() {
                let success = html_doc.exec_command("copy").unwrap_or(false);

                // Clean up
                let _ = body.remove_child(&textarea);

                if success {
                    Ok(())
                } else {
                    Err("Copy command returned false".to_string())
                }
            } else {
                let _ = body.remove_child(&textarea);
                Err("Could not access document.execCommand".to_string())
            }
        } else {
            let _ = body.remove_child(&textarea);
            Err("Could not select text".to_string())
        }
    } else {
        Err("No document body found".to_string())
    }
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
