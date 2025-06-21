use crate::components::forms::{ChallengeStats, LanguageSelector, PerformancePreview, StudentInfo};
use crate::components::ui::Button;
use crate::types::AppState;
use chrono::Utc;
use konnektoren_core::certificates::CertificateData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CertificateFormProps {
    pub state: UseStateHandle<AppState>,
}

#[function_component(CertificateForm)]
pub fn certificate_form(props: &CertificateFormProps) -> Html {
    let game_path_name = use_state(|| "German_B2_Complete".to_string());
    let total_challenges = use_state(|| 50);
    let solved_challenges = use_state(|| 47);
    let profile_name = use_state(|| "Language Learner".to_string());
    let is_loading = use_state(|| false);

    let on_language_change = {
        let game_path_name = game_path_name.clone();
        Callback::from(move |value: String| {
            game_path_name.set(value);
        })
    };

    let on_student_name_change = {
        let profile_name = profile_name.clone();
        Callback::from(move |value: String| {
            profile_name.set(value);
        })
    };

    let on_total_challenges_change = {
        let total_challenges = total_challenges.clone();
        Callback::from(move |value: usize| {
            total_challenges.set(value);
        })
    };

    let on_solved_challenges_change = {
        let solved_challenges = solved_challenges.clone();
        Callback::from(move |value: usize| {
            solved_challenges.set(value);
        })
    };

    let on_submit = {
        let state = props.state.clone();
        let game_path_name = game_path_name.clone();
        let total_challenges = total_challenges.clone();
        let solved_challenges = solved_challenges.clone();
        let profile_name = profile_name.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let mut new_state = (*state).clone();
            is_loading.set(true);

            // Create certificate data
            let certificate_data = CertificateData::new(
                (*game_path_name).clone(),
                *total_challenges,
                *solved_challenges,
                (*profile_name).clone(),
                Utc::now(),
            );

            new_state.set_certificate(certificate_data);
            state.set(new_state);
            is_loading.set(false);
        })
    };

    let on_button_click = {
        let on_submit = on_submit.clone();
        Callback::from(move |_: MouseEvent| {
            // Create a synthetic submit event
            let event = SubmitEvent::new("submit").unwrap();
            on_submit.emit(event);
        })
    };

    html! {
        <form onsubmit={on_submit} class="space-y-6">
            <LanguageSelector
                value={(*game_path_name).clone()}
                onchange={on_language_change}
                disabled={*is_loading}
            />

            <StudentInfo
                name={(*profile_name).clone()}
                onchange={on_student_name_change}
                disabled={*is_loading}
            />

            <ChallengeStats
                total_challenges={*total_challenges}
                solved_challenges={*solved_challenges}
                on_total_change={on_total_challenges_change}
                on_solved_change={on_solved_challenges_change}
                disabled={*is_loading}
            />

            <PerformancePreview
                total_challenges={*total_challenges}
                solved_challenges={*solved_challenges}
            />

            <Button
                button_type="submit"
                variant="primary"
                size="lg"
                full_width={true}
                loading={*is_loading}
                disabled={*is_loading}
                onclick={on_button_click}
                class="mt-6"
            >
                if *is_loading {
                    {"Generating Certificate..."}
                } else {
                    {"🎓 Generate Certificate"}
                }
            </Button>
        </form>
    }
}
