use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ErrorDisplayProps {
    pub error: Option<String>,
    pub on_dismiss: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: String,
    #[prop_or("error".to_string())]
    pub variant: String,
    #[prop_or_default]
    pub dismissible: bool,
}

#[function_component(ErrorDisplay)]
pub fn error_display(props: &ErrorDisplayProps) -> Html {
    if props.error.is_none() {
        return html! {};
    }

    let error_message = props.error.as_ref().unwrap();

    html! {
        <div class={classes!(
            "alert",
            format!("alert-{}", props.variant),
            props.class.clone()
        )}>
            <div class="flex items-start w-full">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="stroke-current shrink-0 h-6 w-6"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                </svg>

                <div class="flex-1 ml-3">
                    <div class="font-semibold">{"Error"}</div>
                    <div class="text-sm mt-1">{error_message}</div>
                </div>

                if props.dismissible {
                    <button
                        class="btn btn-sm btn-ghost ml-2"
                        onclick={props.on_dismiss.clone()}
                        aria-label="Dismiss error"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-4 w-4"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M6 18L18 6M6 6l12 12"
                            />
                        </svg>
                    </button>
                }
            </div>
        </div>
    }
}
