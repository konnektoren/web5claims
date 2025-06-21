use crate::components::{
    certificate_display::CertificateDisplay,
    certificate_form::CertificateForm,
    layout::{PageLayout, TwoColumnLayout},
    ui::Card,
};
use crate::types::AppState;
use yew::prelude::*;

#[function_component(IssuerPage)]
pub fn issuer_page() -> Html {
    let app_state = use_state(|| AppState::default());

    let left_content = html! {
        <Card title="🎓 Create Language Certificate">
            <CertificateForm state={app_state.clone()} />
        </Card>
    };

    let right_content = html! {
        <Card title="🔐 ZK Proof Generation">
            <CertificateDisplay state={app_state.clone()} />
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
