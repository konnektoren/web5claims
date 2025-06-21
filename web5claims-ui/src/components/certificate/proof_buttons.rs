use crate::components::ui::Button;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProofButtonsProps {
    pub on_generate_language_proof: Callback<MouseEvent>,
    pub on_generate_performance_proof: Callback<MouseEvent>,
    pub on_generate_combined_proof: Callback<MouseEvent>,
    #[prop_or_default]
    pub is_generating: bool,
    #[prop_or_default]
    pub has_certificate: bool,
    #[prop_or_default]
    pub class: String,
}

#[function_component(ProofButtons)]
pub fn proof_buttons(props: &ProofButtonsProps) -> Html {
    if !props.has_certificate {
        return html! {
            <div class={classes!("alert", "alert-info", props.class.clone())}>
                <div class="flex">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                    <span>{"Please generate a certificate first to create ZK proofs."}</span>
                </div>
            </div>
        };
    }

    html! {
        <div class={classes!("space-y-4", props.class.clone())}>
            <div class="text-sm text-base-content/70 mb-4">
                {"Choose the type of zero-knowledge proof you want to generate:"}
            </div>

            <div class="grid gap-3">
                <Button
                    variant="outline"
                    size="md"
                    full_width={true}
                    loading={props.is_generating}
                    disabled={props.is_generating}
                    onclick={props.on_generate_language_proof.clone()}
                >
                    <div class="flex items-center gap-2">
                        <span>{"🌍"}</span>
                        <div class="text-left">
                            <div class="font-medium">{"Language Proficiency"}</div>
                            <div class="text-xs opacity-70">{"Prove language level without revealing score"}</div>
                        </div>
                    </div>
                </Button>

                <Button
                    variant="outline"
                    size="md"
                    full_width={true}
                    loading={props.is_generating}
                    disabled={props.is_generating}
                    onclick={props.on_generate_performance_proof.clone()}
                >
                    <div class="flex items-center gap-2">
                        <span>{"📊"}</span>
                        <div class="text-left">
                            <div class="font-medium">{"Performance Threshold"}</div>
                            <div class="text-xs opacity-70">{"Prove score above 90% without revealing exact score"}</div>
                        </div>
                    </div>
                </Button>

                <Button
                    variant="primary"
                    size="md"
                    full_width={true}
                    loading={props.is_generating}
                    disabled={props.is_generating}
                    onclick={props.on_generate_combined_proof.clone()}
                >
                    <div class="flex items-center gap-2">
                        <span>{"🔐"}</span>
                        <div class="text-left">
                            <div class="font-medium">{"Combined Proof"}</div>
                            <div class="text-xs opacity-70">{"Prove both language level and performance"}</div>
                        </div>
                    </div>
                </Button>
            </div>

            if props.is_generating {
                <div class="alert alert-info">
                    <div class="flex">
                        <span class="loading loading-spinner loading-sm"></span>
                        <span>{"Generating zero-knowledge proof... This may take a moment."}</span>
                    </div>
                </div>
            }
        </div>
    }
}
