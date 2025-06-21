use crate::components::{layout::PageLayout, ui::Card, wallet::ZkPassportWallet};
use crate::services::zkpassport_service::{PassportData, ZkPassportProof};
use yew::prelude::*;

#[function_component(ZkPassportPage)]
pub fn zkpassport_page() -> Html {
    let passport_data = use_state(|| None::<PassportData>);
    let zkpassport_proof = use_state(|| None::<ZkPassportProof>);
    let error_message = use_state(|| None::<String>);

    let on_passport_scanned = {
        let passport_data = passport_data.clone();
        let error_message = error_message.clone();
        Callback::from(move |data: PassportData| {
            log::info!("Passport scanned: {:?}", data);
            passport_data.set(Some(data));
            error_message.set(None);
        })
    };

    let on_zkpassport_proof = {
        let zkpassport_proof = zkpassport_proof.clone();
        let error_message = error_message.clone();
        Callback::from(move |proof: ZkPassportProof| {
            log::info!("ZK Passport proof generated");
            zkpassport_proof.set(Some(proof));
            error_message.set(None);
        })
    };

    let on_zkpassport_error = {
        let error_message = error_message.clone();
        Callback::from(move |error: String| {
            log::error!("ZK Passport error: {}", error);
            error_message.set(Some(error));
        })
    };

    html! {
        <PageLayout>
            <div class="max-w-4xl mx-auto">
                <div class="text-center mb-8">
                    <h1 class="text-3xl font-bold mb-2">{"🛂 ZK Passport Identity Verification"}</h1>
                    <p class="text-base-content/70">
                        {"Verify your identity using zero-knowledge proofs without revealing personal information"}
                    </p>
                </div>

                <div class="grid lg:grid-cols-2 gap-8">
                    // ZK Passport Scanner
                    <Card title="📱 Passport Scanner">
                        <ZkPassportWallet
                            on_passport_scanned={on_passport_scanned}
                            on_proof_generated={on_zkpassport_proof}
                            on_error={on_zkpassport_error}
                        />
                    </Card>

                    // Results and Information
                    <Card title="📊 Verification Results">
                        <div class="space-y-4">
                            {if let Some(error) = &*error_message {
                                html! {
                                    <div class="alert alert-error">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                        </svg>
                                        <span>{error}</span>
                                    </div>
                                }
                            } else {
                                html! { <></> }
                            }}

                            {if let Some(data) = &*passport_data {
                                html! {
                                    <div class="card bg-base-200 p-4">
                                        <h3 class="font-semibold mb-3">{"🆔 Identity Information"}</h3>
                                        <div class="space-y-2 text-sm">
                                            <div class="flex justify-between">
                                                <span class="font-medium">{"Nationality:"}</span>
                                                <span class="badge badge-primary">{&data.nationality}</span>
                                            </div>
                                            <div class="flex justify-between">
                                                <span class="font-medium">{"Age Verification:"}</span>
                                                <span class={if data.age_over_18 { "badge badge-success" } else { "badge badge-warning" }}>
                                                    {if data.age_over_18 { "18+" } else { "Under 18" }}
                                                </span>
                                            </div>
                                            <div class="flex justify-between">
                                                <span class="font-medium">{"Document Type:"}</span>
                                                <span>{&data.document_type}</span>
                                            </div>
                                            <div class="flex justify-between">
                                                <span class="font-medium">{"Expiry Date:"}</span>
                                                <span>{&data.expiry_date}</span>
                                            </div>
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {
                                    <div class="alert alert-info">
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                        </svg>
                                        <span>{"Scan your passport to see identity verification results"}</span>
                                    </div>
                                }
                            }}

                            {if let Some(proof) = &*zkpassport_proof {
                                html! {
                                    <div class="card bg-success text-success-content p-4">
                                        <h3 class="font-semibold mb-3">{"🔐 ZK Proof Generated"}</h3>
                                        <div class="space-y-2 text-sm">
                                            <div>{"✅ Identity verified without revealing personal data"}</div>
                                            <div>{"✅ Zero-knowledge proof generated successfully"}</div>
                                            <div>{"✅ Proof can be used for anonymous verification"}</div>
                                            <div class="mt-3">
                                                <span class="font-medium">{"Proof Length: "}</span>
                                                <span class="badge badge-accent">{proof.proof.len()}{" chars"}</span>
                                            </div>
                                            <div>
                                                <span class="font-medium">{"Public Signals: "}</span>
                                                <span class="badge badge-accent">{proof.public_signals.len()}{" signals"}</span>
                                            </div>
                                        </div>
                                    </div>
                                }
                            } else {
                                html! { <></> }
                            }}
                        </div>
                    </Card>
                </div>

                // How it works section
                <div class="mt-12">
                    <Card title="❓ How ZK Passport Works">
                        <div class="grid md:grid-cols-3 gap-6">
                            <div class="text-center">
                                <div class="text-4xl mb-2">{"📱"}</div>
                                <h3 class="font-semibold mb-2">{"1. Scan Passport"}</h3>
                                <p class="text-sm text-base-content/70">
                                    {"Use your device to scan the NFC chip in your passport securely"}
                                </p>
                            </div>
                            <div class="text-center">
                                <div class="text-4xl mb-2">{"🔐"}</div>
                                <h3 class="font-semibold mb-2">{"2. Generate Proof"}</h3>
                                <p class="text-sm text-base-content/70">
                                    {"Create a zero-knowledge proof of your identity without revealing personal details"}
                                </p>
                            </div>
                            <div class="text-center">
                                <div class="text-4xl mb-2">{"✅"}</div>
                                <h3 class="font-semibold mb-2">{"3. Verify Anonymously"}</h3>
                                <p class="text-sm text-base-content/70">
                                    {"Use the proof to verify your identity claims without exposing private information"}
                                </p>
                            </div>
                        </div>
                    </Card>
                </div>
            </div>
        </PageLayout>
    }
}
