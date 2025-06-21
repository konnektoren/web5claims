use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProgressBarProps {
    pub value: u8,
    #[prop_or(100)]
    pub max: u8,
    #[prop_or("primary".to_string())]
    pub variant: String,
    #[prop_or_default]
    pub show_percentage: bool,
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub class: String,
    #[prop_or("md".to_string())]
    pub size: String,
}

#[function_component(ProgressBar)]
pub fn progress_bar(props: &ProgressBarProps) -> Html {
    let percentage = if props.max > 0 {
        ((props.value as f64 / props.max as f64) * 100.0) as u8
    } else {
        0
    };

    let progress_classes = classes!(
        "progress",
        format!("progress-{}", props.variant),
        format!("progress-{}", props.size),
        "w-full",
        props.class.clone()
    );

    html! {
        <div class="space-y-2">
            if let Some(label) = &props.label {
                <div class="flex justify-between items-center">
                    <span class="text-sm font-medium">{label}</span>
                    if props.show_percentage {
                        <div class="badge badge-primary badge-lg">
                            {format!("{}%", percentage)}
                        </div>
                    }
                </div>
            } else if props.show_percentage {
                <div class="flex justify-end">
                    <div class="badge badge-primary badge-lg">
                        {format!("{}%", percentage)}
                    </div>
                </div>
            }
            <progress
                class={progress_classes}
                value={props.value.to_string()}
                max={props.max.to_string()}
            ></progress>
        </div>
    }
}
