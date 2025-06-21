use crate::components::{
    certificate_display::CertificateDisplay, certificate_form::CertificateForm, header::Header,
};
use crate::types::AppState;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(|| AppState::default());

    html! {
        <div class="min-h-screen bg-base-200">
            <Header />
            <div class="container mx-auto px-4 py-8">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    // Certificate Input Form
                    <div class="card bg-base-100 shadow-xl">
                        <div class="card-body">
                            <h2 class="card-title text-2xl mb-4">
                                {"🎓 Create Language Certificate"}
                            </h2>
                            <CertificateForm state={app_state.clone()} />
                        </div>
                    </div>

                    // Certificate Display & ZK Proof
                    <div class="card bg-base-100 shadow-xl">
                        <div class="card-body">
                            <h2 class="card-title text-2xl mb-4">
                                {"🔐 ZK Verification"}
                            </h2>
                            <CertificateDisplay state={app_state.clone()} />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
