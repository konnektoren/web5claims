use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub title: Option<String>,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class={classes!("card", "bg-base-100", "shadow-xl", props.class.clone())}>
            <div class="card-body">
                if let Some(title) = &props.title {
                    <h2 class="card-title text-2xl mb-4">
                        {title}
                    </h2>
                }
                {for props.children.iter()}
            </div>
        </div>
    }
}
