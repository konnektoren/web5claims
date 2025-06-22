use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StudentInfoProps {
    pub name: String,
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub verified_name: Option<String>,
}

#[function_component(StudentInfo)]
pub fn student_info(props: &StudentInfoProps) -> Html {
    let on_input = {
        let onchange = props.onchange.clone();
        let verified_name = props.verified_name.clone();

        Callback::from(move |e: InputEvent| {
            // Only allow changes if name is not verified
            if verified_name.is_none() {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                onchange.emit(input.value());
            }
        })
    };

    let is_verified = props.verified_name.is_some();
    let is_readonly = props.disabled || is_verified;

    html! {
        <div class="form-control">
            <label class="label">
                <span class="label-text font-medium">{"Student Name"}</span>
                {if is_verified {
                    html! {
                        <span class="label-text-alt text-success flex items-center gap-1">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            {"Verified"}
                        </span>
                    }
                } else {
                    html! { <></> }
                }}
            </label>

            <input
                type="text"
                class={classes!(
                    "input",
                    "input-bordered",
                    if is_verified { "input-success" } else { "" },
                    if props.disabled { "input-disabled" } else { "" }
                )}
                placeholder={if is_verified { "Verified name from ZKPassport" } else { "Enter student name" }}
                value={props.name.clone()}
                readonly={is_readonly}
                disabled={props.disabled}
                oninput={on_input}
            />

            {if is_verified {
                html! {
                    <label class="label">
                        <span class="label-text-alt text-success text-xs flex items-center gap-1">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                            </svg>
                            {"Name verified with ZKPassport - cannot be modified"}
                        </span>
                    </label>
                }
            } else if !props.disabled {
                html! {
                    <label class="label">
                        <span class="label-text-alt text-base-content/60 text-xs">
                            {"Enter the name as it should appear on the certificate"}
                        </span>
                    </label>
                }
            } else {
                html! { <></> }
            }}
        </div>
    }
}
