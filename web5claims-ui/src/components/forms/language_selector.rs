use crate::components::ui::{SelectField, SelectOption};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LanguageSelectorProps {
    pub value: String,
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: String,
}

#[function_component(LanguageSelector)]
pub fn language_selector(props: &LanguageSelectorProps) -> Html {
    let options = vec![
        SelectOption {
            value: "German_A1_Basic".to_string(),
            label: "🇩🇪 German A1 - Basic".to_string(),
        },
        SelectOption {
            value: "German_A2_Elementary".to_string(),
            label: "🇩🇪 German A2 - Elementary".to_string(),
        },
        SelectOption {
            value: "German_B1_Intermediate".to_string(),
            label: "🇩🇪 German B1 - Intermediate".to_string(),
        },
        SelectOption {
            value: "German_B2_Complete".to_string(),
            label: "🇩🇪 German B2 - Upper Intermediate".to_string(),
        },
        SelectOption {
            value: "German_C1_Advanced".to_string(),
            label: "🇩🇪 German C1 - Advanced".to_string(),
        },
        SelectOption {
            value: "Spanish_A1_Basic".to_string(),
            label: "🇪🇸 Spanish A1 - Basic".to_string(),
        },
        SelectOption {
            value: "Spanish_B2_Complete".to_string(),
            label: "🇪🇸 Spanish B2 - Upper Intermediate".to_string(),
        },
        SelectOption {
            value: "French_A2_Elementary".to_string(),
            label: "🇫🇷 French A2 - Elementary".to_string(),
        },
        SelectOption {
            value: "Italian_B1_Intermediate".to_string(),
            label: "🇮🇹 Italian B1 - Intermediate".to_string(),
        },
    ];

    html! {
        <SelectField
            label="Language Course"
            icon="🌍"
            options={options}
            value={props.value.clone()}
            onchange={props.onchange.clone()}
            disabled={props.disabled}
            class={props.class.clone()}
            required={true}
        />
    }
}
