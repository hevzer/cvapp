import { useResumeStore } from '@/store/useResumeStore';
import { i18n, getSafeLanguage } from '@/lib/i18n';
import { formatDateNumeric } from '@/lib/date';
import type { PersonalInfo } from '@/types/resume';
import ContactBlock from './_shared/ContactBlock';
import 'bootstrap-icons/font/bootstrap-icons.css';

export default function TimelineCreative() {
  const resume = useResumeStore((s) => s.resumeData);
  const hidePhoto = useResumeStore((s) => s.hidePhoto);
  const personalInfo: Partial<PersonalInfo> = resume.personalInfo || {};
  const summary = resume.summary || '';
  const experience = resume.experience || [];
  const education = resume.education || [];
  const volunteering = resume.volunteering || [];
  const certifications = resume.certifications || [];
  const technicalSkills = resume.technicalSkills || [];
  const softSkills = resume.softSkills || [];
  const languages = resume.languages || [];
  const interests = resume.interests || [];

  const lang = getSafeLanguage(resume.cvLanguage);

  const hasContent =
    personalInfo.fullName ||
    summary ||
    experience.length > 0 ||
    volunteering.length > 0 ||
    education.length > 0 ||
    certifications.length > 0 ||
    technicalSkills.length > 0 ||
    softSkills.length > 0 ||
    languages.length > 0 ||
    interests.length > 0;

  if (!hasContent) {
    return (
      <div className="flex items-center justify-center h-full text-gray-400 dark:text-gray-500">
        <div className="text-center">
          <div className="text-5xl mb-4">📄</div>
          <p className="text-lg font-medium">Your CV preview will appear here</p>
          <p className="text-sm mt-1">Start filling in the form on the left</p>
        </div>
      </div>
    );
  }

  return (
    <article lang={lang} className="bg-slate-50 text-slate-900 w-full max-w-[210mm] mx-auto text-[11.5px] leading-[1.5] min-h-[297mm] max-h-[297mm] overflow-hidden print:w-[210mm] print:-m-4">
      {/* Header Section */}
      <header className="bg-white px-6 py-4 border-b border-slate-200">
        <div className="flex items-center gap-4">
          {!hidePhoto && personalInfo.photoUrl && (
            <img
              src={personalInfo.photoUrl}
              alt={personalInfo.fullName}
              className="w-20 h-20 rounded-2xl object-cover shadow-sm ring-1 ring-slate-200"
            />
          )}
          <div className="flex-1">
            {personalInfo.fullName && (
              <h1 className="text-2xl font-extrabold text-slate-900 tracking-tight">
                {personalInfo.fullName}
              </h1>
            )}
            {personalInfo.title && (
              <p className="text-sm font-semibold mt-0.5" style={{ color: 'var(--accent, #6366f1)' }}>
                {personalInfo.title}
              </p>
            )}
            {/* Contact Grid (semantic ul + sr-only labels) */}
            <ContactBlock
              personalInfo={personalInfo}
              lang={lang}
              listClassName="grid grid-cols-2 gap-y-1.5 gap-x-4 mt-4 text-sm text-slate-600 list-none"
              rowClassName="flex items-center gap-2"
              iconClassName="text-slate-400 text-xs text-center w-4"
              valueClassName=""
            />
          </div>
        </div>
      </header>

      {/* Main Two-Column Layout */}
      <div className="flex px-6 py-5 gap-6">
        {/* Left Column (Timeline for Experience/Education) */}
        <div className="w-[62%]">
          {summary && (
            <section className="mb-5">
              <h2 className="text-[11.5px] font-bold text-slate-900 uppercase tracking-widest mb-2.5 flex items-center gap-2">
                <span aria-hidden="true"><i className="bi bi-person-lines-fill" style={{ color: 'var(--accent, #6366f1)' }}></i></span>
                {i18n[lang].profile}
              </h2>
              <p className="text-slate-700 leading-[1.5] text-[11px] bg-white p-3 rounded-xl border border-slate-100 shadow-sm shadow-slate-200/50 whitespace-pre-wrap">
                {summary}
              </p>
            </section>
          )}

          {experience.length > 0 && (
            <section className="mb-5">
              <h2 className="text-[11.5px] font-bold text-slate-900 uppercase tracking-widest mb-2.5 flex items-center gap-2">
                <span aria-hidden="true"><i className="bi bi-briefcase-fill" style={{ color: 'var(--accent, #6366f1)' }}></i></span>
                {i18n[lang].experience}
              </h2>
              <div className="relative ml-2.5 space-y-4" style={{ borderLeft: '2px solid var(--accent-light, #e0e7ff)' }}>
                {experience.map((exp) => (
                  <div key={exp.id} className="relative pl-5">
                    <div aria-hidden="true" className="absolute -left-[5.5px] top-1.5 w-2 h-2 rounded-full ring-4 ring-slate-50" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
                    <div className="mb-1">
                      <h3 className="text-[12px] font-bold text-slate-900">{exp.position}</h3>
                      <div className="flex justify-between items-center mt-0.5">
                        <span className="font-semibold text-[11px]" style={{ color: 'var(--accent, #6366f1)' }}>{exp.company}</span>
                        <span className="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                          {formatDateNumeric(exp.startDate)}
                          {(exp.startDate || exp.endDate || exp.current) && ' — '}
                          {exp.current ? i18n[lang].present : formatDateNumeric(exp.endDate)}
                        </span>
                      </div>
                    </div>
                    {exp.description && (
                      <p className="text-slate-600 text-[11px] leading-[1.5] mt-1 whitespace-pre-wrap">{exp.description}</p>
                    )}
                  </div>
                ))}
              </div>
            </section>
          )}

          {volunteering.length > 0 && (
            <section className="mb-5">
              <h2 className="text-[11.5px] font-bold text-slate-900 uppercase tracking-widest mb-2.5 flex items-center gap-2">
                <span aria-hidden="true"><i className="bi bi-heart-fill" style={{ color: 'var(--accent, #6366f1)' }}></i></span>
                {i18n[lang].volunteering}
              </h2>
              <div className="relative ml-2.5 space-y-4" style={{ borderLeft: '2px solid var(--accent-light, #e0e7ff)' }}>
                {volunteering.map((v) => (
                  <div key={v.id} className="relative pl-5">
                    <div aria-hidden="true" className="absolute -left-[5.5px] top-1.5 w-2 h-2 rounded-full ring-4 ring-slate-50" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
                    <div className="mb-1">
                      <h3 className="text-[12px] font-bold text-slate-900">{v.role}</h3>
                      <div className="flex justify-between items-center mt-0.5">
                        <span className="font-semibold text-[11px]" style={{ color: 'var(--accent, #6366f1)' }}>{v.organization}</span>
                        <span className="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                          {formatDateNumeric(v.startDate)}
                          {(v.startDate || v.endDate || v.current) && ' — '}
                          {v.current ? i18n[lang].present : formatDateNumeric(v.endDate)}
                        </span>
                      </div>
                    </div>
                    {v.description && (
                      <p className="text-slate-600 text-[11px] leading-[1.5] mt-1 whitespace-pre-wrap">{v.description}</p>
                    )}
                  </div>
                ))}
              </div>
            </section>
          )}

          {education.length > 0 && (
            <section className="mb-5">
              <h2 className="text-xs font-bold text-slate-900 uppercase tracking-widest mb-2.5 flex items-center gap-2">
                <span aria-hidden="true"><i className="bi bi-mortarboard-fill" style={{ color: 'var(--accent, #6366f1)' }}></i></span>
                {i18n[lang].education}
              </h2>
              <div className="relative ml-2.5 space-y-4" style={{ borderLeft: '2px solid var(--accent-light, #e0e7ff)' }}>
                {education.map((edu) => (
                  <div key={edu.id} className="relative pl-5">
                    <div aria-hidden="true" className="absolute -left-[5.5px] top-1.5 w-2 h-2 rounded-full ring-4 ring-slate-50" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
                    <div className="mb-1">
                      <h3 className="text-[12.5px] font-bold text-slate-900">
                        {edu.degree}
                      </h3>
                      {edu.field && (
                        <p className="text-[11.5px] font-medium text-slate-700 mt-0.5">{edu.field}</p>
                      )}
                      <div className="flex justify-between items-center mt-0.5">
                        <span className="font-medium text-[11px]" style={{ color: 'var(--accent, #6366f1)' }}>{edu.institution}</span>
                        <span className="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                          {formatDateNumeric(edu.startDate)}
                          {(edu.startDate || edu.endDate) && ' — '}
                          {edu.endDate ? formatDateNumeric(edu.endDate) : i18n[lang].present}
                        </span>
                      </div>
                    </div>
                    {edu.description && (
                      <p className="text-slate-600 text-[11px] leading-[1.5] mt-1.5 whitespace-pre-wrap">{edu.description}</p>
                    )}
                  </div>
                ))}
              </div>
            </section>
          )}

          {certifications.length > 0 && (
            <section className="relative">
              <div className="flex items-center gap-2 mb-3">
                <div aria-hidden="true" className="w-6 h-6 rounded flex items-center justify-center text-white" style={{ backgroundColor: 'var(--accent, #6366f1)' }}>
                  <i className="bi bi-award text-xs"></i>
                </div>
                <h2 className="text-[13px] font-bold text-slate-900 uppercase tracking-widest">
                  {i18n[lang].certifications}
                </h2>
              </div>
              <div className="relative ml-2.5 space-y-4" style={{ borderLeft: '2px solid var(--accent-light, #e0e7ff)' }}>
                {certifications.map((cert) => (
                  <div key={cert.id} className="relative pl-5">
                    <div aria-hidden="true" className="absolute -left-[5.5px] top-1.5 w-2 h-2 rounded-full ring-4 ring-slate-50" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
                    <div className="mb-1">
                      <h3 className="text-[12.5px] font-bold text-slate-900">
                        {cert.name}
                        {cert.url && (
                          <a href={cert.url} target="_blank" rel="noopener noreferrer" className="ml-2 text-cyan-600 hover:text-cyan-800 break-all text-xs print:text-gray-900 print:no-underline">
                            <span aria-hidden="true"><i className="bi bi-box-arrow-up-right print:hidden"></i></span>
                            <span className="hidden print:inline"> ({cert.url})</span>
                          </a>
                        )}
                      </h3>
                      <div className="flex justify-between items-center mt-0.5">
                        <span className="font-medium text-[11px]" style={{ color: 'var(--accent, #6366f1)' }}>{cert.issuer}</span>
                        <span className="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                          {formatDateNumeric(cert.date)}
                        </span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </section>
          )}
        </div>

        {/* Right Column (Objective, Skills, Languages, Interests) */}
        <aside className="w-[38%] space-y-5">
          {personalInfo.objective && (
            <section>
              <p className="text-slate-700 italic text-[11px] bg-white/50 p-2 rounded border border-slate-200/50">
                "{personalInfo.objective}"
              </p>
            </section>
          )}

          {technicalSkills.length > 0 && (
            <section>
              <h2 className="text-[11px] font-bold text-slate-900 uppercase tracking-widest mb-2 border-b-2 border-slate-200 pb-1.5">
                {i18n[lang].technicalSkills}
              </h2>
              <ul className="flex flex-wrap gap-1 list-none">
                {technicalSkills.map((skill, i) => (
                  <li
                    key={typeof skill === 'string' ? `tech-${i}` : skill.id}
                    className="px-2 py-0.5 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 text-[11px] rounded font-medium border border-gray-200 dark:border-gray-700"
                  >
                    {typeof skill === 'string' ? skill : skill.name}
                  </li>
                ))}
              </ul>
            </section>
          )}

          {softSkills.length > 0 && (
            <section>
              <h2 className="text-[11px] font-bold text-slate-900 uppercase tracking-widest mb-2 border-b-2 border-slate-200 pb-1.5">
                {i18n[lang].softSkills}
              </h2>
              <ul className="space-y-1 list-none">
                {softSkills.map((skill, i) => (
                  <li key={`soft-${i}`} className="text-[11px] text-slate-700 flex items-center gap-1.5">
                    <span aria-hidden="true"><i className="bi bi-check2 text-xs" style={{ color: 'var(--accent, #6366f1)' }}></i></span>
                    {typeof skill === 'string' ? skill : (skill as { name: string }).name}
                  </li>
                ))}
              </ul>
            </section>
          )}

          {languages.length > 0 && (
            <section>
              <h2 className="text-[11px] font-bold text-slate-900 uppercase tracking-widest mb-2 border-b-2 border-slate-200 pb-1.5">
                {i18n[lang].languages}
              </h2>
              <ul className="flex flex-col gap-1.5 list-none">
                {languages.map((langItem, i) => (
                  <li key={`lang-${i}`} className="flex items-center gap-2 bg-white px-2.5 py-1.5 border border-slate-100 rounded shadow-sm shadow-slate-200/30">
                    <span aria-hidden="true"><i className="bi bi-translate text-slate-400 text-[10px]"></i></span>
                    <span className="text-[11px] text-slate-700 font-medium">{langItem}</span>
                  </li>
                ))}
              </ul>
            </section>
          )}

          {interests.length > 0 && (
            <section>
              <h2 className="text-[11px] font-bold text-slate-900 uppercase tracking-widest mb-2 border-b-2 border-slate-200 pb-1.5">
                {i18n[lang].interests}
              </h2>
              <ul className="space-y-1.5 list-none">
                {interests.map((it, i) => {
                  const o = typeof it === 'string' ? { id: String(i), name: it, description: '' } : it;
                  return (
                    <li key={o.id}>
                      <h3 className="text-[11.5px] font-bold text-slate-900">{o.name}</h3>
                      {o.description && (
                        <p className="text-[10.5px] text-slate-600 leading-snug mt-0.5 whitespace-pre-wrap">{o.description}</p>
                      )}
                    </li>
                  );
                })}
              </ul>
            </section>
          )}
        </aside>
      </div>
    </article>
  );
}
