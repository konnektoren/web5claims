use crate::components::ui::InputField;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StudentInfoProps {
    pub name: String,
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: String,
}

#[function_component(StudentInfo)]
pub fn student_info(props: &StudentInfoProps) -> Html {
    html! {
        <InputField
            label="Student Name"
            icon="👤"
            placeholder="Enter student name"
            value={props.name.clone()}
            onchange={props.onchange.clone()}
            disabled={props.disabled}
            class={props.class.clone()}
            required={true}
        />
    }
}
