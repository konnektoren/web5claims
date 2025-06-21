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

    let generate_language_proof = {
        let state = props.state.clone();
        let zk_service = zk_service.clone();
        let wallet_address = props.wallet_info.as_ref().map(|w| w.address.clone());

        Callback::from(move |_| {
            if let Some(cert) = &state.certificate_data {
                let mut new_state = (*state).clone();
                new_state.is_generating_proof = true;
                new_state.clear_error();
                state.set(new_state);

                if let Some(address) = &wallet_address {
                    log::info!("Generating proof with wallet: {}", address);
                } else {
                    log::info!("Generating proof locally (no wallet)");
                }

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
                    "local".to_string(), // Use "local" instead of "aleo"
                    on_success,
                    on_error,
                );
            }
        })
    };

    let generate_performance_proof = {
        let state = props.state.clone();
        let zk_service = zk_service.clone();
        let wallet_address = props.wallet_info.as_ref().map(|w| w.address.clone());

        Callback::from(move |_| {
            if let Some(cert) = &state.certificate_data {
                let mut new_state = (*state).clone();
                new_state.is_generating_proof = true;
                new_state.clear_error();
                state.set(new_state);

                if let Some(address) = &wallet_address {
                    log::info!("Generating performance proof with wallet: {}", address);
                } else {
                    log::info!("Generating performance proof locally (no wallet)");
                }

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
                    90,
                    "local".to_string(), // Use "local" instead of "aleo"
                    on_success,
                    on_error,
                );
            }
        })
    };

    let generate_combined_proof = {
        let state = props.state.clone();
        let zk_service = zk_service.clone();
        let wallet_address = props.wallet_info.as_ref().map(|w| w.address.clone());

        Callback::from(move |_| {
            if let Some(cert) = &state.certificate_data {
                let mut new_state = (*state).clone();
                new_state.is_generating_proof = true;
                new_state.clear_error();
                state.set(new_state);

                if let Some(address) = &wallet_address {
                    log::info!("Generating combined proof with wallet: {}", address);
                } else {
                    log::info!("Generating combined proof locally (no wallet)");
                }

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
                    "local".to_string(), // Use "local" instead of "aleo"
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
