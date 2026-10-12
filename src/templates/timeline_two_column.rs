use crate::date::format_date_numeric;
use crate::i18n::get_dictionary;
use crate::state::AppState;
use crate::templates::contact_block::ContactBlock;
use leptos::prelude::*;

#[component]
pub fn TimelineTwoColumn() -> impl IntoView {
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
                <article class="bg-white text-gray-900 w-full max-w-[210mm] mx-auto text-[11.5px] leading-[1.5] min-h-[297mm] max-h-[297mm] overflow-hidden flex shadow-2xl print:shadow-none print:m-0">
                    {/* Left Sidebar */}
                    <aside class="w-[33%] text-white p-6 flex flex-col self-stretch" style="background-color: var(--sidebar, #1e293b);">
                        {if !hide_photo.get() && !p.photo_url.is_empty() {
                            view! {
                                <div class="flex justify-center mb-4">
                                    <img
                                        src=p.photo_url.clone()
                                        alt=p.full_name.clone()
                                        class="w-24 h-24 rounded-full object-cover"
                                        style="box-shadow: 0 0 0 4px var(--accent, #6366f1)33"
                                    />
                                </div>
                            }.into_any()
                        } else { view! { <span></span> }.into_any() }}

                        <header class="mb-5 text-center">
                            {if !p.full_name.is_empty() {
                                view! { <h1 class="text-xl font-bold leading-tight tracking-tight mb-1">{p.full_name.clone()}</h1> }.into_any()
                            } else { view! { <span></span> }.into_any() }}
                            {if !p.title.is_empty() {
                                view! { <p class="text-xs font-medium uppercase tracking-wider" style="color: var(--accent, #6366f1)">{p.title.clone()}</p> }.into_any()
                            } else { view! { <span></span> }.into_any() }}
                        </header>

                        <section class="mb-5">
                            <h2 class="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
                                {dict.contact}
                            </h2>
                            <ContactBlock
                                personal_info=p.clone()
                                lang=lang.clone()
                                list_class_name="space-y-2 list-none"
                                row_class_name="flex items-start gap-2.5"
                                icon_class_name="mt-[1px] text-[11px]"
                                value_class_name="text-[11px] text-slate-200 leading-tight"
                            />
                        </section>

                        {if !data.summary.is_empty() {
                            view! {
                                <section class="mb-5">
                                    <h2 class="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
                                        {dict.profile}
                                    </h2>
                                    <p class="text-slate-300 text-[11px] leading-relaxed whitespace-pre-wrap">
                                        {data.summary}
                                    </p>
                                </section>
                            }.into_any()
                        } else { view! { <span></span> }.into_any() }}

                        {if !data.technical_skills.is_empty() {
                            view! {
                                <section class="mb-5">
                                    <h2 class="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
                                        {dict.technical_skills}
                                    </h2>
                                    <ul class="flex flex-wrap gap-1 list-none">
                                        {data.technical_skills.into_iter().map(|skill| {
                                            view! {
                                                <li class="px-1.5 py-0.5 bg-slate-800 border border-slate-700 text-slate-200 text-[10px] rounded font-medium">
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
                                <section class="mb-5">
                                    <h2 class="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
                                        {dict.soft_skills}
                                    </h2>
                                    <ul class="space-y-1 list-none">
                                        {data.soft_skills.into_iter().map(|skill| {
                                            view! {
                                                <li class="text-[11px] text-slate-300 flex items-center gap-1.5">
                                                    <span aria-hidden="true"><i class="bi bi-check-circle-fill text-[9px]" style="color: var(--accent, #6366f1)"></i></span>
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
                                <section class="mb-5">
                                    <h2 class="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
                                        {dict.languages}
                                    </h2>
                                    <ul class="flex flex-col gap-1 list-none">
                                        {data.languages.into_iter().map(|l| {
                                            view! {
                                                <li class="flex items-center gap-1.5 text-slate-300">
                                                    <span aria-hidden="true"><i class="bi bi-chat-left-text-fill text-[9px]" style="color: var(--accent, #6366f1)"></i></span>
                                                    <span class="text-[11px] font-medium">{l}</span>
                                                </li>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </ul>
                                </section>
                            }.into_any()
                        } else { view! { <span></span> }.into_any() }}

                        {if !data.interests.is_empty() {
                            view! {
                                <section class="mb-5">
                                    <h2 class="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
                                        {dict.interests}
                                    </h2>
                                    <ul class="space-y-2 list-none">
                                        {data.interests.into_iter().map(|it| {
                                            view! {
                                                <li>
                                                    <h3 class="text-[11px] font-bold text-slate-100 leading-tight">{it.name}</h3>
                                                    {if !it.description.is_empty() {
                                                        view! { <p class="text-[10px] text-slate-300 leading-snug mt-0.5 whitespace-pre-wrap">{it.description}</p> }.into_any()
                                                    } else { view! { <span></span> }.into_any() }}
                                                </li>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </ul>
                                </section>
                            }.into_any()
                        } else { view! { <span></span> }.into_any() }}
                    </aside>

                    {/* Right Main Content */}
                    <main class="w-[67%] p-6 bg-slate-50 min-h-[297mm]">
                        {if !p.objective.is_empty() {
                            view! {
                                <section class="mb-6">
                                    <p class="text-slate-700 italic text-[11px] leading-relaxed pl-3 py-0.5 border-l-2" style="border-color: var(--accent, #6366f1)">
                                        "\""{p.objective}"\""
                                    </p>
                                </section>
                            }.into_any()
                        } else { view! { <span></span> }.into_any() }}

                        {if !data.experience.is_empty() {
                            view! {
                                <section class="mb-6">
                                    <h2 class="text-xs font-bold uppercase tracking-widest text-slate-900 mb-3.5 flex items-center gap-2">
                                        <div aria-hidden="true" class="w-5 h-[2px]" style="background-color: var(--accent, #6366f1)"></div>
                                        {dict.experience}
                                    </h2>
                                    <div class="relative ml-[5px] space-y-4" style="border-left: 2px solid var(--accent-light, #c7d2fe)">
                                        {data.experience.into_iter().map(|exp| {
                                            let s_date = format_date_numeric(&exp.start_date);
                                            let e_date = if exp.current { dict.present.to_string() } else { format_date_numeric(&exp.end_date) };
                                            let has_dates = !exp.start_date.is_empty() || !exp.end_date.is_empty() || exp.current;

                                            view! {
                                                <div class="relative pl-5">
                                                  <div aria-hidden="true" class="absolute -left-[7px] top-1.5 w-3 h-3 rounded-full ring-4 ring-slate-50" style="background-color: var(--accent, #6366f1)"></div>
                                                  <div class="flex flex-col gap-0.5 mb-1">
                                                    <h3 class="font-bold text-slate-900 text-[12.5px]">{exp.position}</h3>
                                                    <div class="flex justify-between items-center pr-2">
                                                      <span class="font-semibold text-[11.5px]" style="color: var(--accent, #6366f1)">{exp.company}</span>
                                                      <span class="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                                                        {s_date}{if has_dates { " — " } else { "" }}{e_date}
                                                      </span>
                                                    </div>
                                                  </div>
                                                  {if !exp.description.is_empty() {
                                                      view! { <p class="text-slate-600 text-[11px] leading-snug mt-1 whitespace-pre-wrap">{exp.description}</p> }.into_any()
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
                                <section class="mb-6">
                                    <h2 class="text-xs font-bold uppercase tracking-widest text-slate-900 mb-3.5 flex items-center gap-2">
                                        <div aria-hidden="true" class="w-5 h-[2px]" style="background-color: var(--accent, #6366f1)"></div>
                                        {dict.volunteering}
                                    </h2>
                                    <div class="relative ml-[5px] space-y-4" style="border-left: 2px solid var(--accent-light, #c7d2fe)">
                                        {data.volunteering.into_iter().map(|v| {
                                            let s_date = format_date_numeric(&v.start_date);
                                            let e_date = if v.current { dict.present.to_string() } else { format_date_numeric(&v.end_date) };
                                            let has_dates = !v.start_date.is_empty() || !v.end_date.is_empty() || v.current;

                                            view! {
                                                <div class="relative pl-5">
                                                  <div aria-hidden="true" class="absolute -left-[7px] top-1.5 w-3 h-3 rounded-full ring-4 ring-slate-50" style="background-color: var(--accent, #6366f1)"></div>
                                                  <div class="flex flex-col gap-0.5 mb-1">
                                                    <h3 class="font-bold text-slate-900 text-[12.5px]">{v.role}</h3>
                                                    <div class="flex justify-between items-center pr-2">
                                                      <span class="font-semibold text-[11.5px]" style="color: var(--accent, #6366f1)">{v.organization}</span>
                                                      <span class="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                                                        {s_date}{if has_dates { " — " } else { "" }}{e_date}
                                                      </span>
                                                    </div>
                                                  </div>
                                                  {if !v.description.is_empty() {
                                                      view! { <p class="text-slate-600 text-[11px] leading-snug mt-1 whitespace-pre-wrap">{v.description}</p> }.into_any()
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
                                <section class="mb-6">
                                    <h2 class="text-xs font-bold uppercase tracking-widest text-slate-900 mb-3.5 flex items-center gap-2">
                                        <div aria-hidden="true" class="w-5 h-[2px]" style="background-color: var(--accent, #6366f1)"></div>
                                        {dict.education}
                                    </h2>
                                    <div class="relative ml-[5px] space-y-4" style="border-left: 2px solid var(--accent-light, #c7d2fe)">
                                        {data.education.into_iter().map(|edu| {
                                            let s_date = format_date_numeric(&edu.start_date);
                                            let e_date = if edu.end_date.is_empty() { dict.present.to_string() } else { format_date_numeric(&edu.end_date) };
                                            let has_dates = !edu.start_date.is_empty() || !edu.end_date.is_empty();

                                            view! {
                                                <div class="relative pl-5">
                                                  <div aria-hidden="true" class="absolute -left-[7px] top-1.5 w-3 h-3 rounded-full ring-4 ring-slate-50" style="background-color: var(--accent, #6366f1)"></div>
                                                  <div class="flex flex-col gap-0.5 mb-1">
                                                    <h3 class="font-bold text-slate-900 text-[12.5px]">{edu.degree}</h3>
                                                    {if !edu.field.is_empty() {
                                                        view! { <p class="text-[11.5px] font-medium text-slate-700">{edu.field}</p> }.into_any()
                                                    } else { view! { <span></span> }.into_any() }}
                                                    <div class="flex justify-between items-center pr-2">
                                                      <span class="font-semibold text-[11.5px]" style="color: var(--accent, #6366f1)">{edu.institution}</span>
                                                      <span class="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                                                        {s_date}{if has_dates { " — " } else { "" }}{e_date}
                                                      </span>
                                                    </div>
                                                  </div>
                                                  {if !edu.description.is_empty() {
                                                      view! { <p class="text-slate-600 text-[11px] leading-snug mt-1 whitespace-pre-wrap">{edu.description}</p> }.into_any()
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
                                <section class="mb-6">
                                    <h2 class="text-xs font-bold uppercase tracking-widest text-slate-900 mb-3.5 flex items-center gap-2">
                                        <div aria-hidden="true" class="w-5 h-[2px]" style="background-color: var(--accent, #6366f1)"></div>
                                        {dict.certifications}
                                    </h2>
                                    <div class="relative ml-[5px] space-y-4" style="border-left: 2px solid var(--accent-light, #c7d2fe)">
                                        {data.certifications.into_iter().map(|cert| {
                                            view! {
                                                <div class="relative pl-5">
                                                  <div aria-hidden="true" class="absolute -left-[7px] top-1.5 w-3 h-3 rounded-full ring-4 ring-slate-50" style="background-color: var(--accent, #6366f1)"></div>
                                                  <div class="flex flex-col gap-0.5 mb-1">
                                                    <h3 class="font-bold text-slate-900 text-[12.5px]">
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
                                                    <div class="flex justify-between items-center pr-2">
                                                      <span class="font-semibold text-[11.5px]" style="color: var(--accent, #6366f1)">{cert.issuer}</span>
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
                    </main>
                </article>
            }.into_any()
        }}
    }
}
