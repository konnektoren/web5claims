use crate::components::certificate_image::Web5CertificateImage;
use crate::services::ZkService;
use crate::types::AppState;
use gloo::utils::document;
use wasm_bindgen::JsCast;
use web5claims::{CefrLevel, ClaimType};
use web_sys::{HtmlDocument, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CertificateDisplayProps {
    pub state: UseStateHandle<AppState>,
}

#[function_component(CertificateDisplay)]
pub fn certificate_display(props: &CertificateDisplayProps) -> Html {
    let zk_service = use_state(|| ZkService::new());
    let copy_status = use_state(|| None::<String>);

    let generate_language_proof = {
        let state = props.state.clone();
        let zk_service = zk_service.clone();

        Callback::from(move |_| {
            if let Some(cert) = &state.certificate_data {
                let mut new_state = (*state).clone();
                new_state.is_generating_proof = true;
                new_state.clear_error();
                state.set(new_state);

                // Extract language from certificate
                let language = cert
                    .game_path_name
                    .split('_')
                    .next()
                    .unwrap_or("German")
                    .to_string();
                let min_level =
                    CefrLevel::from_course_name(&cert.game_path_name).unwrap_or(CefrLevel::B2);

                let on_success = {
                    let state = state.clone();
                    Callback::from(move |proof| {
                        let mut new_state = (*state).clone();
                        new_state.is_generating_proof = false;
                        new_state.set_zk_proof(proof);
                        state.set(new_state);
                    })
                };

                let on_error = {
                    let state = state.clone();
                    Callback::from(move |error: String| {
                        let mut new_state = (*state).clone();
                        new_state.set_error(error);
                        state.set(new_state);
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
        let state = props.state.clone();
        let zk_service = zk_service.clone();

        Callback::from(move |_| {
            if let Some(cert) = &state.certificate_data {
                let mut new_state = (*state).clone();
                new_state.is_generating_proof = true;
                new_state.clear_error();
                state.set(new_state);

                let on_success = {
                    let state = state.clone();
                    Callback::from(move |proof| {
                        let mut new_state = (*state).clone();
                        new_state.is_generating_proof = false;
                        new_state.set_zk_proof(proof);
                        state.set(new_state);
                    })
                };

                let on_error = {
                    let state = state.clone();
                    Callback::from(move |error: String| {
                        let mut new_state = (*state).clone();
                        new_state.set_error(error);
                        state.set(new_state);
                    })
                };

                zk_service.generate_performance_proof(
                    cert.clone(),
                    90, // 90% threshold
                    "aleo".to_string(),
                    on_success,
                    on_error,
                );
            }
        })
    };

    let generate_combined_proof = {
        let state = props.state.clone();
        let zk_service = zk_service.clone();

        Callback::from(move |_| {
            if let Some(cert) = &state.certificate_data {
                let mut new_state = (*state).clone();
                new_state.is_generating_proof = true;
                new_state.clear_error();
                state.set(new_state);

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
                    let state = state.clone();
                    Callback::from(move |proof| {
                        let mut new_state = (*state).clone();
                        new_state.is_generating_proof = false;
                        new_state.set_zk_proof(proof);
                        state.set(new_state);
                    })
                };

                let on_error = {
                    let state = state.clone();
                    Callback::from(move |error: String| {
                        let mut new_state = (*state).clone();
                        new_state.set_error(error);
                        state.set(new_state);
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
                if let Ok(json) = serde_json::to_string_pretty(proof) {
                    copy_to_clipboard_simple(&json, copy_status.clone());
                } else {
                    copy_status.set(Some("❌ Failed to serialize proof".to_string()));
                }
            }
        })
    };

    let copy_certificate_data = {
        let state = props.state.clone();
        let copy_status = copy_status.clone();

        Callback::from(move |_| {
            if let Some(cert) = &state.certificate_data {
                let cert_data = cert.to_base64();
                copy_to_clipboard_simple(&cert_data, copy_status.clone());
            }
        })
    };

    match &props.state.certificate_data {
        Some(cert) => {
            // Convert date to string
            let formatted_date = cert.date.format("%Y-%m-%d").to_string();

            html! {
                            <div class="space-y-6">
                                // Copy Status Toast
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

                                // Error Display
                                if let Some(error) = &props.state.error_message {
                                    <div class="alert alert-error shadow-lg">
                                        <div>
                                            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current flex-shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                            </svg>
                                            <span>{error}</span>
                                        </div>
                                    </div>
                                }

                                // Certificate Image Display
                                <div class="card bg-gradient-to-br from-blue-50 to-indigo-100 border border-blue-200">
                                    <div class="card-body p-6">
                                        <h3 class="card-title text-xl mb-4 text-center">
                                            {"🎓 Certificate of Achievement"}
                                        </h3>

                                        <div class="flex justify-center mb-4">
                                            <Web5CertificateImage certificate_data={cert.clone()} />
                                        </div>

                                        // Certificate Actions
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
                                                href={format!("data:text/plain;charset=utf-8,{}", cert.to_base64())}
                                                download="certificate.txt"
                                            >
                                                {"💾 Download"}
                                            </a>
                                        </div>
                                    </div>
                                </div>

                                // Certificate Summary Stats
                                <div class="stats stats-vertical lg:stats-horizontal shadow w-full">
                                    <div class="stat">
                                        <div class="stat-figure text-secondary">{"🎯"}</div>
                                        <div class="stat-title">{"Performance"}</div>
                                        <div class="stat-value text-primary">{cert.performance_percentage}{"%"}</div>
                                        <div class="stat-desc">
                                            {format!("{}/{} challenges", cert.solved_challenges, cert.total_challenges)}
                                        </div>
                                    </div>

                                    <div class="stat">
                                        <div class="stat-figure text-secondary">{"🌍"}</div>
                                        <div class="stat-title">{"Course"}</div>
                                        <div class="stat-value text-secondary text-lg">
                                            {cert.game_path_name.replace("_", " ")}
                                        </div>
                                        <div class="stat-desc">{formatted_date}</div>
                                    </div>

                                    <div class="stat">
                                        <div class="stat-figure text-secondary">{"👤"}</div>
                                        <div class="stat-title">{"Student"}</div>
                                        <div class="stat-value text-accent text-lg">{&cert.profile_name}</div>
                                        <div class="stat-desc">{"Verified learner"}</div>
                                    </div>
                                </div>

                                // ZK Proof Generation Section
                                <div class="card bg-gradient-to-r from-primary to-secondary text-primary-content">
                                    <div class="card-body">
                                        <h3 class="card-title text-xl mb-3">
                                            {"🔐 Zero-Knowledge Proof Generation"}
                                        </h3>
                                        <p class="text-sm opacity-90 mb-4">
                                            {"Generate cryptographic proofs to verify your skills without revealing private data."}
                                        </p>

                                        // Proof Type Selection
                                        <div class="grid grid-cols-1 md:grid-cols-3 gap-3 mb-4">
                                            <button
                                                class={classes!(
                                                    "btn", "btn-accent",
                                                    if props.state.is_generating_proof { "loading" } else { "" }
                                                )}
                                                onclick={generate_language_proof}
                                                disabled={props.state.is_generating_proof}
                                            >
                                                {"🌍 Language Proof"}
                                            </button>
                                            <button
                                                class={classes!(
                                                    "btn", "btn-accent",
                                                    if props.state.is_generating_proof { "loading" } else { "" }
                                                )}
                                                onclick={generate_performance_proof}
                                                disabled={props.state.is_generating_proof}
                                            >
                                                {"📊 Performance Proof"}
                                            </button>
                                            <button
                                                class={classes!(
                                                    "btn", "btn-accent",
                                                    if props.state.is_generating_proof { "loading" } else { "" }
                                                )}
                                                onclick={generate_combined_proof}
                                                disabled={props.state.is_generating_proof}
                                            >
                                                {"🔗 Combined Proof"}
                                            </button>
                                        </div>

                                        if props.state.is_generating_proof {
                                            <div class="bg-base-100 bg-opacity-20 rounded-lg p-4 mb-4">
                                                <div class="flex items-center justify-center space-x-2">
                                                    <span class="loading loading-spinner loading-sm"></span>
                                                    <span>{"Generating ZK proof with Aleo circuits..."}</span>
                                                </div>
                                            </div>
                                        }
                                    </div>
                                </div>

                                // ZK Proof Display Section
                                if let Some(proof) = &props.state.zk_proof {
                                    <div class="card bg-success text-success-content">
                                        <div class="card-body">
                                            <h3 class="card-title">{"✅ ZK Proof Generated!"}</h3>
                                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                                                <div>
                                                    <p class="text-sm opacity-90 mb-2">{"Proof Details:"}</p>
                                                    <ul class="text-xs space-y-1">
                                                        <li>{"• Proof ID: "}{&proof.proof_id[..8]}{"..."}</li>
                                                        <li>{"• Circuit: "}{&proof.proof_data.circuit_id}</li>
                                                        <li>{"• Platform: "}{&proof.metadata.platform}</li>
                                                        <li>{"• Valid: "}{if proof.public_inputs.verification_result { "✅" } else { "❌" }}</li>
                                                    </ul>
                                                </div>
                                                <div>
                                                    <p class="text-sm opacity-90 mb-2">{"Public Claims:"}</p>
                                                    <ul class="text-xs space-y-1">
                                                        {for proof.public_inputs.requirements.iter().map(|(key, value)| {
                                                            html! {
                                                                <li>{"• "}{key}{": "}{value}</li>
                                                            }
                                                        })}
                                                    </ul>
                                                </div>
                                            </div>

                                            <div class="flex gap-2">
                                                <button
                                                    class={classes!(
                                                        "btn", "btn-outline", "btn-sm",
                                                        if props.state.is_verifying_proof { "loading" } else { "" }
                                                    )}
                                                    onclick={verify_proof}
                                                    disabled={props.state.is_verifying_proof}
                                                >
                                                    {"🔍 Verify Proof"}
                                                </button>
                                                <button
                                                    class="btn btn-outline btn-sm"
                                                    onclick={copy_proof_data}
                                                >
                                                    {"📋 Copy JSON"}
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                }

                                // Verification Result Display
                                if let Some(result) = &props.state.verification_result {
                                    <div class={classes!(
                                        "card",
                                        if result.is_valid && result.requirements_met {
                                            "bg-success text-success-content"
                                        } else {
                                            "bg-error text-error-content"
                                        }
                                    )}>
                                        <div class="card-body">
                                            <h3 class="card-title">
                                                {if result.is_valid && result.requirements_met {
                                                    "🎉 Verification Successful!"
                                                } else {
                                                    "❌ Verification Failed"
                                                }}
                                            </h3>
                                            <div class="text-sm">
                                                <p>{"Valid Proof: "}{if result.is_valid { "✅" } else { "❌" }}</p>
                                                <p>{"Requirements Met: "}{if result.requirements_met { "✅" } else { "❌" }}</p>
                                                <p>{"Platform: "}{&result.details.platform}</p>
                                                <p>{"Verified At: "}{result.details.verified_at.format("%Y-%m-%d %H:%M:%S")}</p>
                                            </div>

                                            if !result.warnings.is_empty() {
                                                <div class="mt-2">
                                                    <p class="text-sm font-semibold">{"Warnings:"}</p>
                                                    <ul class="text-xs">
                                                        {for result.warnings.iter().map(|warning| {
                                                            html! { <li>{"• "}{warning}</li> }
                                                        })}
                                                    </ul>
                                                </div>
                                            }
                                        </div>
                                    </div>
                                }

                                // Technical Details (Collapsible)
                                <div class="collapse collapse-arrow bg-base-200">
                                    <input type="checkbox" />
                                    <div class="collapse-title text-sm font-medium">
                                        {"🔧 Technical Details & ZK Circuit Preview"}
                                    </div>
                                    <div class="collapse-content">
                                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                                            <div>
                                                <h4 class="font-semibold mb-2">{"📜 Certificate Structure:"}</h4>
                                                <pre class="text-xs bg-base-300 p-3 rounded overflow-auto max-h-64">
                                                    {format!("{:#?}", cert)}
                                                </pre>
                                            </div>

                                            <div>
                                                <h4 class="font-semibold mb-2">{"⚙️ Aleo ZK Circuit:"}</h4>
                                                <pre class="text-xs bg-base-300 p-3 rounded overflow-auto max-h-64">
            {r#"// Language Proficiency Verification
transition verify_language_skill(
    private cert_level: u8,       // 4 (B2)
    private performance: u8,      // 94
    private student_hash: field,  // Private
    public min_level: u8,         // 4 (B2)
    public language: field,       // "German"
) -> bool {
    let level_check = cert_level >= min_level;
    let performance_check = performance >= 70u8;

    return level_check && performance_check;
}

// What gets proven: ✅ B2+ German
// What stays private: ❌ Exact score (94%)"#}
                                                </pre>
                                            </div>
                                        </div>

                                        if let Some(proof) = &props.state.zk_proof {
                                            <div class="mt-4">
                                                <h4 class="font-semibold mb-2">{"🔐 Generated ZK Proof:"}</h4>
                                                <pre class="text-xs bg-base-300 p-3 rounded overflow-auto max-h-32">
                                                    {serde_json::to_string_pretty(proof).unwrap_or_else(|_| "Error serializing proof".to_string())}
                                                </pre>
                                            </div>
                                        }
                                    </div>
                                </div>
                            </div>
                        }
        }
        None => {
            html! {
                <div class="hero min-h-[400px] bg-base-200 rounded-lg">
                    <div class="hero-content text-center">
                        <div class="max-w-md">
                            <div class="text-6xl mb-4">{"📝"}</div>
                            <h3 class="text-2xl font-bold mb-4">{"Ready for ZK Magic!"}</h3>
                            <p class="text-gray-600 mb-6">
                                {"Generate a language learning certificate first, then create zero-knowledge proofs to verify your skills without revealing personal information."}
                            </p>
                            <div class="flex flex-wrap gap-2 justify-center text-sm">
                                <div class="badge badge-primary">{"🔐 Privacy-First"}</div>
                                <div class="badge badge-secondary">{"⚡ Aleo Powered"}</div>
                                <div class="badge badge-accent">{"🏆 ZK Hack Berlin"}</div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}

// Simplified clipboard function - Fixed lifetime issue by taking owned String
fn copy_to_clipboard_simple(text: &str, copy_status: UseStateHandle<Option<String>>) {
    // Convert &str to owned String to fix lifetime issue
    let text_owned = text.to_string();

    if let Some(window) = web_sys::window() {
        // clipboard() returns Clipboard directly
        let clipboard = window.navigator().clipboard();
        let promise = clipboard.write_text(&text_owned);
        let copy_status = copy_status.clone();

        wasm_bindgen_futures::spawn_local(async move {
            match wasm_bindgen_futures::JsFuture::from(promise).await {
                Ok(_) => {
                    copy_status.set(Some("✅ Copied to clipboard!".to_string()));
                    // Clear message after 3 seconds
                    let copy_status_clear = copy_status.clone();
                    gloo::timers::callback::Timeout::new(3000, move || {
                        copy_status_clear.set(None);
                    })
                    .forget();
                }
                Err(_) => {
                    // Fall back to legacy method
                    fallback_copy(&text_owned, copy_status);
                }
            }
        });
    } else {
        copy_status.set(Some("❌ Cannot access clipboard".to_string()));
    }
}

// Fallback copy function - Fixed style() API call and lifetime issue
fn fallback_copy(text: &str, copy_status: UseStateHandle<Option<String>>) {
    let document = document();

    // Create temporary textarea
    if let Ok(textarea) = document.create_element("textarea") {
        if let Ok(textarea) = textarea.dyn_into::<HtmlTextAreaElement>() {
            textarea.set_value(text);

            // Style the textarea to be invisible - style() returns CssStyleDeclaration directly
            let style = textarea.style();
            let _ = style.set_property("position", "fixed");
            let _ = style.set_property("left", "-9999px");
            let _ = style.set_property("opacity", "0");

            // Add to DOM, select, copy, and remove
            if let Some(body) = document.body() {
                if body.append_child(&textarea).is_ok() {
                    textarea.select();

                    // Try to copy
                    if let Ok(html_doc) = document.dyn_into::<HtmlDocument>() {
                        if html_doc.exec_command("copy").unwrap_or(false) {
                            copy_status.set(Some("✅ Copied to clipboard!".to_string()));
                        } else {
                            copy_status.set(Some("❌ Copy failed".to_string()));
                        }
                    } else {
                        copy_status.set(Some("❌ Copy not supported".to_string()));
                    }

                    let _ = body.remove_child(&textarea);

                    // Clear message after 3 seconds
                    let copy_status_clear = copy_status.clone();
                    gloo::timers::callback::Timeout::new(3000, move || {
                        copy_status_clear.set(None);
                    })
                    .forget();
                }
            }
        }
    }
}
