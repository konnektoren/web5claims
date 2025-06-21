use crate::services::zkpassport_service::{PassportData, ZkPassportProof, ZkPassportService};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ZkPassportWalletProps {
    #[prop_or_default]
    pub on_passport_scanned: Callback<PassportData>,
    #[prop_or_default]
    pub on_proof_generated: Callback<ZkPassportProof>,
    #[prop_or_default]
    pub on_error: Callback<String>,
    #[prop_or_default]
    pub class: String,
}

#[derive(Clone, PartialEq)]
enum WalletState {
    Idle,
    Initializing,
    Ready,
    Scanning,
    GeneratingProof,
    Error(String),
}

#[function_component(ZkPassportWallet)]
pub fn zkpassport_wallet(props: &ZkPassportWalletProps) -> Html {
    let service = use_state(|| ZkPassportService::new());
    let wallet_state = use_state(|| WalletState::Idle);
    let passport_data = use_state(|| None::<PassportData>);
    let current_proof = use_state(|| None::<ZkPassportProof>);

    // Initialize SDK on component mount
    {
        let service = service.clone();
        let wallet_state = wallet_state.clone();
        let on_error = props.on_error.clone();

        use_effect_with((), move |_| {
            if service.is_available() {
                wallet_state.set(WalletState::Initializing);

                let service_clone = (*service).clone();
                let wallet_state_clone = wallet_state.clone();
                let on_error_clone = on_error.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    match service_clone.initialize().await {
                        Ok(_) => {
                            wallet_state_clone.set(WalletState::Ready);
                        }
                        Err(e) => {
                            wallet_state_clone.set(WalletState::Error(e.clone()));
                            on_error_clone.emit(e);
                        }
                    }
                });
            } else {
                wallet_state.set(WalletState::Error(
                    "ZK Passport SDK not available".to_string(),
                ));
                on_error
                    .emit("ZK Passport SDK not available. Please refresh the page.".to_string());
            }
            || ()
        });
    }

    let scan_passport = {
        let service = service.clone();
        let wallet_state = wallet_state.clone();
        let passport_data = passport_data.clone();
        let on_passport_scanned = props.on_passport_scanned.clone();
        let on_error = props.on_error.clone();

        Callback::from(move |_| {
            wallet_state.set(WalletState::Scanning);

            let on_success = {
                let wallet_state = wallet_state.clone();
                let passport_data = passport_data.clone();
                let on_passport_scanned = on_passport_scanned.clone();

                Callback::from(move |data: PassportData| {
                    passport_data.set(Some(data.clone()));
                    wallet_state.set(WalletState::Ready);
                    on_passport_scanned.emit(data);
                })
            };

            let on_error_cb = {
                let wallet_state = wallet_state.clone();
                let on_error = on_error.clone();

                Callback::from(move |error: String| {
                    wallet_state.set(WalletState::Error(error.clone()));
                    on_error.emit(error);
                })
            };

            service.scan_passport_async(on_success, on_error_cb);
        })
    };

    let generate_identity_proof = {
        let service = service.clone();
        let wallet_state = wallet_state.clone();
        let passport_data = passport_data.clone();
        let current_proof = current_proof.clone();
        let on_proof_generated = props.on_proof_generated.clone();
        let on_error = props.on_error.clone();

        Callback::from(move |_| {
            if let Some(data) = (*passport_data).clone() {
                wallet_state.set(WalletState::GeneratingProof);

                let on_success = {
                    let wallet_state = wallet_state.clone();
                    let current_proof = current_proof.clone();
                    let on_proof_generated = on_proof_generated.clone();

                    Callback::from(move |proof: ZkPassportProof| {
                        current_proof.set(Some(proof.clone()));
                        wallet_state.set(WalletState::Ready);
                        on_proof_generated.emit(proof);
                    })
                };

                let on_error_cb = {
                    let wallet_state = wallet_state.clone();
                    let on_error = on_error.clone();

                    Callback::from(move |error: String| {
                        wallet_state.set(WalletState::Error(error.clone()));
                        on_error.emit(error);
                    })
                };

                service.generate_proof_async(
                    data,
                    "identity_verification".to_string(),
                    on_success,
                    on_error_cb,
                );
            }
        })
    };

    html! {
        <div class={classes!("zkpassport-wallet", props.class.clone())}>
            {match &*wallet_state {
                WalletState::Idle => html! {
                    <div class="card bg-base-200 p-4">
                        <div class="text-center">
                            <div class="text-4xl mb-2">{"🛂"}</div>
                            <p>{"ZK Passport integration ready"}</p>
                        </div>
                    </div>
                },
                WalletState::Initializing => html! {
                    <div class="card bg-base-200 p-4">
                        <div class="flex items-center justify-center space-x-2">
                            <span class="loading loading-spinner loading-sm"></span>
                            <span>{"Initializing ZK Passport SDK..."}</span>
                        </div>
                    </div>
                },
                WalletState::Ready => html! {
                    <div class="space-y-4">
                        {if passport_data.is_none() {
                            html! {
                                <div class="card bg-base-100 shadow-lg">
                                    <div class="card-body text-center">
                                        <div class="text-4xl mb-4">{"🛂"}</div>
                                        <h3 class="card-title justify-center">{"ZK Passport Ready"}</h3>
                                        <p class="text-base-content/70 mb-4">
                                            {"Scan your passport to generate privacy-preserving identity proofs"}
                                        </p>
                                        <button
                                            class="btn btn-primary btn-lg"
                                            onclick={scan_passport}
                                        >
                                            {"📱 Scan Passport"}
                                        </button>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {
                                <div class="card bg-base-100 shadow-lg">
                                    <div class="card-body">
                                        <div class="flex items-center justify-between mb-4">
                                            <div class="flex items-center space-x-2">
                                                <span class="text-2xl">{"✅"}</span>
                                                <div>
                                                    <h3 class="font-bold">{"Passport Verified"}</h3>
                                                    <div class="badge badge-success badge-sm">{"Identity Confirmed"}</div>
                                                </div>
                                            </div>
                                            <button
                                                class="btn btn-sm btn-outline"
                                                onclick={scan_passport}
                                            >
                                                {"Scan Again"}
                                            </button>
                                        </div>

                                        {if let Some(data) = &*passport_data {
                                            html! {
                                                <div class="space-y-3">
                                                    <div class="stats stats-vertical lg:stats-horizontal shadow w-full">
                                                        <div class="stat">
                                                            <div class="stat-figure text-secondary">{"🌍"}</div>
                                                            <div class="stat-title">{"Nationality"}</div>
                                                            <div class="stat-value text-primary text-lg">{&data.nationality}</div>
                                                        </div>
                                                        <div class="stat">
                                                            <div class="stat-figure text-secondary">{"🔞"}</div>
                                                            <div class="stat-title">{"Age Verification"}</div>
                                                            <div class="stat-value text-secondary text-lg">
                                                                {if data.age_over_18 { "18+" } else { "Under 18" }}
                                                            </div>
                                                        </div>
                                                    </div>

                                                    <button
                                                        class="btn btn-accent btn-block"
                                                        onclick={generate_identity_proof}
                                                        disabled={current_proof.is_some()}
                                                    >
                                                        {if current_proof.is_some() {
                                                            "✅ Identity Proof Generated"
                                                        } else {
                                                            "🔐 Generate Identity Proof"
                                                        }}
                                                    </button>

                                                    {if let Some(_proof) = &*current_proof {
                                                        html! {
                                                            <div class="alert alert-success">
                                                                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                                                </svg>
                                                                <div>
                                                                    <div class="font-semibold">{"ZK Identity Proof Generated"}</div>
                                                                    <div class="text-sm">{"Your identity has been verified without revealing personal information"}</div>
                                                                </div>
                                                            </div>
                                                        }
                                                    } else {
                                                        html! { <></> }
                                                    }}
                                                </div>
                                            }
                                        } else {
                                            html! { <></> }
                                        }}
                                    </div>
                                </div>
                            }
                        }}
                    </div>
                },
                WalletState::Scanning => html! {
                    <div class="card bg-base-200 p-4">
                        <div class="flex items-center justify-center space-x-2">
                            <span class="loading loading-spinner loading-sm"></span>
                            <span>{"Scanning passport... Please follow device instructions"}</span>
                        </div>
                    </div>
                },
                WalletState::GeneratingProof => html! {
                    <div class="card bg-base-200 p-4">
                        <div class="flex items-center justify-center space-x-2">
                            <span class="loading loading-spinner loading-sm"></span>
                            <span>{"Generating zero-knowledge proof..."}</span>
                        </div>
                    </div>
                },
                WalletState::Error(error) => html! {
                    <div class="alert alert-error">
                        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        <div>
                            <div class="font-semibold">{"ZK Passport Error"}</div>
                            <div class="text-sm">{error}</div>
                        </div>
                    </div>
                },
            }}
        </div>
    }
}
