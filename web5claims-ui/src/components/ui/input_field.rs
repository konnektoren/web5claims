use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputFieldProps {
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub onchange: Callback<String>,
    #[prop_or("text".to_string())]
    pub input_type: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub min: Option<String>,
    #[prop_or_default]
    pub max: Option<String>,
    #[prop_or_default]
    pub icon: Option<String>,
}

#[function_component(InputField)]
pub fn input_field(props: &InputFieldProps) -> Html {
    let on_input_change = {
        let onchange = props.onchange.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            onchange.emit(input.value());
        })
    };

    html! {
        <div class={classes!("form-control", props.class.clone())}>
            if let Some(label) = &props.label {
                <label class="label">
                    <span class="label-text font-semibold">
                        if let Some(icon) = &props.icon {
                            {format!("{} {}", icon, label)}
                        } else {
                            {label}
                        }
                    </span>
                </label>
            }
            <input
                type={props.input_type.clone()}
                class="input input-bordered w-full"
                placeholder={props.placeholder.clone()}
                value={props.value.clone()}
                onchange={on_input_change}
                disabled={props.disabled}
                required={props.required}
                min={props.min.clone()}
                max={props.max.clone()}
            />
        </div>
    }
}
