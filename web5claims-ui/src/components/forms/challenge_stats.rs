use crate::components::ui::InputField;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeStatsProps {
    pub total_challenges: usize,
    pub solved_challenges: usize,
    pub on_total_change: Callback<usize>,
    pub on_solved_change: Callback<usize>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: String,
}

#[function_component(ChallengeStats)]
pub fn challenge_stats(props: &ChallengeStatsProps) -> Html {
    let on_total_change = {
        let callback = props.on_total_change.clone();
        Callback::from(move |value: String| {
            if let Ok(num) = value.parse::<usize>() {
                callback.emit(num);
            }
        })
    };

    let on_solved_change = {
        let callback = props.on_solved_change.clone();
        Callback::from(move |value: String| {
            if let Ok(num) = value.parse::<usize>() {
                callback.emit(num);
            }
        })
    };

    html! {
        <div class={classes!("grid", "grid-cols-2", "gap-4", props.class.clone())}>
            <InputField
                label="Total Challenges"
                icon="📚"
                input_type="number"
                value={props.total_challenges.to_string()}
                onchange={on_total_change}
                disabled={props.disabled}
                min="1"
                max="100"
                required={true}
            />
            <InputField
                label="Solved Challenges"
                icon="✅"
                input_type="number"
                value={props.solved_challenges.to_string()}
                onchange={on_solved_change}
                disabled={props.disabled}
                min="0"
                max={props.total_challenges.to_string()}
                required={true}
            />
        </div>
    }
}
