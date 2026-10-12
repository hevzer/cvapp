use crate::components::about_modal::AboutModal;
use crate::components::data_actions::DataActions;
use crate::components::export_button::ExportButton;
use crate::components::forms::certifications_form::CertificationsForm;
use crate::components::forms::education_form::EducationForm;
use crate::components::forms::experience_form::ExperienceForm;
use crate::components::forms::hidden_keywords_form::HiddenKeywordsForm;
use crate::components::forms::interests_form::InterestsForm;
use crate::components::forms::languages_form::LanguagesForm;
use crate::components::forms::personal_info_form::PersonalInfoForm;
use crate::components::forms::soft_skills_form::SoftSkillsForm;
use crate::components::forms::summary_form::SummaryForm;
use crate::components::forms::technical_skills_form::TechnicalSkillsForm;
use crate::components::forms::volunteering_form::VolunteeringForm;
use crate::components::linkedin_modal::LinkedInModal;
use crate::components::ui::accordion::Accordion;
use crate::components::ui::dark_mode_toggle::DarkModeToggle;
use crate::components::ui::sliders::{FontScaleSlider, SpacingSlider, TextScaleSlider};
use crate::components::ui::style_customizer::StyleCustomizer;
use crate::components::ui::template_toggle::TemplateToggle;
use crate::i18n::{get_dictionary, LANGUAGE_OPTIONS};
use crate::models::TemplateType;
use crate::state::AppState;
use crate::templates::ats_minimalist::ATSMinimalist;
use crate::templates::modern_creative::ModernCreative;
use crate::templates::timeline_creative::TimelineCreative;
use crate::templates::timeline_two_column::TimelineTwoColumn;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);

    // Sync dark mode class on <html>
    Effect::new(move |_| {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(root) = doc.document_element() {
                let class_list = root.class_list();
                if state.dark_mode.get() {
                    let _ = class_list.add_1("dark");
                } else {
                    let _ = class_list.remove_1("dark");
                }
            }
        }
    });

    // Listen to real-time operating system theme changes (dark/light)
    if let Some(window) = web_sys::window() {
        if let Ok(Some(mql)) = window.match_media("(prefers-color-scheme: dark)") {
            use wasm_bindgen::JsCast;
            let dm = state.dark_mode;
            let on_change = wasm_bindgen::closure::Closure::wrap(Box::new(move |_ev: web_sys::Event| {
                if let Some(w) = web_sys::window() {
                    if let Ok(Some(mq)) = w.match_media("(prefers-color-scheme: dark)") {
                        dm.set(mq.matches());
                    }
                }
            }) as Box<dyn FnMut(_)>);
            let _ = mql.add_event_listener_with_callback("change", on_change.as_ref().unchecked_ref());
            on_change.forget();
        }
    }

    // Auto-persist state changes
    Effect::new(move |_| {
        state.save_to_storage();
    });

    let dict = move || {
        let lang = state.resume_data.with(|r| r.cv_language.clone());
        get_dictionary(&lang)
    };

    view! {
        <div class="flex flex-col h-screen bg-slate-100 dark:bg-black print:bg-none print:h-auto print:block">
            {/* Header */}
            <header class="flex-shrink-0 bg-white dark:bg-[#0a0a0a] shadow-md z-20 px-6 py-3 animate-slide-up print:hidden relative border-b border-transparent dark:border-white/[0.05]">
                <div class="flex items-center justify-between max-w-[1920px] mx-auto">
                    {/* Brand */}
                    <div class="flex items-center gap-3 group cursor-default select-none">
                        <div class="w-11 h-11 flex items-center justify-center transition-shadow duration-300">
                            <img src="/cvapp_macos_logo.png" alt="CVapp Logo" class="w-full h-full object-contain drop-shadow-md" />
                        </div>
                        <h1 class="text-lg font-extrabold tracking-tight text-gray-900 dark:text-white">
                            "CVapp"
                        </h1>
                    </div>

                    {/* Actions */}
                    <div class="flex items-center gap-1.5">
                        <AboutModal />
                        <button
                            type="button"
                            on:click=move |_| state.load_example()
                            class="px-4 py-2 text-[11px] font-bold uppercase tracking-widest rounded-full text-cyan-700 bg-cyan-100 dark:text-cyan-300 dark:bg-cyan-900/50 hover:bg-cyan-200 dark:hover:bg-cyan-800/60 hover:shadow-sm active:scale-95 transition-all duration-200 cursor-pointer"
                        >
                            "✨ Fill Example"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| state.clear_all()
                            class="px-4 py-2 text-[11px] font-bold uppercase tracking-widest rounded-full text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-slate-700 dark:hover:text-slate-300 active:scale-95 transition-all duration-200 cursor-pointer"
                        >
                            "Clear All"
                        </button>
                        <div class="w-px h-5 bg-gray-200 dark:bg-white/10 mx-1.5" />
                        <select
                            prop:value=move || state.resume_data.with(|r| r.cv_language.clone())
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                state.resume_data.update(|r| r.cv_language = val);
                                state.save_to_storage();
                            }
                            class="text-[12px] font-bold uppercase tracking-wider bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 rounded-full px-3 py-2 outline-none border border-transparent focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 transition-all hover:bg-slate-200 dark:hover:bg-slate-700 cursor-pointer appearance-none shadow-sm"
                            style="padding-right: 2rem; background-image: url(\"data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e\"); background-position: right 0.25rem center; background-repeat: no-repeat; background-size: 1.5em 1.5em;"
                        >
                            {LANGUAGE_OPTIONS.iter().map(|opt| {
                                let code = opt.code;
                                let label = opt.label;
                                view! {
                                    <option value=code>{label}</option>
                                }
                            }).collect_view()}
                        </select>
                        <div class="w-px h-5 bg-gray-200 dark:bg-white/10 mx-1.5" />
                        <DarkModeToggle />
                    </div>
                </div>
            </header>

            {/* Main */}
            <div class="flex flex-1 overflow-hidden print:block print:overflow-visible">
                {/* Left Panel — Forms */}
                <aside class="w-[440px] flex-shrink-0 overflow-y-auto border-r border-slate-200 dark:border-white/[0.05] bg-slate-50 dark:bg-[#0a0a0a] p-5 animate-stagger space-y-4 print:hidden z-10 shadow-xl">
                    <TemplateToggle />
                    <TextScaleSlider />
                    <SpacingSlider />
                    <FontScaleSlider />
                    <StyleCustomizer />
                    <LinkedInModal />

                    <Accordion title=Signal::derive(move || dict().profile.to_string()) icon="👤".to_string() default_open=true>
                        <PersonalInfoForm />
                    </Accordion>

                    <Accordion title=Signal::derive(move || format!("{} (Summary)", dict().profile)) icon="📝".to_string() default_open=true>
                        <SummaryForm />
                    </Accordion>

                    <Accordion title=Signal::derive(move || dict().experience.to_string()) icon="💼".to_string()>
                        <ExperienceForm />
                    </Accordion>

                    <Accordion title=Signal::derive(move || dict().volunteering.to_string()) icon="🤝".to_string()>
                        <VolunteeringForm />
                    </Accordion>

                    <Accordion title=Signal::derive(move || dict().education.to_string()) icon="🎓".to_string()>
                        <EducationForm />
                    </Accordion>

                    <Accordion title=Signal::derive(move || dict().certifications.to_string()) icon="📜".to_string()>
                        <CertificationsForm />
                    </Accordion>

                    <Accordion title=Signal::derive(move || dict().technical_skills.to_string()) icon="💻".to_string()>
                        <TechnicalSkillsForm />
                    </Accordion>

                    <Accordion title=Signal::derive(move || dict().soft_skills.to_string()) icon="🤝".to_string()>
                        <SoftSkillsForm />
                    </Accordion>

                    <Accordion title=Signal::derive(move || dict().languages.to_string()) icon="🌍".to_string()>
                        <LanguagesForm />
                    </Accordion>

                    <Accordion title=Signal::derive(move || dict().interests.to_string()) icon="🎨".to_string()>
                        <InterestsForm />
                    </Accordion>

                    <Accordion title=Signal::derive(move || "ATS Hidden Keywords".to_string()) icon="🔍".to_string()>
                        <HiddenKeywordsForm />
                    </Accordion>

                    <div class="pt-2">
                        <DataActions />
                        <ExportButton />
                    </div>
                </aside>

                {/* Right Panel — Preview */}
                <main class="flex-1 overflow-y-auto bg-slate-200 dark:bg-black p-8 print:p-0 print:m-0 print:bg-none print:overflow-visible flex flex-col items-center">
                    <div class="max-w-[900px] mx-auto animate-scale-in print:max-w-none print:mx-0 print:animate-none">
                        <div
                            id="cv-preview"
                            class="rounded-none overflow-hidden shadow-[0_8px_30px_rgb(0,0,0,0.12)] dark:shadow-[0_8px_30px_rgb(0,0,0,0.8)] ring-1 ring-black/5 dark:ring-white/5 transition-shadow duration-500 hover:shadow-[0_20px_60px_-15px_rgba(0,0,0,0.2)] dark:hover:shadow-[0_20px_60px_-15px_rgba(0,0,0,0.9)] print:shadow-none print:ring-0"
                            style=move || {
                                let zoom = state.text_scale.get();
                                let ss = state.spacing_scale.get();
                                let fs = state.font_scale.get();
                                let accent = state.accent_color.get();
                                let sidebar = state.sidebar_color.get();
                                let font = state.font_family.get();
                                format!("zoom: {zoom}; --ss: {ss}; --fs: {fs}; --accent: {accent}; --accent-light: {accent}20; --sidebar: {sidebar}; font-family: '{font}', sans-serif;")
                            }
                        >
                            {move || match state.active_template.get() {
                                TemplateType::Ats => view! { <ATSMinimalist /> }.into_any(),
                                TemplateType::Modern => view! { <ModernCreative /> }.into_any(),
                                TemplateType::Timeline => view! { <TimelineCreative /> }.into_any(),
                                TemplateType::TimelineTwoColumn => view! { <TimelineTwoColumn /> }.into_any(),
                            }}
                        </div>
                    </div>
                </main>
            </div>
        </div>
    }
}
