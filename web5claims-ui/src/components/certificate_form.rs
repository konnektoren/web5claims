use crate::types::AppState;
use chrono::Utc;
use konnektoren_core::certificates::CertificateData;
use web_sys::HtmlInputElement;
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

    let on_game_path_change = {
        let game_path_name = game_path_name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            game_path_name.set(input.value());
        })
    };

    let on_total_challenges_change = {
        let total_challenges = total_challenges.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<usize>() {
                total_challenges.set(value);
            }
        })
    };

    let on_solved_challenges_change = {
        let solved_challenges = solved_challenges.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<usize>() {
                solved_challenges.set(value);
            }
        })
    };

    let on_profile_name_change = {
        let profile_name = profile_name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            profile_name.set(input.value());
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

            new_state.certificate_data = Some(certificate_data);
            state.set(new_state);
            is_loading.set(false);
        })
    };

    let performance_percentage = if *total_challenges > 0 {
        ((*solved_challenges as f64 / *total_challenges as f64) * 100.0) as u8
    } else {
        0
    };

    html! {
        <form onsubmit={on_submit} class="space-y-4">
            // Language Course Selection
            <div class="form-control">
                <label class="label">
                    <span class="label-text font-semibold">{"🌍 Language Course"}</span>
                </label>
                <select
                    class="select select-bordered w-full"
                    onchange={on_game_path_change}
                    value={(*game_path_name).clone()}
                >
                    <option value="German_A1_Basic">{"🇩🇪 German A1 - Basic"}</option>
                    <option value="German_A2_Elementary">{"🇩🇪 German A2 - Elementary"}</option>
                    <option value="German_B1_Intermediate">{"🇩🇪 German B1 - Intermediate"}</option>
                    <option value="German_B2_Complete">{"🇩🇪 German B2 - Upper Intermediate"}</option>
                    <option value="German_C1_Advanced">{"🇩🇪 German C1 - Advanced"}</option>
                    <option value="Spanish_A1_Basic">{"🇪🇸 Spanish A1 - Basic"}</option>
                    <option value="Spanish_B2_Complete">{"🇪🇸 Spanish B2 - Upper Intermediate"}</option>
                    <option value="French_A2_Elementary">{"🇫🇷 French A2 - Elementary"}</option>
                    <option value="Italian_B1_Intermediate">{"🇮🇹 Italian B1 - Intermediate"}</option>
                </select>
            </div>

            // Student Name
            <div class="form-control">
                <label class="label">
                    <span class="label-text font-semibold">{"👤 Student Name"}</span>
                </label>
                <input
                    type="text"
                    class="input input-bordered w-full"
                    placeholder="Enter student name"
                    value={(*profile_name).clone()}
                    onchange={on_profile_name_change}
                />
            </div>

            // Challenge Statistics
            <div class="grid grid-cols-2 gap-4">
                <div class="form-control">
                    <label class="label">
                        <span class="label-text font-semibold">{"📚 Total Challenges"}</span>
                    </label>
                    <input
                        type="number"
                        class="input input-bordered w-full"
                        value={total_challenges.to_string()}
                        onchange={on_total_challenges_change}
                        min="1"
                        max="100"
                    />
                </div>

                <div class="form-control">
                    <label class="label">
                        <span class="label-text font-semibold">{"✅ Solved Challenges"}</span>
                    </label>
                    <input
                        type="number"
                        class="input input-bordered w-full"
                        value={solved_challenges.to_string()}
                        onchange={on_solved_challenges_change}
                        min="0"
                        max={total_challenges.to_string()}
                    />
                </div>
            </div>

            // Performance Preview
            <div class="card bg-base-200 p-4">
                <div class="flex justify-between items-center">
                    <span class="text-sm font-medium">{"Performance Preview:"}</span>
                    <div class="badge badge-primary badge-lg">
                        {format!("{}%", performance_percentage)}
                    </div>
                </div>
                <progress
                    class="progress progress-primary w-full mt-2"
                    value={performance_percentage.to_string()}
                    max="100"
                ></progress>
            </div>

            // Submit Button
            <button
                type="submit"
                class={classes!(
                    "btn", "btn-primary", "w-full", "mt-6",
                    if *is_loading { "loading" } else { "" }
                )}
                disabled={*is_loading}
            >
                if *is_loading {
                    {"Generating Certificate..."}
                } else {
                    {"🎓 Generate Certificate"}
                }
            </button>
        </form>
    }
}
