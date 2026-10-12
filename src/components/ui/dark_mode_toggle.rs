use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn DarkModeToggle() -> impl IntoView {
    let state = expect_context::<AppState>();
    let dark_mode = state.dark_mode;

    view! {
        <button
            type="button"
            on:click=move |_| {
                dark_mode.update(|dm| *dm = !*dm);
                state.save_to_storage();
            }
            class=move || {
                if dark_mode.get() {
                    "relative flex items-center justify-center w-10 h-10 rounded-full transition-all duration-300 active:scale-90 outline-none bg-slate-800 text-amber-300 hover:bg-slate-700 shadow-inner ring-1 ring-white/10"
                } else {
                    "relative flex items-center justify-center w-10 h-10 rounded-full transition-all duration-300 active:scale-90 outline-none bg-white text-cyan-600 hover:bg-slate-50 shadow-sm border border-slate-200"
                }
            }
            aria-label="Toggle dark mode"
        >
            <i class=move || {
                if dark_mode.get() {
                    "bi block transition-transform duration-500 bi-moon-stars-fill rotate-[360deg] scale-100"
                } else {
                    "bi block transition-transform duration-500 bi-sun-fill rotate-0 scale-110"
                }
            }></i>
        </button>
    }
}
