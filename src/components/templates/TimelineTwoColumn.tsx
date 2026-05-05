import { useResumeStore } from '@/store/useResumeStore';
import { i18n, getSafeLanguage } from '@/lib/i18n';
import { formatDateNumeric } from '@/lib/date';
import type { PersonalInfo } from '@/types/resume';
import ContactBlock from './_shared/ContactBlock';
import 'bootstrap-icons/font/bootstrap-icons.css';

export default function TimelineTwoColumn() {
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
    <article lang={lang} className="bg-white text-gray-900 w-full max-w-[210mm] mx-auto text-[11.5px] leading-[1.5] min-h-[297mm] max-h-[297mm] overflow-hidden flex shadow-2xl print:shadow-none print:m-0">
      {/* Left Sidebar */}
      <aside className="w-[33%] text-white p-6 flex flex-col self-stretch" style={{ backgroundColor: 'var(--sidebar, #1e293b)' }}>
        {/* Photo */}
        {!hidePhoto && personalInfo.photoUrl && (
          <div className="flex justify-center mb-4">
            <img
              src={personalInfo.photoUrl}
              alt={personalInfo.fullName}
              className="w-24 h-24 rounded-full object-cover" style={{ boxShadow: '0 0 0 4px var(--accent, #6366f1)33' }}
            />
          </div>
        )}

        {/* Name & Title */}
        <header className="mb-5 text-center">
          {personalInfo.fullName && (
            <h1 className="text-xl font-bold leading-tight tracking-tight mb-1">{personalInfo.fullName}</h1>
          )}
          {personalInfo.title && (
            <p className="text-xs font-medium uppercase tracking-wider" style={{ color: 'var(--accent, #6366f1)' }}>{personalInfo.title}</p>
          )}
        </header>

        {/* Contact Info */}
        {(personalInfo.email || personalInfo.phone || personalInfo.location || personalInfo.linkedin || personalInfo.github || personalInfo.website || personalInfo.drivingLicense) && (
          <section className="mb-5">
            <h2 className="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
              {i18n[lang].contact}
            </h2>
            <ContactBlock
              personalInfo={personalInfo}
              lang={lang}
              listClassName="space-y-2 list-none"
              rowClassName="flex items-start gap-2.5"
              iconClassName="mt-[1px] text-[11px]"
              valueClassName="text-[11px] text-slate-200 leading-tight"
            />
          </section>
        )}

        {/* Summary (Profile) */}
        {summary && (
          <section className="mb-5">
            <h2 className="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
              {i18n[lang].profile}
            </h2>
            <p className="text-slate-300 text-[11px] leading-relaxed whitespace-pre-wrap">
              {summary}
            </p>
          </section>
        )}

        {/* Technical Skills */}
        {technicalSkills.length > 0 && (
          <section className="mb-5">
            <h2 className="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
              {i18n[lang].technicalSkills}
            </h2>
            <ul className="flex flex-wrap gap-1 list-none">
              {technicalSkills.map((skill, i) => (
                <li
                  key={typeof skill === 'string' ? `tech-${i}` : skill.id}
                  className="px-1.5 py-0.5 bg-slate-800 border border-slate-700 text-slate-200 text-[10px] rounded font-medium"
                >
                  {typeof skill === 'string' ? skill : skill.name}
                </li>
              ))}
            </ul>
          </section>
        )}

        {/* Soft Skills */}
        {softSkills.length > 0 && (
          <section className="mb-5">
            <h2 className="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
              {i18n[lang].softSkills}
            </h2>
            <ul className="space-y-1 list-none">
              {softSkills.map((skill, i) => (
                <li key={`soft-${i}`} className="text-[11px] text-slate-300 flex items-center gap-1.5">
                  <span aria-hidden="true"><i className="bi bi-check-circle-fill text-[9px]" style={{ color: 'var(--accent, #6366f1)' }}></i></span>
                  {typeof skill === 'string' ? skill : (skill as { name: string }).name}
                </li>
              ))}
            </ul>
          </section>
        )}

        {/* Languages */}
        {languages.length > 0 && (
          <section className="mb-5">
            <h2 className="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
              {i18n[lang].languages}
            </h2>
            <ul className="flex flex-col gap-1 list-none">
              {languages.map((l, i) => {
                const display = typeof l === 'string'
                  ? l
                  : (() => {
                      const o = l as { name: string; level?: string };
                      return `${o.name}${o.level ? ` (${o.level})` : ''}`;
                    })();
                return (
                  <li key={`lang-${i}`} className="flex items-center gap-1.5 text-slate-300">
                    <span aria-hidden="true"><i className="bi bi-chat-left-text-fill text-[9px]" style={{ color: 'var(--accent, #6366f1)' }}></i></span>
                    <span className="text-[11px] font-medium">{display}</span>
                  </li>
                );
              })}
            </ul>
          </section>
        )}

        {/* Interests */}
        {interests.length > 0 && (
          <section className="mb-5">
            <h2 className="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
              {i18n[lang].interests}
            </h2>
            <ul className="space-y-2 list-none">
              {interests.map((it, i) => {
                const o = typeof it === 'string' ? { id: String(i), name: it, description: '' } : it;
                return (
                  <li key={o.id}>
                    <h3 className="text-[11px] font-bold text-slate-100 leading-tight">{o.name}</h3>
                    {o.description && (
                      <p className="text-[10px] text-slate-300 leading-snug mt-0.5 whitespace-pre-wrap">{o.description}</p>
                    )}
                  </li>
                );
              })}
            </ul>
          </section>
        )}
      </aside>

      {/* Right Main Content */}
      <main className="w-[67%] p-6 bg-slate-50 h-[297mm]">

        {/* Objective */}
        {personalInfo.objective && (
          <section className="mb-6">
            <p className="text-slate-700 italic text-[11px] leading-relaxed pl-3 py-0.5 border-l-2" style={{ borderColor: 'var(--accent, #6366f1)' }}>
              "{personalInfo.objective}"
            </p>
          </section>
        )}

        {/* Experience Timeline */}
        {experience.length > 0 && (
          <section className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-900 mb-3.5 flex items-center gap-2">
              <div aria-hidden="true" className="w-5 h-[2px]" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
              {i18n[lang].experience}
            </h2>
            <div className="relative ml-[5px] space-y-4" style={{ borderLeft: '2px solid var(--accent-light, #c7d2fe)' }}>
              {experience.map((exp) => (
                <div key={exp.id} className="relative pl-5">
                  <div aria-hidden="true" className="absolute -left-[7px] top-1.5 w-3 h-3 rounded-full ring-4 ring-slate-50" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
                  <div className="flex flex-col gap-0.5 mb-1">
                    <h3 className="font-bold text-slate-900 text-[12.5px]">{exp.position}</h3>
                    <div className="flex justify-between items-center pr-2">
                      <span className="font-semibold text-[11.5px]" style={{ color: 'var(--accent, #6366f1)' }}>{exp.company}</span>
                      <span className="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                        {formatDateNumeric(exp.startDate)}
                        {(exp.startDate || exp.endDate || exp.current) && ' — '}
                        {exp.current ? i18n[lang].present : formatDateNumeric(exp.endDate)}
                      </span>
                    </div>
                  </div>
                  {exp.description && (
                    <p className="text-slate-600 text-[11px] leading-snug mt-1 whitespace-pre-wrap">{exp.description}</p>
                  )}
                </div>
              ))}
            </div>
          </section>
        )}

        {/* Volunteering Timeline */}
        {volunteering.length > 0 && (
          <section className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-900 mb-3.5 flex items-center gap-2">
              <div aria-hidden="true" className="w-5 h-[2px]" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
              {i18n[lang].volunteering}
            </h2>
            <div className="relative ml-[5px] space-y-4" style={{ borderLeft: '2px solid var(--accent-light, #c7d2fe)' }}>
              {volunteering.map((v) => (
                <div key={v.id} className="relative pl-5">
                  <div aria-hidden="true" className="absolute -left-[7px] top-1.5 w-3 h-3 rounded-full ring-4 ring-slate-50" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
                  <div className="flex flex-col gap-0.5 mb-1">
                    <h3 className="font-bold text-slate-900 text-[12.5px]">{v.role}</h3>
                    <div className="flex justify-between items-center pr-2">
                      <span className="font-semibold text-[11.5px]" style={{ color: 'var(--accent, #6366f1)' }}>{v.organization}</span>
                      <span className="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                        {formatDateNumeric(v.startDate)}
                        {(v.startDate || v.endDate || v.current) && ' — '}
                        {v.current ? i18n[lang].present : formatDateNumeric(v.endDate)}
                      </span>
                    </div>
                  </div>
                  {v.description && (
                    <p className="text-slate-600 text-[11px] leading-snug mt-1 whitespace-pre-wrap">{v.description}</p>
                  )}
                </div>
              ))}
            </div>
          </section>
        )}

        {/* Education Timeline */}
        {education.length > 0 && (
          <section className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-900 mb-3.5 flex items-center gap-2">
              <div aria-hidden="true" className="w-5 h-[2px]" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
              {i18n[lang].education}
            </h2>
            <div className="relative ml-[5px] space-y-4" style={{ borderLeft: '2px solid var(--accent-light, #c7d2fe)' }}>
              {education.map((edu) => (
                <div key={edu.id} className="relative pl-5">
                  <div aria-hidden="true" className="absolute -left-[7px] top-1.5 w-3 h-3 rounded-full ring-4 ring-slate-50" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
                  <div className="flex flex-col gap-0.5 mb-1">
                    <h3 className="font-bold text-slate-900 text-[12.5px]">{edu.degree}</h3>
                    {edu.field && <p className="text-[11.5px] font-medium text-slate-700">{edu.field}</p>}
                    <div className="flex justify-between items-center pr-2">
                      <span className="font-semibold text-[11.5px]" style={{ color: 'var(--accent, #6366f1)' }}>{edu.institution}</span>
                      <span className="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                        {formatDateNumeric(edu.startDate)}
                        {(edu.startDate || edu.endDate) && ' — '}
                        {edu.endDate ? formatDateNumeric(edu.endDate) : i18n[lang].present}
                      </span>
                    </div>
                  </div>
                  {edu.description && (
                    <p className="text-slate-600 text-[11px] leading-snug mt-1 whitespace-pre-wrap">{edu.description}</p>
                  )}
                </div>
              ))}
            </div>
          </section>
        )}

        {/* Certifications Timeline */}
        {certifications.length > 0 && (
          <section className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-900 mb-3.5 flex items-center gap-2">
              <div aria-hidden="true" className="w-5 h-[2px]" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
              {i18n[lang].certifications}
            </h2>
            <div className="relative ml-[5px] space-y-4" style={{ borderLeft: '2px solid var(--accent-light, #c7d2fe)' }}>
              {certifications.map((cert) => (
                <div key={cert.id} className="relative pl-5">
                  <div aria-hidden="true" className="absolute -left-[7px] top-1.5 w-3 h-3 rounded-full ring-4 ring-slate-50" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
                  <div className="flex flex-col gap-0.5 mb-1">
                    <h3 className="font-bold text-slate-900 text-[12.5px]">
                      {cert.name}
                      {cert.url && (
                        <a href={cert.url} target="_blank" rel="noopener noreferrer" className="ml-2 text-cyan-600 hover:text-cyan-800 break-all text-xs print:text-gray-900 print:no-underline">
                          <span aria-hidden="true"><i className="bi bi-box-arrow-up-right print:hidden"></i></span>
                          <span className="hidden print:inline"> ({cert.url})</span>
                        </a>
                      )}
                    </h3>
                    <div className="flex justify-between items-center pr-2">
                      <span className="font-semibold text-[11.5px]" style={{ color: 'var(--accent, #6366f1)' }}>{cert.issuer}</span>
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
      </main>
    </article>
  );
}
