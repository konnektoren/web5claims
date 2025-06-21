use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[derive(Properties, PartialEq)]
pub struct SelectFieldProps {
    #[prop_or_default]
    pub label: Option<String>,
    pub options: Vec<SelectOption>,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub icon: Option<String>,
}

#[function_component(SelectField)]
pub fn select_field(props: &SelectFieldProps) -> Html {
    let on_select_change = {
        let onchange = props.onchange.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            onchange.emit(select.value());
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
            <select
                class="select select-bordered w-full"
                onchange={on_select_change}
                value={props.value.clone()}
                disabled={props.disabled}
                required={props.required}
            >
                {for props.options.iter().map(|option| {
                    html! {
                        <option value={option.value.clone()}>
                            {&option.label}
                        </option>
                    }
                })}
            </select>
        </div>
    }
}
