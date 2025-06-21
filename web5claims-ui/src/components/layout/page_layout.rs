use crate::components::header::Header;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageLayoutProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(PageLayout)]
pub fn page_layout(props: &PageLayoutProps) -> Html {
    html! {
        <div class={classes!("min-h-screen", "bg-base-200", props.class.clone())}>
            <Header />
            <div class="container mx-auto px-4 py-8">
                {for props.children.iter()}
            </div>
        </div>
    }
}
