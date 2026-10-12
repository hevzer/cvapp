use crate::date::format_date_numeric;
use crate::i18n::get_dictionary;
use crate::state::AppState;
use crate::templates::contact_block::ContactBlock;
use leptos::prelude::*;

#[component]
pub fn TimelineCreative() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;
    let hide_photo = state.hide_photo;

    view! {
        {move || {
            let data = resume.get();
            let p = data.personal_info.clone();
            let lang = data.cv_language.clone();
            let dict = get_dictionary(&lang);

            let has_content = !p.full_name.is_empty()
                || !data.summary.is_empty()
                || !data.experience.is_empty()
                || !data.volunteering.is_empty()
                || !data.education.is_empty()
                || !data.certifications.is_empty()
                || !data.technical_skills.is_empty()
                || !data.soft_skills.is_empty()
                || !data.languages.is_empty()
                || !data.interests.is_empty();

            if !has_content {
                return view! {
                    <div class="flex items-center justify-center h-full text-gray-400 dark:text-gray-500 min-h-[400px]">
                        <div class="text-center">
                            <div class="text-5xl mb-4">"📄"</div>
                            <p class="text-lg font-medium">"Your CV preview will appear here"</p>
                            <p class="text-sm mt-1">"Start filling in the form on the left"</p>
                        </div>
                    </div>
                }.into_any();
            }

            view! {
                <article class="bg-slate-50 text-slate-900 w-full max-w-[210mm] mx-auto text-[11.5px] leading-[1.5] min-h-[297mm] max-h-[297mm] overflow-hidden">
                    {/* Header Section */}
                    <header class="bg-white px-6 py-4 border-b border-slate-200">
                        <div class="flex items-center gap-4">
                            {if !hide_photo.get() && !p.photo_url.is_empty() {
                                view! {
                                    <img src=p.photo_url.clone() alt=p.full_name.clone() class="w-20 h-20 rounded-2xl object-cover shadow-sm ring-1 ring-slate-200" />
                                }.into_any()
                            } else { view! { <span></span> }.into_any() }}

                            <div class="flex-1">
                                {if !p.full_name.is_empty() {
                                    view! { <h1 class="text-2xl font-extrabold text-slate-900 tracking-tight">{p.full_name.clone()}</h1> }.into_any()
                                } else { view! { <span></span> }.into_any() }}

                                {if !p.title.is_empty() {
                                    view! {
                                        <p class="text-sm font-semibold mt-0.5" style="color: var(--accent, #6366f1)">
                                            {p.title.clone()}
                                        </p>
                                    }.into_any()
                                } else { view! { <span></span> }.into_any() }}

                                <ContactBlock
                                    personal_info=p.clone()
                                    lang=lang.clone()
                                    list_class_name="grid grid-cols-2 gap-y-1.5 gap-x-4 mt-4 text-sm text-slate-600 list-none"
                                    row_class_name="flex items-center gap-2"
                                    icon_class_name="text-slate-400 text-xs text-center w-4"
                                    value_class_name=""
                                />
                            </div>
                        </div>
                    </header>

                    {/* Main Two-Column Layout */}
                    <div class="flex px-6 py-5 gap-6">
                        {/* Left Column (Timeline for Experience/Education) */}
                        <div class="w-[62%]">
                            {if !data.summary.is_empty() {
                                view! {
                                    <section class="mb-5">
                                        <h2 class="text-[11.5px] font-bold text-slate-900 uppercase tracking-widest mb-2.5 flex items-center gap-2">
                                            <span aria-hidden="true"><i class="bi bi-person-lines-fill" style="color: var(--accent, #6366f1)"></i></span>
                                            {dict.profile}
                                        </h2>
                                        <p class="text-slate-700 leading-[1.5] text-[11px] bg-white p-3 rounded-xl border border-slate-100 shadow-sm shadow-slate-200/50 whitespace-pre-wrap">
                                            {data.summary}
                                        </p>
                                    </section>
                                }.into_any()
                            } else { view! { <span></span> }.into_any() }}

                            {if !data.experience.is_empty() {
                                view! {
                                    <section class="mb-5">
                                        <h2 class="text-[11.5px] font-bold text-slate-900 uppercase tracking-widest mb-2.5 flex items-center gap-2">
                                            <span aria-hidden="true"><i class="bi bi-briefcase-fill" style="color: var(--accent, #6366f1)"></i></span>
                                            {dict.experience}
                                        </h2>
                                        <div class="relative ml-2.5 space-y-4" style="border-left: 2px solid var(--accent-light, #e0e7ff)">
                                            {data.experience.into_iter().map(|exp| {
                                                let s_date = format_date_numeric(&exp.start_date);
                                                let e_date = if exp.current { dict.present.to_string() } else { format_date_numeric(&exp.end_date) };
                                                let has_dates = !exp.start_date.is_empty() || !exp.end_date.is_empty() || exp.current;

                                                view! {
                                                    <div class="relative pl-5">
                                                        <div aria-hidden="true" class="absolute -left-[5.5px] top-1.5 w-2 h-2 rounded-full ring-4 ring-slate-50" style="background-color: var(--accent, #6366f1)"></div>
                                                        <div class="mb-1">
                                                            <h3 class="text-[12px] font-bold text-slate-900">{exp.position}</h3>
                                                            <div class="flex justify-between items-center mt-0.5">
                                                                <span class="font-semibold text-[11px]" style="color: var(--accent, #6366f1)">{exp.company}</span>
                                                                <span class="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                                                                    {s_date}{if has_dates { " — " } else { "" }}{e_date}
                                                                </span>
                                                            </div>
                                                        </div>
                                                        {if !exp.description.is_empty() {
                                                            view! { <p class="text-slate-600 text-[11px] leading-[1.5] mt-1 whitespace-pre-wrap">{exp.description}</p> }.into_any()
                                                        } else { view! { <span></span> }.into_any() }}
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </section>
                                }.into_any()
                            } else { view! { <span></span> }.into_any() }}

                            {if !data.volunteering.is_empty() {
                                view! {
                                    <section class="mb-5">
                                        <h2 class="text-[11.5px] font-bold text-slate-900 uppercase tracking-widest mb-2.5 flex items-center gap-2">
                                            <span aria-hidden="true"><i class="bi bi-heart-fill" style="color: var(--accent, #6366f1)"></i></span>
                                            {dict.volunteering}
                                        </h2>
                                        <div class="relative ml-2.5 space-y-4" style="border-left: 2px solid var(--accent-light, #e0e7ff)">
                                            {data.volunteering.into_iter().map(|v| {
                                                let s_date = format_date_numeric(&v.start_date);
                                                let e_date = if v.current { dict.present.to_string() } else { format_date_numeric(&v.end_date) };
                                                let has_dates = !v.start_date.is_empty() || !v.end_date.is_empty() || v.current;

                                                view! {
                                                    <div class="relative pl-5">
                                                        <div aria-hidden="true" class="absolute -left-[5.5px] top-1.5 w-2 h-2 rounded-full ring-4 ring-slate-50" style="background-color: var(--accent, #6366f1)"></div>
                                                        <div class="mb-1">
                                                            <h3 class="text-[12px] font-bold text-slate-900">{v.role}</h3>
                                                            <div class="flex justify-between items-center mt-0.5">
                                                                <span class="font-semibold text-[11px]" style="color: var(--accent, #6366f1)">{v.organization}</span>
                                                                <span class="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                                                                    {s_date}{if has_dates { " — " } else { "" }}{e_date}
                                                                </span>
                                                            </div>
                                                        </div>
                                                        {if !v.description.is_empty() {
                                                            view! { <p class="text-slate-600 text-[11px] leading-[1.5] mt-1 whitespace-pre-wrap">{v.description}</p> }.into_any()
                                                        } else { view! { <span></span> }.into_any() }}
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </section>
                                }.into_any()
                            } else { view! { <span></span> }.into_any() }}

                            {if !data.education.is_empty() {
                                view! {
                                    <section class="mb-5">
                                        <h2 class="text-xs font-bold text-slate-900 uppercase tracking-widest mb-2.5 flex items-center gap-2">
                                            <span aria-hidden="true"><i class="bi bi-mortarboard-fill" style="color: var(--accent, #6366f1)"></i></span>
                                            {dict.education}
                                        </h2>
                                        <div class="relative ml-2.5 space-y-4" style="border-left: 2px solid var(--accent-light, #e0e7ff)">
                                            {data.education.into_iter().map(|edu| {
                                                let s_date = format_date_numeric(&edu.start_date);
                                                let e_date = if edu.end_date.is_empty() { dict.present.to_string() } else { format_date_numeric(&edu.end_date) };
                                                let has_dates = !edu.start_date.is_empty() || !edu.end_date.is_empty();

                                                view! {
                                                    <div class="relative pl-5">
                                                        <div aria-hidden="true" class="absolute -left-[5.5px] top-1.5 w-2 h-2 rounded-full ring-4 ring-slate-50" style="background-color: var(--accent, #6366f1)"></div>
                                                        <div class="mb-1">
                                                            <h3 class="text-[12.5px] font-bold text-slate-900">{edu.degree}</h3>
                                                            {if !edu.field.is_empty() {
                                                                view! { <p class="text-[11.5px] font-medium text-slate-700 mt-0.5">{edu.field}</p> }.into_any()
                                                            } else { view! { <span></span> }.into_any() }}
                                                            <div class="flex justify-between items-center mt-0.5">
                                                                <span class="font-medium text-[11px]" style="color: var(--accent, #6366f1)">{edu.institution}</span>
                                                                <span class="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                                                                    {s_date}{if has_dates { " — " } else { "" }}{e_date}
                                                                </span>
                                                            </div>
                                                        </div>
                                                        {if !edu.description.is_empty() {
                                                            view! { <p class="text-slate-600 text-[11px] leading-[1.5] mt-1.5 whitespace-pre-wrap">{edu.description}</p> }.into_any()
                                                        } else { view! { <span></span> }.into_any() }}
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </section>
                                }.into_any()
                            } else { view! { <span></span> }.into_any() }}

                            {if !data.certifications.is_empty() {
                                view! {
                                    <section class="relative">
                                        <div class="flex items-center gap-2 mb-3">
                                            <div aria-hidden="true" class="w-6 h-6 rounded flex items-center justify-center text-white" style="background-color: var(--accent, #6366f1)">
                                                <i class="bi bi-award text-xs"></i>
                                            </div>
                                            <h2 class="text-[13px] font-bold text-slate-900 uppercase tracking-widest">
                                                {dict.certifications}
                                            </h2>
                                        </div>
                                        <div class="relative ml-2.5 space-y-4" style="border-left: 2px solid var(--accent-light, #e0e7ff)">
                                            {data.certifications.into_iter().map(|cert| {
                                                view! {
                                                    <div class="relative pl-5">
                                                        <div aria-hidden="true" class="absolute -left-[5.5px] top-1.5 w-2 h-2 rounded-full ring-4 ring-slate-50" style="background-color: var(--accent, #6366f1)"></div>
                                                        <div class="mb-1">
                                                            <h3 class="text-[12.5px] font-bold text-slate-900">
                                                                {cert.name}
                                                                {if !cert.url.is_empty() {
                                                                    view! {
                                                                        <a href=cert.url.clone() target="_blank" rel="noopener noreferrer" class="ml-2 text-cyan-600 hover:text-cyan-800 break-all text-xs print:text-gray-900 print:no-underline">
                                                                            <span aria-hidden="true"><i class="bi bi-box-arrow-up-right print:hidden"></i></span>
                                                                            <span class="hidden print:inline">{format!(" ({})", cert.url)}</span>
                                                                        </a>
                                                                    }.into_any()
                                                                } else { view! { <span></span> }.into_any() }}
                                                            </h3>
                                                            <div class="flex justify-between items-center mt-0.5">
                                                                <span class="font-medium text-[11px]" style="color: var(--accent, #6366f1)">{cert.issuer}</span>
                                                                <span class="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                                                                    {format_date_numeric(&cert.date)}
                                                                </span>
                                                            </div>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </section>
                                }.into_any()
                            } else { view! { <span></span> }.into_any() }}
                        </div>

                        {/* Right Column (Objective, Skills, Languages, Interests) */}
                        <aside class="w-[38%] space-y-5">
                            {if !p.objective.is_empty() {
                                view! {
                                    <section>
                                        <p class="text-slate-700 italic text-[11px] bg-white/50 p-2 rounded border border-slate-200/50">
                                            "\""{p.objective}"\""
                                        </p>
                                    </section>
                                }.into_any()
                            } else { view! { <span></span> }.into_any() }}

                            {if !data.technical_skills.is_empty() {
                                view! {
                                    <section>
                                        <h2 class="text-[11px] font-bold text-slate-900 uppercase tracking-widest mb-2 border-b-2 border-slate-200 pb-1.5">
                                            {dict.technical_skills}
                                        </h2>
                                        <ul class="flex flex-wrap gap-1 list-none">
                                            {data.technical_skills.into_iter().map(|skill| {
                                                view! {
                                                    <li class="px-2 py-0.5 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 text-[11px] rounded font-medium border border-gray-200 dark:border-gray-700">
                                                        {skill.name}
                                                    </li>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </ul>
                                    </section>
                                }.into_any()
                            } else { view! { <span></span> }.into_any() }}

                            {if !data.soft_skills.is_empty() {
                                view! {
                                    <section>
                                        <h2 class="text-[11px] font-bold text-slate-900 uppercase tracking-widest mb-2 border-b-2 border-slate-200 pb-1.5">
                                            {dict.soft_skills}
                                        </h2>
                                        <ul class="space-y-1 list-none">
                                            {data.soft_skills.into_iter().map(|skill| {
                                                view! {
                                                    <li class="text-[11px] text-slate-700 flex items-center gap-1.5">
                                                        <span aria-hidden="true"><i class="bi bi-check2 text-xs" style="color: var(--accent, #6366f1)"></i></span>
                                                        {skill}
                                                    </li>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </ul>
                                    </section>
                                }.into_any()
                            } else { view! { <span></span> }.into_any() }}

                            {if !data.languages.is_empty() {
                                view! {
                                    <section>
                                        <h2 class="text-[11px] font-bold text-slate-900 uppercase tracking-widest mb-2 border-b-2 border-slate-200 pb-1.5">
                                            {dict.languages}
                                        </h2>
                                        <ul class="flex flex-col gap-1.5 list-none">
                                            {data.languages.into_iter().map(|lang_item| {
                                                view! {
                                                    <li class="flex items-center gap-2 bg-white px-2.5 py-1.5 border border-slate-100 rounded shadow-sm shadow-slate-200/30">
                                                        <span aria-hidden="true"><i class="bi bi-translate text-slate-400 text-[10px]"></i></span>
                                                        <span class="text-[11px] text-slate-700 font-medium">{lang_item}</span>
                                                    </li>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </ul>
                                    </section>
                                }.into_any()
                            } else { view! { <span></span> }.into_any() }}

                            {if !data.interests.is_empty() {
                                view! {
                                    <section>
                                        <h2 class="text-[11px] font-bold text-slate-900 uppercase tracking-widest mb-2 border-b-2 border-slate-200 pb-1.5">
                                            {dict.interests}
                                        </h2>
                                        <ul class="space-y-1.5 list-none">
                                            {data.interests.into_iter().map(|it| {
                                                view! {
                                                    <li>
                                                        <h3 class="text-[11.5px] font-bold text-slate-900">{it.name}</h3>
                                                        {if !it.description.is_empty() {
                                                            view! { <p class="text-[10.5px] text-slate-600 leading-snug mt-0.5 whitespace-pre-wrap">{it.description}</p> }.into_any()
                                                        } else { view! { <span></span> }.into_any() }}
                                                    </li>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </ul>
                                    </section>
                                }.into_any()
                            } else { view! { <span></span> }.into_any() }}
                        </aside>
                    </div>
                </article>
            }.into_any()
        }}
    }
}
