use crate::components::ui::material_textarea::MaterialTextarea;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn SummaryForm() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;

    view! {
        <div>
            <MaterialTextarea
                label="Professional Summary"
                value=Signal::derive(move || resume.get().summary)
                on_change=Callback::new(move |val| {
                    resume.update(|r| r.summary = val);
                    state.save_to_storage();
                })
                rows=5
            />
        </div>
    }
}
