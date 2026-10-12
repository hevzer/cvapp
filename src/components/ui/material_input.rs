use leptos::prelude::*;

#[component]
pub fn MaterialInput(
    #[prop(into)] label: String,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(optional, into)] input_type: Option<&'static str>,
    #[prop(optional, into)] placeholder: Option<&'static str>,
    #[prop(optional, into)] class: Option<&'static str>,
) -> impl IntoView {
    let t = input_type.unwrap_or("text");
    let ph = placeholder.unwrap_or(" ");
    let extra_class = class.unwrap_or("");

    view! {
        <div class=format!("relative {}", extra_class)>
            <input
                type=t
                placeholder=ph
                prop:value=move || value.get()
                on:input=move |ev| {
                    on_change.run(event_target_value(&ev));
                }
                class="block px-3 pb-2.5 pt-4 w-full text-sm text-gray-900 bg-transparent rounded-lg border-2 border-slate-300 appearance-none dark:text-white dark:border-slate-600 dark:focus:border-cyan-500 focus:outline-none focus:ring-0 focus:border-cyan-600 peer transition-colors"
            />
            <label
                class="absolute text-sm font-medium text-slate-500 dark:text-slate-400 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] bg-white dark:bg-slate-900 px-2 peer-focus:px-2 peer-focus:text-cyan-600 peer-focus:dark:text-cyan-400 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 start-1 pointer-events-none"
            >
                {label}
            </label>
        </div>
    }
}
