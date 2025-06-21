use crate::components::certificate::{ErrorDisplay, ProofButtons, ProofDisplay};
use crate::components::certificate_image::Web5CertificateImage;
use crate::components::wallet::WalletInfo;
use crate::services::ZkService;
use crate::types::AppState;
use crate::utils::clipboard::copy_to_clipboard_simple;
use crate::utils::proof_link::generate_verify_link;
use web5claims::{CefrLevel, ClaimType};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CertificateDisplayProps {
    pub state: UseStateHandle<AppState>,
    #[prop_or_default]
    pub wallet_info: Option<WalletInfo>,
}

#[function_component(CertificateDisplay)]
pub fn certificate_display(props: &CertificateDisplayProps) -> Html {
    let zk_service = use_state(|| ZkService::new());
    let copy_status = use_state(|| None::<String>);

    // Helper function to extract and validate language from course name
    fn extract_and_validate_language(course_name: &str) -> Result<String, String> {
        let language = course_name
            .split('_')
            .next()
            .ok_or("Could not extract language from course name")?;

        log::info!(
            "Extracted language: '{}' from course: '{}'",
            language,
            course_name
        );

        // Validate that it's a supported language and normalize it
        let normalized_language = match language.to_lowercase().as_str() {
            "german" => "German",
            "spanish" => "Spanish",
            "french" => "French",
            "italian" => "Italian",
            "english" => "English",
            "portuguese" => "Portuguese",
            "dutch" => "Dutch",
            _ => {
                log::warn!("Unsupported language: '{}', defaulting to German", language);
                "German" // Fallback to a supported language
            }
        };

        Ok(normalized_language.to_string())
    }

    // Helper function to extract and validate CEFR level
    fn extract_and_validate_level(course_name: &str) -> Result<CefrLevel, String> {
        log::info!("Extracting CEFR level from course: '{}'", course_name);

        // Try the built-in method first
        if let Some(level) = CefrLevel::from_course_name(course_name) {
            log::info!("Successfully extracted level: {:?}", level);
            return Ok(level);
        }

        // Manual extraction as fallback
        let level = if course_name.to_uppercase().contains("A1") {
            CefrLevel::A1
        } else if course_name.to_uppercase().contains("A2") {
            CefrLevel::A2
        } else if course_name.to_uppercase().contains("B1") {
            CefrLevel::B1
        } else if course_name.to_uppercase().contains("B2") {
            CefrLevel::B2
        } else if course_name.to_uppercase().contains("C1") {
            CefrLevel::C1
        } else if course_name.to_uppercase().contains("C2") {
            CefrLevel::C2
        } else {
            log::warn!(
                "Could not determine CEFR level from '{}', defaulting to B1",
                course_name
            );
            CefrLevel::B1 // Safe default
        };

        log::info!("Manually extracted level: {:?}", level);
        Ok(level)
    }

    // Helper function to validate certificate for ZK proof generation
    fn validate_certificate_for_proof(
        cert: &konnektoren_core::certificates::CertificateData,
    ) -> Result<(), String> {
        if cert.profile_name.trim().is_empty() {
            return Err("Certificate must have a valid student name".to_string());
        }

        if cert.game_path_name.trim().is_empty() {
            return Err("Certificate must have a valid course name".to_string());
        }

        if cert.total_challenges == 0 {
            return Err("Certificate must have at least one challenge".to_string());
        }

        if cert.performance_percentage > 100 {
            return Err("Performance percentage cannot exceed 100%".to_string());
        }

        log::info!("Certificate validation passed for: {}", cert.profile_name);
        Ok(())
    }

    // Helper function to validate claim compatibility with certificate
    fn validate_claim_compatibility(
        cert: &konnektoren_core::certificates::CertificateData,
        language: &str,
        level: CefrLevel,
    ) -> Result<(), String> {
        // Check if the certificate's course name contains the language
        let cert_language = cert
            .game_path_name
            .split('_')
            .next()
            .unwrap_or("")
            .to_lowercase();

        if cert_language != language.to_lowercase() {
            return Err(format!(
                "Certificate language '{}' does not match claim language '{}'",
                cert_language, language
            ));
        }

        // Check if the certificate's level matches or exceeds the minimum level
        if let Some(cert_level) = CefrLevel::from_course_name(&cert.game_path_name) {
            if cert_level < level {
                return Err(format!(
                    "Certificate level {:?} does not meet minimum level {:?}",
                    cert_level, level
                ));
            }
        } else {
            log::warn!("Could not determine certificate level, proceeding anyway");
        }

        Ok(())
    }

    let generate_language_proof = {
        let state = props.state.clone();
        let zk_service = zk_service.clone();
        let wallet_address = props.wallet_info.as_ref().map(|w| w.address.clone());

        Callback::from(move |_| {
            if let Some(cert) = &state.certificate_data {
                log::info!("=== Starting Language Proficiency Proof Generation ===");
                log::info!("Certificate data: {}", cert.profile_name);
                log::info!("Course: {}", cert.game_path_name);
                log::info!("Performance: {}%", cert.performance_percentage);

                // Validate certificate first
                if let Err(validation_error) = validate_certificate_for_proof(cert) {
                    log::error!("Certificate validation failed: {}", validation_error);
                    let mut new_state = (*state).clone();
                    new_state.set_error(format!(
                        "Certificate validation failed: {}",
                        validation_error
                    ));
                    state.set(new_state);
                    return;
                }

                let mut new_state = (*state).clone();
                new_state.is_generating_proof = true;
                new_state.clear_error();
                state.set(new_state);

                // Extract and validate language
                let language = match extract_and_validate_language(&cert.game_path_name) {
                    Ok(lang) => lang,
                    Err(e) => {
                        log::error!("Language extraction failed: {}", e);
                        let mut new_state = (*state).clone();
                        new_state.is_generating_proof = false;
                        new_state.set_error(format!("Language extraction failed: {}", e));
                        state.set(new_state);
                        return;
                    }
                };

                // Extract and validate CEFR level
                let min_level = match extract_and_validate_level(&cert.game_path_name) {
                    Ok(level) => level,
                    Err(e) => {
                        log::error!("CEFR level extraction failed: {}", e);
                        let mut new_state = (*state).clone();
                        new_state.is_generating_proof = false;
                        new_state.set_error(format!("CEFR level extraction failed: {}", e));
                        state.set(new_state);
                        return;
                    }
                };

                // Validate claim compatibility
                if let Err(compatibility_error) =
                    validate_claim_compatibility(cert, &language, min_level.clone())
                {
                    log::error!(
                        "Claim compatibility validation failed: {}",
                        compatibility_error
                    );
                    let mut new_state = (*state).clone();
                    new_state.is_generating_proof = false;
                    new_state.set_error(format!(
                        "Claim compatibility failed: {}",
                        compatibility_error
                    ));
                    state.set(new_state);
                    return;
                }

                if let Some(address) = &wallet_address {
                    log::info!("Generating proof with wallet: {}", address);
                } else {
                    log::info!("Generating proof locally (no wallet)");
                }

                log::info!(
                    "Final parameters - Language: '{}', Level: {:?}",
                    language,
                    min_level
                );

                let on_success = {
                    let state = state.clone();
                    Callback::from(move |proof| {
                        log::info!("Language proof generated successfully");
                        let mut new_state = (*state).clone();
                        new_state.is_generating_proof = false;
                        new_state.set_zk_proof(proof);
                        state.set(new_state);
                    })
                };

                let on_error = {
                    let state = state.clone();
                    Callback::from(move |error: String| {
                        log::error!("Language proof generation failed: {}", error);
                        let mut new_state = (*state).clone();
                        new_state.is_generating_proof = false;
                        new_state.set_error(format!("Language proof generation failed: {}", error));
                        state.set(new_state);
                    })
                };

                zk_service.generate_language_proficiency_proof(
                    cert.clone(),
                    language,
                    min_level,
                    "web5claims_local".to_string(),
                    on_success,
                    on_error,
                );
            } else {
                log::error!("No certificate data available for proof generation");
                let mut new_state = (*state).clone();
                new_state.set_error(
                    "No certificate data available. Please generate a certificate first."
                        .to_string(),
                );
                state.set(new_state);
            }
        })
    };

    let generate_performance_proof = {
        let state = props.state.clone();
        let zk_service = zk_service.clone();
        let wallet_address = props.wallet_info.as_ref().map(|w| w.address.clone());

        Callback::from(move |_| {
            if let Some(cert) = &state.certificate_data {
                log::info!("=== Starting Performance Proof Generation ===");

                // Validate certificate first
                if let Err(validation_error) = validate_certificate_for_proof(cert) {
                    log::error!("Certificate validation failed: {}", validation_error);
                    let mut new_state = (*state).clone();
                    new_state.set_error(format!(
                        "Certificate validation failed: {}",
                        validation_error
                    ));
                    state.set(new_state);
                    return;
                }

                let mut new_state = (*state).clone();
                new_state.is_generating_proof = true;
                new_state.clear_error();
                state.set(new_state);

                if let Some(address) = &wallet_address {
                    log::info!("Generating performance proof with wallet: {}", address);
                } else {
                    log::info!("Generating performance proof locally (no wallet)");
                }

                let threshold = 90u8;
                log::info!(
                    "Performance threshold: {}%, Actual performance: {}%",
                    threshold,
                    cert.performance_percentage
                );

                // Validate performance meets threshold
                if cert.performance_percentage < threshold {
                    log::error!(
                        "Certificate performance {}% does not meet threshold {}%",
                        cert.performance_percentage,
                        threshold
                    );
                    let mut new_state = (*state).clone();
                    new_state.is_generating_proof = false;
                    new_state.set_error(format!(
                        "Certificate performance {}% does not meet the required threshold of {}%",
                        cert.performance_percentage, threshold
                    ));
                    state.set(new_state);
                    return;
                }

                let on_success = {
                    let state = state.clone();
                    Callback::from(move |proof| {
                        log::info!("Performance proof generated successfully");
                        let mut new_state = (*state).clone();
                        new_state.is_generating_proof = false;
                        new_state.set_zk_proof(proof);
                        state.set(new_state);
                    })
                };

                let on_error = {
                    let state = state.clone();
                    Callback::from(move |error: String| {
                        log::error!("Performance proof generation failed: {}", error);
                        let mut new_state = (*state).clone();
                        new_state.is_generating_proof = false;
                        new_state
                            .set_error(format!("Performance proof generation failed: {}", error));
                        state.set(new_state);
                    })
                };

                zk_service.generate_performance_proof(
                    cert.clone(),
                    threshold,
                    "web5claims_local".to_string(),
                    on_success,
                    on_error,
                );
            } else {
                log::error!("No certificate data available for proof generation");
                let mut new_state = (*state).clone();
                new_state.set_error(
                    "No certificate data available. Please generate a certificate first."
                        .to_string(),
                );
                state.set(new_state);
            }
        })
    };

    let generate_combined_proof = {
        let state = props.state.clone();
        let zk_service = zk_service.clone();
        let wallet_address = props.wallet_info.as_ref().map(|w| w.address.clone());

        Callback::from(move |_| {
            if let Some(cert) = &state.certificate_data {
                log::info!("=== Starting Combined Proof Generation ===");

                // Validate certificate first
                if let Err(validation_error) = validate_certificate_for_proof(cert) {
                    log::error!("Certificate validation failed: {}", validation_error);
                    let mut new_state = (*state).clone();
                    new_state.set_error(format!(
                        "Certificate validation failed: {}",
                        validation_error
                    ));
                    state.set(new_state);
                    return;
                }

                let mut new_state = (*state).clone();
                new_state.is_generating_proof = true;
                new_state.clear_error();
                state.set(new_state);

                // Extract and validate language and level
                let language = match extract_and_validate_language(&cert.game_path_name) {
                    Ok(lang) => lang,
                    Err(e) => {
                        log::error!("Language extraction failed: {}", e);
                        let mut new_state = (*state).clone();
                        new_state.is_generating_proof = false;
                        new_state.set_error(format!("Language extraction failed: {}", e));
                        state.set(new_state);
                        return;
                    }
                };

                let min_level = match extract_and_validate_level(&cert.game_path_name) {
                    Ok(level) => level,
                    Err(e) => {
                        log::error!("CEFR level extraction failed: {}", e);
                        let mut new_state = (*state).clone();
                        new_state.is_generating_proof = false;
                        new_state.set_error(format!("CEFR level extraction failed: {}", e));
                        state.set(new_state);
                        return;
                    }
                };

                // Validate claim compatibility (no need to clone since CefrLevel is Copy)
                if let Err(compatibility_error) =
                    validate_claim_compatibility(cert, &language, min_level.clone())
                {
                    log::error!(
                        "Claim compatibility validation failed: {}",
                        compatibility_error
                    );
                    let mut new_state = (*state).clone();
                    new_state.is_generating_proof = false;
                    new_state.set_error(format!(
                        "Claim compatibility failed: {}",
                        compatibility_error
                    ));
                    state.set(new_state);
                    return;
                }

                // Validate performance for combined proof
                let performance_threshold = 90u8;
                if cert.performance_percentage < performance_threshold {
                    log::error!(
                        "Certificate performance {}% does not meet threshold {}%",
                        cert.performance_percentage,
                        performance_threshold
                    );
                    let mut new_state = (*state).clone();
                    new_state.is_generating_proof = false;
                    new_state.set_error(format!(
                        "Certificate performance {}% does not meet the required threshold of {}%",
                        cert.performance_percentage, performance_threshold
                    ));
                    state.set(new_state);
                    return;
                }

                if let Some(address) = &wallet_address {
                    log::info!("Generating combined proof with wallet: {}", address);
                } else {
                    log::info!("Generating combined proof locally (no wallet)");
                }

                // Create criteria vector (move language and min_level here)
                let criteria = vec![
                    ClaimType::LanguageProficiency {
                        language,
                        min_level,
                    },
                    ClaimType::PerformanceThreshold {
                        min_percentage: performance_threshold,
                    },
                ];

                log::info!("Combined criteria: {} claims", criteria.len());

                let on_success = {
                    let state = state.clone();
                    Callback::from(move |proof| {
                        log::info!("Combined proof generated successfully");
                        let mut new_state = (*state).clone();
                        new_state.is_generating_proof = false;
                        new_state.set_zk_proof(proof);
                        state.set(new_state);
                    })
                };

                let on_error = {
                    let state = state.clone();
                    Callback::from(move |error: String| {
                        log::error!("Combined proof generation failed: {}", error);
                        let mut new_state = (*state).clone();
                        new_state.is_generating_proof = false;
                        new_state.set_error(format!("Combined proof generation failed: {}", error));
                        state.set(new_state);
                    })
                };

                zk_service.generate_combined_proof(
                    cert.clone(),
                    criteria,
                    "web5claims_local".to_string(),
                    on_success,
                    on_error,
                );
            } else {
                log::error!("No certificate data available for proof generation");
                let mut new_state = (*state).clone();
                new_state.set_error(
                    "No certificate data available. Please generate a certificate first."
                        .to_string(),
                );
                state.set(new_state);
            }
        })
    };

    let verify_proof = {
        let state = props.state.clone();
        let zk_service = zk_service.clone();

        Callback::from(move |_| {
            if let Some(proof) = &state.zk_proof {
                let mut new_state = (*state).clone();
                new_state.is_verifying_proof = true;
                new_state.clear_error();
                state.set(new_state);

                let on_success = {
                    let state = state.clone();
                    Callback::from(move |result| {
                        let mut new_state = (*state).clone();
                        new_state.is_verifying_proof = false;
                        new_state.set_verification_result(result);
                        state.set(new_state);
                    })
                };

                let on_error = {
                    let state = state.clone();
                    Callback::from(move |error: String| {
                        let mut new_state = (*state).clone();
                        new_state.is_verifying_proof = false;
                        new_state.set_error(error);
                        state.set(new_state);
                    })
                };

                zk_service.verify_proof(proof.clone(), on_success, on_error);
            }
        })
    };

    let copy_proof_data = {
        let state = props.state.clone();
        let copy_status = copy_status.clone();

        Callback::from(move |_| {
            if let Some(proof) = &state.zk_proof {
                match generate_verify_link(proof) {
                    Ok(link) => {
                        copy_to_clipboard_simple(&link, copy_status.clone());
                    }
                    Err(_) => {
                        if let Ok(json) = serde_json::to_string_pretty(proof) {
                            copy_to_clipboard_simple(&json, copy_status.clone());
                        } else {
                            copy_status.set(Some("❌ Failed to serialize proof".to_string()));
                        }
                    }
                }
            }
        })
    };

    let on_dismiss_error = {
        let state = props.state.clone();
        Callback::from(move |_| {
            let mut new_state = (*state).clone();
            new_state.clear_error();
            state.set(new_state);
        })
    };

    html! {
        <div class="space-y-6">
            <ErrorDisplay
                error={props.state.error_message.clone()}
                on_dismiss={on_dismiss_error}
                dismissible={true}
            />

            // Wallet connection status (informational only)
            if let Some(wallet) = &props.wallet_info {
                <div class="alert alert-success">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    <div>
                        <div class="font-semibold">{"Leo Wallet Connected"}</div>
                        <div class="text-sm font-mono">{format!("{}...{}", &wallet.address[..8], &wallet.address[wallet.address.len()-8..])}</div>
                        <div class="text-xs">{"Future on-chain functionality available"}</div>
                    </div>
                </div>
            }

            if let Some(cert) = &props.state.certificate_data {
                <CertificatePreview certificate={cert.clone()} />
            }

            <ProofButtons
                on_generate_language_proof={generate_language_proof}
                on_generate_performance_proof={generate_performance_proof}
                on_generate_combined_proof={generate_combined_proof}
                is_generating={props.state.is_generating_proof}
                has_certificate={props.state.certificate_data.is_some()}
            />

            <ProofDisplay
                proof={props.state.zk_proof.clone()}
                verification_result={props.state.verification_result.clone()}
                on_verify={verify_proof}
                on_copy_proof={copy_proof_data}
                is_verifying={props.state.is_verifying_proof}
                copy_status={(*copy_status).clone()}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CertificatePreviewProps {
    pub certificate: konnektoren_core::certificates::CertificateData,
}

#[function_component(CertificatePreview)]
pub fn certificate_preview(props: &CertificatePreviewProps) -> Html {
    let copy_status = use_state(|| None::<String>);

    let copy_certificate_data = {
        let certificate = props.certificate.clone();
        let copy_status = copy_status.clone();

        Callback::from(move |_| {
            let cert_data = certificate.to_base64();
            copy_to_clipboard_simple(&cert_data, copy_status.clone());
        })
    };

    html! {
        <div class="space-y-4">
            if let Some(status) = &*copy_status {
                <div class="toast toast-top toast-center">
                    <div class={classes!(
                        "alert",
                        if status.contains("✅") { "alert-success" } else { "alert-error" }
                    )}>
                        <span>{status}</span>
                    </div>
                </div>
            }

            <div class="card bg-gradient-to-br from-blue-50 to-indigo-100 border border-blue-200">
                <div class="card-body p-6">
                    <h3 class="card-title text-xl mb-4 text-center">
                        {"🎓 Certificate of Achievement"}
                    </h3>

                    <div class="flex justify-center mb-4">
                        <Web5CertificateImage certificate_data={props.certificate.clone()} />
                    </div>

                    <div class="flex flex-wrap gap-2 justify-center">
                        <button
                            class="btn btn-outline btn-sm"
                            onclick={copy_certificate_data}
                            title="Copy certificate data"
                        >
                            {"📋 Copy Data"}
                        </button>
                        <a
                            class="btn btn-outline btn-sm"
                            href={format!("data:text/plain;charset=utf-8,{}", props.certificate.to_base64())}
                            download="certificate.txt"
                        >
                            {"💾 Download"}
                        </a>
                    </div>
                </div>
            </div>

            <div class="stats stats-vertical lg:stats-horizontal shadow w-full">
                <div class="stat">
                    <div class="stat-figure text-secondary">{"🎯"}</div>
                    <div class="stat-title">{"Performance"}</div>
                    <div class="stat-value text-primary">{props.certificate.performance_percentage}{"%"}</div>
                    <div class="stat-desc">{format!("{}/{} challenges completed", props.certificate.solved_challenges, props.certificate.total_challenges)}</div>
                </div>

                <div class="stat">
                    <div class="stat-figure text-secondary">{"🌍"}</div>
                    <div class="stat-title">{"Language"}</div>
                    <div class="stat-value text-secondary">{props.certificate.game_path_name.split('_').next().unwrap_or("Unknown")}</div>
                    <div class="stat-desc">{props.certificate.game_path_name.split('_').nth(1).unwrap_or("Unknown Level")}</div>
                </div>

                <div class="stat">
                    <div class="stat-figure text-secondary">{"👤"}</div>
                    <div class="stat-title">{"Student"}</div>
                    <div class="stat-value text-accent text-lg">{&props.certificate.profile_name}</div>
                    <div class="stat-desc">{props.certificate.date.format("%Y-%m-%d").to_string()}</div>
                </div>
            </div>
        </div>
    }
}
