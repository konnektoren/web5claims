use crate::components::{
    certificate_display::CertificateDisplay,
    certificate_form::CertificateForm,
    layout::{PageLayout, TwoColumnLayout},
    ui::Card,
};
use crate::types::AppState;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(|| AppState::default());

    let left_content = html! {
        <Card title="🎓 Create Language Certificate">
            <CertificateForm state={app_state.clone()} />
        </Card>
    };

    let right_content = html! {
        <Card title="🔐 ZK Verification">
            <CertificateDisplay state={app_state.clone()} />
        </Card>
    };

    html! {
        <PageLayout>
            <TwoColumnLayout
                left_content={left_content}
                right_content={right_content}
                breakpoint="lg"
                gap={8}
            />
        </PageLayout>
    }
}
