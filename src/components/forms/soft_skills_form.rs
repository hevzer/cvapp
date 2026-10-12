use crate::components::ui::tag_input::TagInput;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn SoftSkillsForm() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;

    view! {
        <div>
            <TagInput
                tags=Signal::derive(move || resume.get().soft_skills)
                on_change=Callback::new(move |skills| {
                    resume.update(|r| r.soft_skills = skills);
                    state.save_to_storage();
                })
                label="Soft Skills (e.g. Leadership, Mentorship)"
                placeholder="Type a skill and press Enter..."
            />
        </div>
    }
}
