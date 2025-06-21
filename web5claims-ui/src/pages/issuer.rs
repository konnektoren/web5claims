use crate::components::{
    certificate_display::CertificateDisplay,
    certificate_form::CertificateForm,
    layout::{PageLayout, TwoColumnLayout},
    ui::Card,
    wallet::{AleoWallet, WalletInfo, ZkPassportWallet}, // Add ZkPassportWallet import
};
use crate::services::zkpassport_service::{PassportData, ZkPassportProof}; // Add these imports
use crate::types::AppState;
use yew::prelude::*;

#[function_component(IssuerPage)]
pub fn issuer_page() -> Html {
    let app_state = use_state(|| AppState::default());
    let wallet_info = use_state(|| None::<WalletInfo>);
    let wallet_error = use_state(|| None::<String>);

    // Add ZK Passport state
    let passport_data = use_state(|| None::<PassportData>);
    let zkpassport_proof = use_state(|| None::<ZkPassportProof>);

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

    let left_content = html! {
        <div class="space-y-6">
            // Certificate Creation Section
            <Card title="🎓 Create Language Certificate">
                <CertificateForm state={app_state.clone()} />
            </Card>

            // ZK Passport Section - NEW!
            <Card title="🛂 ZK Passport Identity Verification">
                <div class="space-y-4">
                    <div class="alert alert-info">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                        <div>
                            <div class="font-semibold">{"Enhanced Identity Verification"}</div>
                            <div class="text-sm">{"Use ZK Passport to verify your identity without revealing personal information"}</div>
                        </div>
                    </div>

                    <ZkPassportWallet
                        on_passport_scanned={on_passport_scanned}
                        on_proof_generated={on_zkpassport_proof}
                        on_error={on_zkpassport_error}
                    />

                    // Show passport data if available
                    {if let Some(data) = &*passport_data {
                        html! {
                            <div class="alert alert-success">
                                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                </svg>
                                <div>
                                    <div class="font-semibold">{"Identity Verified"}</div>
                                    <div class="text-sm">{format!("Nationality: {} | Age 18+: {}", data.nationality, if data.age_over_18 { "Yes" } else { "No" })}</div>
                                </div>
                            </div>
                        }
                    } else {
                        html! { <></> }
                    }}

                    // Show ZK proof status if available
                    {if let Some(_proof) = &*zkpassport_proof {
                        html! {
                            <div class="alert alert-success">
                                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                </svg>
                                <div>
                                    <div class="font-semibold">{"ZK Identity Proof Ready"}</div>
                                    <div class="text-sm">{"Your identity proof can be used to enhance certificate credibility"}</div>
                                </div>
                            </div>
                        }
                    } else {
                        html! { <></> }
                    }}
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
                            {if passport_data.is_some() {
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
