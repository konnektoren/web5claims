use crate::components::{
    certificate_display::CertificateDisplay,
    certificate_form::CertificateForm,
    layout::{PageLayout, TwoColumnLayout},
    ui::Card,
    wallet::{AleoWallet, WalletInfo},
};
use crate::types::AppState;
use yew::prelude::*;

#[function_component(IssuerPage)]
pub fn issuer_page() -> Html {
    let app_state = use_state(|| AppState::default());
    let wallet_info = use_state(|| None::<WalletInfo>);
    let wallet_error = use_state(|| None::<String>);

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

    let left_content = html! {
        <div class="space-y-6">
            // Certificate Creation Section (moved to top)
            <Card title="🎓 Create Language Certificate">
                <CertificateForm state={app_state.clone()} />
            </Card>

            // Wallet Connection Section (optional, collapsed by default)
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

                        if let Some(error) = &*wallet_error {
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
                        <div class="text-sm">{"Create certificates and generate zero-knowledge proofs locally"}</div>
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
                    {"Create language learning certificates and generate zero-knowledge proofs"}
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
