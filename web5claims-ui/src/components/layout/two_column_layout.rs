use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TwoColumnLayoutProps {
    pub left_content: Html,
    pub right_content: Html,
    #[prop_or_default]
    pub class: String,
    #[prop_or("lg".to_string())]
    pub breakpoint: String,
    #[prop_or(8)]
    pub gap: u8,
}

#[function_component(TwoColumnLayout)]
pub fn two_column_layout(props: &TwoColumnLayoutProps) -> Html {
    let grid_classes = classes!(
        "grid",
        "grid-cols-1",
        format!("{}:grid-cols-2", props.breakpoint),
        format!("gap-{}", props.gap),
        props.class.clone()
    );

    html! {
        <div class={grid_classes}>
            <div class="w-full">
                {props.left_content.clone()}
            </div>
            <div class="w-full">
                {props.right_content.clone()}
            </div>
        </div>
    }
}
