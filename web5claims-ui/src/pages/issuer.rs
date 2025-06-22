use crate::components::{
    certificate_display::CertificateDisplay,
    certificate_form::CertificateForm,
    layout::{PageLayout, TwoColumnLayout},
    ui::Card,
    wallet::{AleoWallet, WalletInfo, ZkPassportWallet},
};
use crate::router::Route;
use crate::services::zkpassport_service::{PassportData, ZkPassportProof};
use crate::types::AppState;
use web_sys::{window, UrlSearchParams};
use yew::prelude::*;

#[function_component(IssuerPage)]
pub fn issuer_page() -> Html {
    let app_state = use_state(|| AppState::default());
    let wallet_info = use_state(|| None::<WalletInfo>);
    let wallet_error = use_state(|| None::<String>);
    let verified_name = use_state(|| None::<String>);
    let verification_status = use_state(|| None::<String>);

    // Add ZK Passport state
    let passport_data = use_state(|| None::<PassportData>);
    let zkpassport_proof = use_state(|| None::<ZkPassportProof>);

    // Check for URL parameters from ZKPass verification
    use_effect_with((), {
        let verified_name = verified_name.clone();
        let verification_status = verification_status.clone();

        move |_| {
            if let Some(window) = window() {
                if let Ok(search) = window.location().search() {
                    if !search.is_empty() {
                        log::info!("URL search params: {}", search);

                        if let Ok(params) = UrlSearchParams::new_with_str(&search) {
                            // Check for verified_name parameter
                            if let Some(name) = params.get("verified_name") {
                                log::info!("Found verified name: {}", name);
                                verified_name.set(Some(name.clone()));

                                // Check for additional verification info
                                let mut status_parts = vec![];

                                if let Some(age_verified) = params.get("verified_age") {
                                    if age_verified == "true" {
                                        status_parts.push("Age (18+)".to_string());
                                    }
                                }

                                if params.get("verified_name").is_some() {
                                    status_parts.push(format!("Name ({})", name));
                                }

                                if let Some(timestamp) = params.get("verification_timestamp") {
                                    status_parts.push(format!(
                                        "Verified: {}",
                                        timestamp
                                            .chars()
                                            .take(19)
                                            .collect::<String>()
                                            .replace('T', " ")
                                    ));
                                }

                                if !status_parts.is_empty() {
                                    verification_status.set(Some(status_parts.join(" • ")));
                                }
                            }
                        }
                    }
                }
            }
            || ()
        }
    });

    let on_wallet_connect = {
        let wallet_info = wallet_info.clone();
        let wallet_error = wallet_error.clone();

        Callback::from(move |info: WalletInfo| {
            log::info!("Wallet connected: {}", info.address);
            wallet_info.set(Some(info));
            wallet_error.set(None);
        })
    };

    let on_wallet_disconnect = {
        let wallet_info = wallet_info.clone();

        Callback::from(move |_| {
            log::info!("Wallet disconnected");
            wallet_info.set(None);
        })
    };

    let on_wallet_error = {
        let wallet_error = wallet_error.clone();

        Callback::from(move |error: String| {
            log::error!("Wallet error: {}", error);
            wallet_error.set(Some(error));
        })
    };

    // Add ZK Passport callbacks
    let on_passport_scanned = {
        let passport_data = passport_data.clone();
        Callback::from(move |data: PassportData| {
            log::info!("Passport scanned: {:?}", data);
            passport_data.set(Some(data));
        })
    };

    let on_zkpassport_proof = {
        let zkpassport_proof = zkpassport_proof.clone();
        Callback::from(move |proof: ZkPassportProof| {
            log::info!("ZK Passport proof generated");
            zkpassport_proof.set(Some(proof));
        })
    };

    let on_zkpassport_error = {
        let wallet_error = wallet_error.clone();
        Callback::from(move |error: String| {
            log::error!("ZK Passport error: {}", error);
            wallet_error.set(Some(error));
        })
    };

    let clear_verification = {
        let verified_name = verified_name.clone();
        let verification_status = verification_status.clone();

        Callback::from(move |_| {
            verified_name.set(None);
            verification_status.set(None);

            // Clear URL parameters
            if let Some(window) = window() {
                if let Ok(history) = window.history() {
                    let _ = history.replace_state_with_url(
                        &wasm_bindgen::JsValue::NULL,
                        "",
                        Some("/issuer"),
                    );
                }
            }
        })
    };

    let left_content = html! {
        <div class="space-y-6">
            // ZKPass Verification Status (if verified)
            {if let (Some(name), Some(status)) = (&*verified_name, &*verification_status) {
                html! {
                    <Card title="🛂 ZKPassport Verification Status">
                        <div class="bg-success/10 border border-success/20 rounded-lg p-4">
                            <div class="flex items-start justify-between">
                                <div>
                                    <h3 class="text-lg font-semibold text-success mb-2">
                                        {"✅ Identity Verified"}
                                    </h3>
                                    <div class="text-sm text-base-content/70 mb-2">
                                        {status}
                                    </div>
                                    <p class="text-xs text-base-content/60">
                                        {"Your certificate will be issued with verified identity for enhanced credibility."}
                                    </p>
                                </div>
                                <button
                                    class="btn btn-ghost btn-sm"
                                    onclick={clear_verification}
                                    title="Clear verification and start over"
                                >
                                    {"×"}
                                </button>
                            </div>
                        </div>
                    </Card>
                }
            } else {
                html! { <></> }
            }}

            // Certificate Creation Section
            <Card title="🎓 Create Language Certificate">
                <CertificateForm
                    state={app_state.clone()}
                    verified_name={(*verified_name).clone()}
                />
            </Card>

            // ZK Passport Section
            <Card title="🛂 Identity Verification Options">
                <div class="space-y-4">
                    {if verified_name.is_none() {
                        html! {
                            <>
                                <div class="alert alert-info">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                    </svg>
                                    <div>
                                        <div class="font-semibold">{"Enhance Certificate Credibility"}</div>
                                        <div class="text-sm">
                                            {"Verify your identity to issue certificates with enhanced trust and credibility."}
                                        </div>
                                    </div>
                                </div>

                                <div class="grid md:grid-cols-2 gap-4">
                                    // Existing ZK Passport Wallet
                                    <div class="card bg-base-200 p-4">
                                        <h4 class="font-semibold mb-2">{"📱 ZK Passport SDK"}</h4>
                                        <p class="text-sm text-base-content/70 mb-3">
                                            {"Basic identity verification integrated into the app"}
                                        </p>
                                        <ZkPassportWallet
                                            on_passport_scanned={on_passport_scanned}
                                            on_proof_generated={on_zkpassport_proof}
                                            on_error={on_zkpassport_error}
                                        />
                                    </div>

                                    // New ZKPass External App
                                    <div class="card bg-base-200 p-4">
                                        <h4 class="font-semibold mb-2">{"🛂 ZKPass Advanced"}</h4>
                                        <p class="text-sm text-base-content/70 mb-3">
                                            {"Full-featured passport scanning with enhanced privacy"}
                                        </p>
                                        <a
                                            href={Route::zkpass_external_url()}
                                            target="_blank"
                                            class="btn btn-primary btn-sm w-full"
                                        >
                                            {"🚀 Launch ZKPass"}
                                        </a>
                                        <p class="text-xs text-base-content/60 mt-2">
                                            {"Opens in new tab with advanced verification features"}
                                        </p>
                                    </div>
                                </div>
                            </>
                        }
                    } else {
                        html! {
                            <div class="alert alert-success">
                                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                </svg>
                                <div>
                                    <div class="font-semibold">{"Identity Verification Complete"}</div>
                                    <div class="text-sm">
                                        {"Your certificates will now be issued with verified identity for enhanced trust."}
                                    </div>
                                </div>
                            </div>
                        }
                    }}

                    <div class="alert alert-info">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                        <div>
                            <div class="font-semibold">{"Choose Your Verification Method"}</div>
                            <div class="text-sm">
                                {"Use the integrated ZK Passport for basic verification, or ZKPass for advanced passport scanning with enhanced privacy features."}
                            </div>
                        </div>
                    </div>
                </div>
            </Card>

            // Leo Wallet Connection Section (collapsed by default)
            <details class="collapse bg-base-200">
                <summary class="collapse-title">
                    <div class="flex items-center space-x-2">
                        <span>{"🦁"}</span>
                        <span class="font-medium">{"Leo Wallet Connection (Optional)"}</span>
                        <div class="badge badge-warning badge-sm">{"Beta"}</div>
                    </div>
                </summary>
                <div class="collapse-content">
                    <div class="space-y-4 pt-4">
                        <div class="alert alert-info">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                            <div>
                                <div class="font-semibold">{"Optional: Connect your Leo Wallet"}</div>
                                <div class="text-sm">{"For future on-chain ZK proof functionality (currently in development)"}</div>
                            </div>
                        </div>

                        <AleoWallet
                            on_connect={on_wallet_connect}
                            on_disconnect={on_wallet_disconnect}
                            on_error={on_wallet_error}
                        />

                        {if let Some(error) = &*wallet_error {
                            html! {
                                <div class="alert alert-warning">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
                                    </svg>
                                    <div>
                                        <div class="font-semibold">{"Wallet Connection Issue"}</div>
                                        <div class="text-sm">{error}</div>
                                        <div class="text-xs mt-1">{"You can still generate ZK proofs without wallet connection"}</div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! { <></> }
                        }}
                    </div>
                </div>
            </details>
        </div>
    };

    let right_content = html! {
        <Card title="🔐 ZK Proof Generation">
            <div class="space-y-4">
                <div class="alert alert-success">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    <div>
                        <div class="font-semibold">{"Ready to Generate ZK Proofs"}</div>
                        <div class="text-sm">
                            {"Create certificates and generate zero-knowledge proofs locally"}
                            {if verified_name.is_some() {
                                " with enhanced identity verification"
                            } else {
                                ""
                            }}
                        </div>
                    </div>
                </div>

                <CertificateDisplay
                    state={app_state.clone()}
                    wallet_info={(*wallet_info).clone()}
                />
            </div>
        </Card>
    };

    html! {
        <PageLayout>
            <div class="mb-6">
                <h1 class="text-3xl font-bold text-center mb-2">{"Certificate Issuer"}</h1>
                <p class="text-center text-base-content/70">
                    {"Create language learning certificates and generate zero-knowledge proofs with optional identity verification"}
                </p>
            </div>
            <TwoColumnLayout
                left_content={left_content}
                right_content={right_content}
                breakpoint="lg"
                gap={8}
            />
        </PageLayout>
    }
}
