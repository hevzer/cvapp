use crate::components::ui::tag_input::TagInput;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn HiddenKeywordsForm() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;

    view! {
        <div class="space-y-3">
            <div class="flex items-start gap-2 p-3 bg-rose-50 dark:bg-rose-950/30 border border-rose-200 dark:border-rose-800 rounded-lg">
                <span class="text-rose-600 text-lg mt-0.5">"⚠️"</span>
                <div class="text-xs text-rose-700 dark:text-rose-300 leading-relaxed space-y-1.5">
                    <p class="font-bold">"No longer rendered into your CV."</p>
                    <p>
                        "Modern ATS (Greenhouse Harmonic, LLM-based screeners) detect zero-size hidden text and flag the resume as deceptive — the technique now hurts more than it helps. Templates produced by CVapp ship clean, well-structured HTML that scores higher on parser tests without it."
                    </p>
                    <p class="text-rose-600 dark:text-rose-400">
                        "Your existing keywords below are preserved in case you want to copy them into a real section, but they will not appear in the printed CV."
                    </p>
                </div>
            </div>
            <TagInput
                tags=Signal::derive(move || resume.get().hidden_keywords)
                on_change=Callback::new(move |kw| {
                    resume.update(|r| r.hidden_keywords = kw);
                    state.save_to_storage();
                })
                placeholder="e.g., Kubernetes, Terraform, SDLC..."
            />
        </div>
    }
}
