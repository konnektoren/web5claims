use crate::components::{
    certificate::{ErrorDisplay, ProofButtons, ProofDisplay},
    layout::PageLayout,
    ui::{Button, Card, InputField},
};
use crate::services::ZkService;
use crate::types::AppState;
use crate::utils::clipboard::copy_to_clipboard_simple;
use crate::utils::proof_link::generate_verify_link;
use konnektoren_core::certificates::CertificateData;
use web5claims::{CefrLevel, ClaimType};
use yew::prelude::*;

#[function_component(CertificateLookupPage)]
pub fn certificate_lookup_page() -> Html {
    let certificate_id = use_state(|| String::new());
    let app_state = use_state(|| AppState::default());
    let zk_service = use_state(|| ZkService::new());
    let copy_status = use_state(|| None::<String>);
    let is_loading = use_state(|| false);

    let on_certificate_id_change = {
        let certificate_id = certificate_id.clone();
        Callback::from(move |value: String| {
            certificate_id.set(value);
        })
    };

    let load_certificate = {
        let certificate_id = certificate_id.clone();
        let app_state = app_state.clone();
        let is_loading = is_loading.clone();

        // CHANGE THIS LINE: Accept MouseEvent
        Callback::from(move |_: MouseEvent| {
            let id = (*certificate_id).clone();
            if id.trim().is_empty() {
                let mut new_state = (*app_state).clone();
                new_state.set_error("Please enter a certificate ID".to_string());
                app_state.set(new_state);
                return;
            }

            is_loading.set(true);
            let mut new_state = (*app_state).clone();
            new_state.clear_error();
            app_state.set(new_state);

            // Try to decode the certificate from base64
            match CertificateData::from_base64(&id) {
                Ok(certificate) => {
                    let mut new_state = (*app_state).clone();
                    new_state.set_certificate(certificate);
                    app_state.set(new_state);
                    is_loading.set(false);
                }
                Err(_) => {
                    // If base64 decoding fails, try as a simple lookup (simulate database lookup)
                    match simulate_certificate_lookup(&id) {
                        Some(certificate) => {
                            let mut new_state = (*app_state).clone();
                            new_state.set_certificate(certificate);
                            app_state.set(new_state);
                        }
                        None => {
                            let mut new_state = (*app_state).clone();
                            new_state.set_error(format!("Certificate not found with ID: {}", id));
                            app_state.set(new_state);
                        }
                    }
                    is_loading.set(false);
                }
            }
        })
    };

    let generate_language_proof = {
        let app_state = app_state.clone();
        let zk_service = zk_service.clone();

        Callback::from(move |_| {
            if let Some(cert) = &app_state.certificate_data {
                let mut new_state = (*app_state).clone();
                new_state.is_generating_proof = true;
                new_state.clear_error();
                app_state.set(new_state);

                let language = cert
                    .game_path_name
                    .split('_')
                    .next()
                    .unwrap_or("German")
                    .to_string();
                let min_level =
                    CefrLevel::from_course_name(&cert.game_path_name).unwrap_or(CefrLevel::B2);

                let on_success = {
                    let app_state = app_state.clone();
                    Callback::from(move |proof| {
                        let mut new_state = (*app_state).clone();
                        new_state.is_generating_proof = false;
                        new_state.set_zk_proof(proof);
                        app_state.set(new_state);
                    })
                };

                let on_error = {
                    let app_state = app_state.clone();
                    Callback::from(move |error: String| {
                        let mut new_state = (*app_state).clone();
                        new_state.set_error(error);
                        app_state.set(new_state);
                    })
                };

                zk_service.generate_language_proficiency_proof(
                    cert.clone(),
                    language,
                    min_level,
                    "aleo".to_string(),
                    on_success,
                    on_error,
                );
            }
        })
    };

    let generate_performance_proof = {
        let app_state = app_state.clone();
        let zk_service = zk_service.clone();

        Callback::from(move |_| {
            if let Some(cert) = &app_state.certificate_data {
                let mut new_state = (*app_state).clone();
                new_state.is_generating_proof = true;
                new_state.clear_error();
                app_state.set(new_state);

                let on_success = {
                    let app_state = app_state.clone();
                    Callback::from(move |proof| {
                        let mut new_state = (*app_state).clone();
                        new_state.is_generating_proof = false;
                        new_state.set_zk_proof(proof);
                        app_state.set(new_state);
                    })
                };

                let on_error = {
                    let app_state = app_state.clone();
                    Callback::from(move |error: String| {
                        let mut new_state = (*app_state).clone();
                        new_state.set_error(error);
                        app_state.set(new_state);
                    })
                };

                zk_service.generate_performance_proof(
                    cert.clone(),
                    90,
                    "aleo".to_string(),
                    on_success,
                    on_error,
                );
            }
        })
    };

    let generate_combined_proof = {
        let app_state = app_state.clone();
        let zk_service = zk_service.clone();

        Callback::from(move |_| {
            if let Some(cert) = &app_state.certificate_data {
                let mut new_state = (*app_state).clone();
                new_state.is_generating_proof = true;
                new_state.clear_error();
                app_state.set(new_state);

                let language = cert
                    .game_path_name
                    .split('_')
                    .next()
                    .unwrap_or("German")
                    .to_string();
                let min_level =
                    CefrLevel::from_course_name(&cert.game_path_name).unwrap_or(CefrLevel::B2);

                let criteria = vec![
                    ClaimType::LanguageProficiency {
                        language,
                        min_level,
                    },
                    ClaimType::PerformanceThreshold { min_percentage: 90 },
                ];

                let on_success = {
                    let app_state = app_state.clone();
                    Callback::from(move |proof| {
                        let mut new_state = (*app_state).clone();
                        new_state.is_generating_proof = false;
                        new_state.set_zk_proof(proof);
                        app_state.set(new_state);
                    })
                };

                let on_error = {
                    let app_state = app_state.clone();
                    Callback::from(move |error: String| {
                        let mut new_state = (*app_state).clone();
                        new_state.set_error(error);
                        app_state.set(new_state);
                    })
                };

                zk_service.generate_combined_proof(
                    cert.clone(),
                    criteria,
                    "aleo".to_string(),
                    on_success,
                    on_error,
                );
            }
        })
    };

    let verify_proof = {
        let app_state = app_state.clone();
        let zk_service = zk_service.clone();

        Callback::from(move |_| {
            if let Some(proof) = &app_state.zk_proof {
                let mut new_state = (*app_state).clone();
                new_state.is_verifying_proof = true;
                new_state.clear_error();
                app_state.set(new_state);

                let on_success = {
                    let app_state = app_state.clone();
                    Callback::from(move |result| {
                        let mut new_state = (*app_state).clone();
                        new_state.is_verifying_proof = false;
                        new_state.set_verification_result(result);
                        app_state.set(new_state);
                    })
                };

                let on_error = {
                    let app_state = app_state.clone();
                    Callback::from(move |error: String| {
                        let mut new_state = (*app_state).clone();
                        new_state.set_error(error);
                        app_state.set(new_state);
                    })
                };

                zk_service.verify_proof(proof.clone(), on_success, on_error);
            }
        })
    };

    let copy_proof_data = {
        let app_state = app_state.clone();
        let copy_status = copy_status.clone();

        Callback::from(move |_| {
            if let Some(proof) = &app_state.zk_proof {
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
        let app_state = app_state.clone();
        Callback::from(move |_| {
            let mut new_state = (*app_state).clone();
            new_state.clear_error();
            app_state.set(new_state);
        })
    };

    // Corrected Callback for SampleCertificateIds
    let on_select_sample_certificate = {
        let certificate_id = certificate_id.clone();
        let app_state = app_state.clone(); // Clone the state handle
        let is_loading = is_loading.clone(); // Clone the state handle

        Callback::from(move |id: String| {
            certificate_id.set(id.clone());

            // Use the cloned id directly instead of accessing certificate_id again
            if id.trim().is_empty() {
                let mut new_state = (*app_state).clone();
                new_state.set_error("Please enter a certificate ID".to_string());
                app_state.set(new_state);
                return;
            }

            is_loading.set(true);
            let mut new_state = (*app_state).clone();
            new_state.clear_error();
            app_state.set(new_state);

            // Try to decode the certificate from base64
            match CertificateData::from_base64(&id) {
                Ok(certificate) => {
                    let mut new_state = (*app_state).clone();
                    new_state.set_certificate(certificate);
                    app_state.set(new_state);
                    is_loading.set(false);
                }
                Err(_) => {
                    // If base64 decoding fails, try as a simple lookup (simulate database lookup)
                    match simulate_certificate_lookup(&id) {
                        Some(certificate) => {
                            let mut new_state = (*app_state).clone();
                            new_state.set_certificate(certificate);
                            app_state.set(new_state);
                        }
                        None => {
                            let mut new_state = (*app_state).clone();
                            new_state.set_error(format!("Certificate not found with ID: {}", id));
                            app_state.set(new_state);
                        }
                    }
                    is_loading.set(false);
                }
            }
        })
    };

    html! {
        <PageLayout>
            <div class="max-w-6xl mx-auto">
                <div class="text-center mb-8">
                    <h1 class="text-3xl font-bold mb-2">{"🔍 Certificate Lookup & ZK Proof Generation"}</h1>
                    <p class="text-base-content/70">
                        {"Enter a certificate ID to load existing certificates and generate zero-knowledge proofs"}
                    </p>
                </div>

                <div class="grid lg:grid-cols-2 gap-8">
                    // Left Column: Certificate Lookup
                    <Card title="📋 Certificate Lookup">
                        <div class="space-y-4">
                            <InputField
                                label="Certificate ID"
                                icon="🆔"
                                placeholder="Enter certificate ID or paste base64 data..."
                                value={(*certificate_id).clone()}
                                onchange={on_certificate_id_change}
                                disabled={*is_loading}
                            />

                            <Button
                                variant="primary"
                                size="lg"
                                full_width={true}
                                loading={*is_loading}
                                disabled={*is_loading || certificate_id.trim().is_empty()}
                                onclick={load_certificate}
                            >
                                if *is_loading {
                                    {"Loading Certificate..."}
                                } else {
                                    {"🔍 Load Certificate"}
                                }
                            </Button>

                            <ErrorDisplay
                                error={app_state.error_message.clone()}
                                on_dismiss={on_dismiss_error.clone()}
                                dismissible={true}
                            />

                            // Certificate Preview
                            if let Some(cert) = &app_state.certificate_data {
                                <CertificatePreview certificate={cert.clone()} />
                            }

                            // Sample Certificate IDs
                            // Pass the new Callback here
                            <SampleCertificateIds on_select={on_select_sample_certificate} />
                        </div>
                    </Card>

                    // Right Column: ZK Proof Generation
                    <Card title="🔐 ZK Proof Generation">
                        <div class="space-y-6">
                            <ProofButtons
                                on_generate_language_proof={generate_language_proof}
                                on_generate_performance_proof={generate_performance_proof}
                                on_generate_combined_proof={generate_combined_proof}
                                is_generating={app_state.is_generating_proof}
                                has_certificate={app_state.certificate_data.is_some()}
                            />

                            <ProofDisplay
                                proof={app_state.zk_proof.clone()}
                                verification_result={app_state.verification_result.clone()}
                                on_verify={verify_proof}
                                on_copy_proof={copy_proof_data}
                                is_verifying={app_state.is_verifying_proof}
                                copy_status={(*copy_status).clone()}
                            />
                        </div>
                    </Card>
                </div>
            </div>
        </PageLayout>
    }
}

// Certificate preview component
#[derive(Properties, PartialEq)]
pub struct CertificatePreviewProps {
    pub certificate: konnektoren_core::certificates::CertificateData,
}

#[function_component(CertificatePreview)]
pub fn certificate_preview(props: &CertificatePreviewProps) -> Html {
    html! {
        <div class="card bg-base-200 p-4">
            <h3 class="font-semibold mb-3">{"📜 Certificate Details"}</h3>
            <div class="space-y-2 text-sm">
                <div class="flex justify-between">
                    <span class="font-medium">{"Student:"}</span>
                    <span>{&props.certificate.profile_name}</span>
                </div>
                <div class="flex justify-between">
                    <span class="font-medium">{"Course:"}</span>
                    <span>{&props.certificate.game_path_name}</span>
                </div>
                <div class="flex justify-between">
                    <span class="font-medium">{"Performance:"}</span>
                    <span class="badge badge-primary">{props.certificate.performance_percentage}{"%"}</span>
                </div>
                <div class="flex justify-between">
                    <span class="font-medium">{"Challenges:"}</span>
                    <span>{format!("{}/{}", props.certificate.solved_challenges, props.certificate.total_challenges)}</span>
                </div>
                <div class="flex justify-between">
                    <span class="font-medium">{"Date:"}</span>
                    <span>{props.certificate.date.format("%Y-%m-%d").to_string()}</span>
                </div>
            </div>
        </div>
    }
}

// Sample certificate IDs component
#[derive(Properties, PartialEq)]
pub struct SampleCertificateIdsProps {
    pub on_select: Callback<String>,
}

#[function_component(SampleCertificateIds)]
pub fn sample_certificate_ids(props: &SampleCertificateIdsProps) -> Html {
    let sample_certificates = get_sample_certificate_ids();

    html! {
        <div class="card bg-base-200 p-4">
            <h3 class="font-semibold mb-3">{"📋 Sample Certificate IDs"}</h3>
            <div class="space-y-2">
                {for sample_certificates.iter().map(|(name, id)| {
                    let id_clone = id.clone();
                    let on_select = props.on_select.clone();
                    html! {
                        <button
                            class="btn btn-sm btn-outline w-full justify-start"
                            onclick={move |_| on_select.emit(id_clone.clone())}
                        >
                            <span class="truncate">{name}</span>
                        </button>
                    }
                })}
            </div>
            <div class="text-xs text-base-content/70 mt-2">
                {"Click any sample to load the certificate"}
            </div>
        </div>
    }
}

// Simulate certificate lookup (replace with real database lookup)
fn simulate_certificate_lookup(id: &str) -> Option<CertificateData> {
    let samples = get_sample_certificate_ids();
    samples
        .iter()
        .find(|(_, sample_id)| sample_id == &id)
        .map(|(_, id)| {
            // For demo purposes, decode the base64 data
            CertificateData::from_base64(id).unwrap_or_else(|_| {
                // Fallback to creating a sample certificate
                use chrono::Utc;
                CertificateData::new(
                    "German_B2_Complete".to_string(),
                    50,
                    47,
                    "Sample Student".to_string(),
                    Utc::now(),
                )
            })
        })
}

// Sample certificate IDs for testing
fn get_sample_certificate_ids() -> Vec<(String, String)> {
    use chrono::Utc;

    let cert1 = CertificateData::new(
        "German_B2_Complete".to_string(),
        50,
        47, // Performance 94%
        "Alice Schmidt".to_string(),
        Utc::now(),
    );

    let cert2 = CertificateData::new(
        "Spanish_A2_Elementary".to_string(),
        30,
        28, // Performance ~93%
        "Bob Martinez".to_string(),
        Utc::now(),
    );

    let cert3 = CertificateData::new(
        "French_C1_Advanced".to_string(),
        60,
        58, // Performance ~97%
        "Carol Dubois".to_string(),
        Utc::now(),
    );

    vec![
        (
            "🇩🇪 German B2 - Alice Schmidt".to_string(),
            cert1.to_base64(),
        ),
        (
            "🇪🇸 Spanish A2 - Bob Martinez".to_string(),
            cert2.to_base64(),
        ),
        ("🇫🇷 French C1 - Carol Dubois".to_string(), cert3.to_base64()),
    ]
}
