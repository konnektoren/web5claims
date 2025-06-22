use crate::components::forms::{ChallengeStats, LanguageSelector, PerformancePreview, StudentInfo};
use crate::components::ui::Button;
use crate::types::AppState;
use chrono::Utc;
use konnektoren_core::certificates::CertificateData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CertificateFormProps {
    pub state: UseStateHandle<AppState>,
    #[prop_or_default]
    pub verified_name: Option<String>,
}

#[function_component(CertificateForm)]
pub fn certificate_form(props: &CertificateFormProps) -> Html {
    let game_path_name = use_state(|| "German_B2_Complete".to_string());
    let total_challenges = use_state(|| 50);
    let solved_challenges = use_state(|| 47);

    // Initialize profile_name with verified_name if available
    let profile_name = use_state(|| {
        props
            .verified_name
            .clone()
            .unwrap_or_else(|| "Language Learner".to_string())
    });

    let is_loading = use_state(|| false);

    // Update profile_name when verified_name changes
    use_effect_with(props.verified_name.clone(), {
        let profile_name = profile_name.clone();
        move |verified_name| {
            if let Some(name) = verified_name.as_ref() {
                if !name.is_empty() {
                    profile_name.set(name.clone());
                    log::info!(
                        "Auto-filled student name from ZKPass verification: {}",
                        name
                    );
                }
            }
            || ()
        }
    });

    let on_language_change = {
        let game_path_name = game_path_name.clone();
        Callback::from(move |value: String| {
            game_path_name.set(value);
        })
    };

    let on_student_name_change = {
        let profile_name = profile_name.clone();
        let verified_name = props.verified_name.clone();

        Callback::from(move |value: String| {
            // Only allow changes if name is not verified
            if verified_name.is_none() {
                profile_name.set(value);
            } else {
                log::warn!("Attempted to change verified name - ignoring change");
            }
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
        let verified_name = props.verified_name.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let mut new_state = (*state).clone();
            is_loading.set(true);

            // Log certificate creation with verification status
            if verified_name.is_some() {
                log::info!(
                    "Creating certificate with verified identity: {}",
                    *profile_name
                );
            } else {
                log::info!(
                    "Creating certificate without identity verification: {}",
                    *profile_name
                );
            }

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
                verified_name={props.verified_name.clone()}
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
                } else if props.verified_name.is_some() {
                    {"🎓 Generate Verified Certificate"}
                } else {
                    {"🎓 Generate Certificate"}
                }
            </Button>

            // Show verification status
            {if let Some(verified_name) = &props.verified_name {
                html! {
                    <div class="alert alert-success mt-4">
                        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        <div>
                            <div class="font-semibold">{"Identity Verified"}</div>
                            <div class="text-sm">
                                {format!("Certificate will be issued for verified identity: {}", verified_name)}
                            </div>
                        </div>
                    </div>
                }
            } else {
                html! {
                    <div class="alert alert-info mt-4">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                        <div>
                            <div class="font-semibold">{"Enhanced Verification Available"}</div>
                            <div class="text-sm">
                                {"Consider using ZKPassport verification to enhance certificate credibility."}
                            </div>
                        </div>
                    </div>
                }
            }}
        </form>
    }
}
