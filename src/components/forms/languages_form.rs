use crate::components::ui::tag_input::TagInput;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn LanguagesForm() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;

    view! {
        <div>
            <TagInput
                tags=Signal::derive(move || resume.get().languages)
                on_change=Callback::new(move |langs| {
                    resume.update(|r| r.languages = langs);
                    state.save_to_storage();
                })
                label="Languages (e.g. English - Fluent, French - Native)"
                placeholder="Type a language and press Enter..."
            />
        </div>
    }
}
