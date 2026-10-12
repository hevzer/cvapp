use leptos::prelude::*;

#[component]
pub fn TagInput(
    #[prop(into)] tags: Signal<Vec<String>>,
    #[prop(into)] on_change: Callback<Vec<String>>,
    #[prop(optional, into)] placeholder: Option<&'static str>,
    #[prop(optional, into)] label: Option<&'static str>,
) -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let (is_focused, set_is_focused) = signal(false);
    let ph = placeholder.unwrap_or("Type and press Enter...");

    let add_tag = move |val: String| {
        let trimmed = val.trim().to_string();
        if !trimmed.is_empty() {
            let mut current = tags.get();
            if !current.contains(&trimmed) {
                current.push(trimmed);
                on_change.run(current);
            }
        }
        set_input.set(String::new());
    };

    let remove_tag = move |index: usize| {
        let mut current = tags.get();
        if index < current.len() {
            current.remove(index);
            on_change.run(current);
        }
    };

    view! {
        <div class="relative pt-2">
            <div
                class=move || {
                    if is_focused.get() {
                        "flex flex-wrap gap-2 px-3 pb-2.5 pt-4 border-2 rounded-lg bg-transparent min-h-[58px] transition-colors border-cyan-600 dark:border-cyan-500"
                    } else {
                        "flex flex-wrap gap-2 px-3 pb-2.5 pt-4 border-2 rounded-lg bg-transparent min-h-[58px] transition-colors border-slate-300 dark:border-slate-600"
                    }
                }
            >
                {move || {
                    tags.get().into_iter().enumerate().map(|(idx, tag)| {
                        view! {
                            <span class="flex items-center gap-1 px-2.5 py-1 bg-cyan-100 dark:bg-cyan-900/50 text-cyan-700 dark:text-cyan-300 text-[13px] font-bold tracking-wide rounded-md">
                                {tag}
                                <button
                                    type="button"
                                    on:click=move |_| remove_tag(idx)
                                    class="ml-0.5 text-cyan-400 hover:text-cyan-600 dark:hover:text-cyan-200 transition-colors"
                                >
                                    "×"
                                </button>
                            </span>
                        }
                    }).collect::<Vec<_>>()
                }}
                <input
                    type="text"
                    prop:value=move || input.get()
                    on:input=move |ev| set_input.set(event_target_value(&ev))
                    on:keydown=move |ev| {
                        let key = ev.key();
                        if key == "Enter" {
                            ev.prevent_default();
                            add_tag(input.get());
                        } else if key == "Backspace" && input.get().is_empty() && !tags.get().is_empty() {
                            remove_tag(tags.get().len() - 1);
                        }
                    }
                    on:focus=move |_| set_is_focused.set(true)
                    on:blur=move |_| set_is_focused.set(false)
                    placeholder=move || {
                        if tags.get().is_empty() && is_focused.get() {
                            ph
                        } else {
                            ""
                        }
                    }
                    class="flex-1 min-w-[120px] outline-none bg-transparent text-sm text-slate-900 dark:text-white placeholder-slate-400"
                />
            </div>
            {label.map(|lbl| {
                view! {
                    <label
                        class=move || {
                            let active = is_focused.get() || !tags.get().is_empty() || !input.get().is_empty();
                            if active {
                                "absolute text-sm font-medium duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] bg-white dark:bg-slate-900 px-2 start-1 pointer-events-none transition-all text-cyan-600 dark:text-cyan-400"
                            } else {
                                "absolute text-sm font-medium duration-300 transform -translate-y-1/2 scale-100 top-1/2 z-10 origin-[0] bg-white dark:bg-slate-900 px-2 start-1 pointer-events-none transition-all text-slate-500 dark:text-slate-400"
                            }
                        }
                    >
                        {lbl}
                    </label>
                }
            })}
        </div>
    }
}
