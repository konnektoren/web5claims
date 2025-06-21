use crate::components::ui::ProgressBar;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PerformancePreviewProps {
    pub total_challenges: usize,
    pub solved_challenges: usize,
    #[prop_or_default]
    pub class: String,
}

#[function_component(PerformancePreview)]
pub fn performance_preview(props: &PerformancePreviewProps) -> Html {
    let percentage = if props.total_challenges > 0 {
        ((props.solved_challenges as f64 / props.total_challenges as f64) * 100.0) as u8
    } else {
        0
    };

    html! {
        <div class={classes!("card", "bg-base-200", "p-4", props.class.clone())}>
            <ProgressBar
                value={percentage}
                max={100}
                variant="primary"
                show_percentage={true}
                label="Performance Preview:"
                size="lg"
            />
        </div>
    }
}
