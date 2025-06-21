use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or("primary".to_string())]
    pub variant: String,
    #[prop_or("md".to_string())]
    pub size: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or("button".to_string())]
    pub button_type: String,
    #[prop_or_default]
    pub full_width: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let button_classes = classes!(
        "btn",
        format!("btn-{}", props.variant),
        format!("btn-{}", props.size),
        if props.full_width { "w-full" } else { "" },
        if props.loading { "loading" } else { "" },
        props.class.clone()
    );

    html! {
        <button
            type={props.button_type.clone()}
            class={button_classes}
            onclick={props.onclick.clone()}
            disabled={props.disabled || props.loading}
        >
            {for props.children.iter()}
        </button>
    }
}
