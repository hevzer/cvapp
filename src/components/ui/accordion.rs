use leptos::prelude::*;

#[component]
pub fn Accordion(
    #[prop(into)] title: Signal<String>,
    #[prop(optional, into)] icon: Option<String>,
    #[prop(optional)] default_open: bool,
    children: Children,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(default_open);

    view! {
        <div class="rounded-2xl overflow-hidden bg-white dark:bg-slate-900 border border-transparent shadow-[0_2px_8px_rgb(0,0,0,0.08)] dark:shadow-none hover:shadow-[0_4px_16px_rgb(0,0,0,0.12)] transition-shadow duration-300">
            <button
                type="button"
                on:click=move |_| set_is_open.update(|open| *open = !*open)
                class="w-full flex items-center justify-between px-6 py-4.5 text-left group hover:bg-slate-50 dark:hover:bg-slate-800/80 active:bg-slate-100 dark:active:bg-slate-800 transition-colors duration-200"
            >
                <span class="flex items-center gap-3 font-bold tracking-wide text-[15px] text-slate-800 dark:text-slate-100">
                    {icon.map(|ic| {
                        view! {
                            <span class=move || {
                                if is_open.get() {
                                    "text-xl transition-transform duration-300 scale-110 text-cyan-600 dark:text-cyan-400"
                                } else {
                                    "text-xl transition-transform duration-300 text-slate-500"
                                }
                            }>
                                {ic}
                            </span>
                        }
                    })}
                    {move || title.get()}
                </span>
                <svg
                    class=move || {
                        if is_open.get() {
                            "w-5 h-5 text-slate-400 transition-transform duration-300 ease-[cubic-bezier(0.4,0,0.2,1)] rotate-180 text-cyan-500"
                        } else {
                            "w-5 h-5 text-slate-400 transition-transform duration-300 ease-[cubic-bezier(0.4,0,0.2,1)]"
                        }
                    }
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                </svg>
            </button>
            <div
                class=move || {
                    if is_open.get() {
                        "transition-all duration-300 block"
                    } else {
                        "transition-all duration-300 hidden"
                    }
                }
            >
                <div class="px-6 pb-6 pt-2 border-t border-slate-100 dark:border-slate-800">
                    {children()}
                </div>
            </div>
        </div>
    }
}
