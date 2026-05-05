import { useResumeStore } from '@/store/useResumeStore';
import { i18n, getSafeLanguage } from '@/lib/i18n';
import { formatDateNumeric } from '@/lib/date';
import type { PersonalInfo } from '@/types/resume';
import ContactBlock from './_shared/ContactBlock';
import 'bootstrap-icons/font/bootstrap-icons.css';

export default function ModernCreative() {
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
    <article lang={lang} className="bg-white text-gray-900 max-w-[210mm] mx-auto text-[13px] leading-relaxed min-h-[297mm] max-h-[297mm] overflow-hidden flex">
      {/* Left Sidebar */}
      <aside className="w-[35%] text-white p-6 flex flex-col" style={{ backgroundColor: 'var(--sidebar, #1e293b)' }}>
        {/* Photo */}
        {!hidePhoto && personalInfo.photoUrl && (
          <div className="flex justify-center mb-5">
            <img
              src={personalInfo.photoUrl}
              alt={personalInfo.fullName}
              className="w-24 h-24 rounded-full object-cover ring-3 ring-slate-600"
            />
          </div>
        )}
        {/* Name & Title */}
        <header className="mb-6">
          {personalInfo.fullName && (
            <h1 className="text-xl font-bold leading-tight">{personalInfo.fullName}</h1>
          )}
          {personalInfo.title && (
            <p className="text-sm text-slate-300 mt-1 font-medium">{personalInfo.title}</p>
          )}
        </header>

        {/* Contact Info */}
        {(personalInfo.email || personalInfo.phone || personalInfo.location || personalInfo.linkedin || personalInfo.github || personalInfo.website || personalInfo.drivingLicense) && (
          <section className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
              {i18n[lang].contact}
            </h2>
            <ContactBlock
              personalInfo={personalInfo}
              lang={lang}
              listClassName="space-y-2 list-none"
              rowClassName="flex items-start gap-2"
              iconClassName="text-slate-400 mt-0.5 text-xs"
              valueClassName="text-sm text-slate-200"
            />
          </section>
        )}

        {/* Profile (Summary) */}
        {summary && (
          <section className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
              {i18n[lang].profile}
            </h2>
            <p className="text-slate-300 text-xs leading-relaxed whitespace-pre-wrap">{summary}</p>
          </section>
        )}

        {/* Soft Skills */}
        {softSkills.length > 0 && (
          <section className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
              {i18n[lang].softSkills}
            </h2>
            <ul className="space-y-1.5 list-none">
              {softSkills.map((skill, i) => (
                <li key={`soft-${i}`} className="text-xs text-slate-300 flex items-center gap-2">
                  <span aria-hidden="true"><i className="bi bi-check2 text-xs" style={{ color: 'var(--accent, #6366f1)' }}></i></span>
                  {typeof skill === 'string' ? skill : (skill as { name: string }).name}
                </li>
              ))}
            </ul>
          </section>
        )}

        {/* Languages */}
        {languages.length > 0 && (
          <section className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
              {i18n[lang].languages}
            </h2>
            <ul className="flex flex-wrap gap-1.5 list-none">
              {languages.map((l, i) => {
                const display = typeof l === 'string'
                  ? l
                  : (() => {
                      const o = l as { name: string; level?: string };
                      return `${o.name}${o.level ? ` (${o.level})` : ''}`;
                    })();
                return (
                  <li
                    key={`lang-${i}`}
                    className="px-2 py-0.5 bg-slate-700 text-slate-200 text-xs rounded font-medium"
                  >
                    {display}
                  </li>
                );
              })}
            </ul>
          </section>
        )}

        {/* Volunteering (compact, sidebar) */}
        {volunteering.length > 0 && (
          <section className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
              {i18n[lang].volunteering}
            </h2>
            <div className="space-y-3">
              {volunteering.map((v) => (
                <div key={v.id}>
                  <h3 className="text-[12px] font-bold text-slate-100 leading-tight">{v.role}</h3>
                  <p className="text-[11px] text-slate-300 font-medium">{v.organization}</p>
                  <p className="text-[10px] text-slate-400 mt-0.5">
                    {formatDateNumeric(v.startDate)}
                    {(v.startDate || v.endDate || v.current) && ' — '}
                    {v.current ? i18n[lang].present : formatDateNumeric(v.endDate)}
                  </p>
                  {v.description && (
                    <p className="text-[10.5px] text-slate-300 leading-snug mt-1 whitespace-pre-wrap">{v.description}</p>
                  )}
                </div>
              ))}
            </div>
          </section>
        )}

        {/* Interests */}
        {interests.length > 0 && (
          <section className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
              {i18n[lang].interests}
            </h2>
            <ul className="space-y-2 list-none">
              {interests.map((it, i) => {
                const o = typeof it === 'string' ? { id: String(i), name: it, description: '' } : it;
                return (
                  <li key={o.id}>
                    <h3 className="text-[12px] font-bold text-slate-100 leading-tight">{o.name}</h3>
                    {o.description && (
                      <p className="text-[10.5px] text-slate-300 leading-snug mt-0.5 whitespace-pre-wrap">{o.description}</p>
                    )}
                  </li>
                );
              })}
            </ul>
          </section>
        )}
      </aside>

      {/* Right Main Content */}
      <main className="w-[65%] p-8">
        {/* Objective */}
        {personalInfo.objective && (
          <section className="mb-6">
            <p className="text-slate-600 font-medium italic text-sm leading-relaxed pl-4 py-1" style={{ borderLeft: '4px solid var(--accent, #6366f1)' }}>
              "{personalInfo.objective}"
            </p>
          </section>
        )}
        {/* Technical Skills */}
        {technicalSkills.length > 0 && (
          <section className="mb-6">
            <h2 className="text-sm font-bold uppercase tracking-wider text-slate-800 pb-1.5 mb-3" style={{ borderBottom: '2px solid var(--accent, #1e293b)' }}>
              {i18n[lang].technicalSkills}
            </h2>
            <ul className="grid grid-cols-2 gap-x-6 gap-y-4 list-none">
              {technicalSkills.map((skill, i) => (
                <li key={typeof skill === 'string' ? `tech-${i}` : skill.id}>
                  <h3 className="font-bold text-gray-900 text-[13px]">
                    {typeof skill === 'string' ? skill : skill.name}
                  </h3>
                  {typeof skill !== 'string' && skill.description && (
                    <p className="mt-1 text-gray-700 leading-relaxed whitespace-pre-wrap text-[12px]">
                      {skill.description}
                    </p>
                  )}
                </li>
              ))}
            </ul>
          </section>
        )}

        {/* Experience */}
        {experience.length > 0 && (
          <section className="mb-6">
            <h2 className="text-sm font-bold uppercase tracking-wider text-slate-800 pb-1.5 mb-3" style={{ borderBottom: '2px solid var(--accent, #1e293b)' }}>
              {i18n[lang].experience}
            </h2>
            <div className="space-y-5">
              {experience.map((exp) => (
                <div key={exp.id}>
                  <div className="flex items-start justify-between">
                    <div>
                      <h3 className="font-bold text-gray-900">{exp.position}</h3>
                      <p className="text-gray-600 font-medium text-sm">{exp.company}</p>
                    </div>
                    <span className="text-xs text-gray-500 whitespace-nowrap ml-4 mt-0.5 bg-gray-100 px-2 py-0.5 rounded">
                      {formatDateNumeric(exp.startDate)}
                      {(exp.startDate || exp.endDate || exp.current) && ' — '}
                      {exp.current ? i18n[lang].present : formatDateNumeric(exp.endDate)}
                    </span>
                  </div>
                  {exp.description && (
                    <p className="mt-2 text-gray-700 leading-relaxed whitespace-pre-wrap">{exp.description}</p>
                  )}
                </div>
              ))}
            </div>
          </section>
        )}

        {/* Education */}
        {education.length > 0 && (
          <section className="mb-6">
            <h2 className="text-sm font-bold uppercase tracking-wider text-slate-800 pb-1.5 mb-3" style={{ borderBottom: '2px solid var(--accent, #1e293b)' }}>
              {i18n[lang].education}
            </h2>
            <div className="space-y-5">
              {education.map((edu) => (
                <div key={edu.id}>
                  <div className="flex items-start justify-between">
                    <div>
                      <h3 className="font-bold text-gray-900">{edu.degree}</h3>
                      {edu.field && <p className="text-gray-700 font-medium text-sm mt-0.5">{edu.field}</p>}
                      <p className="text-gray-600 font-medium text-[12.5px]">{edu.institution}</p>
                    </div>
                    <span className="text-xs text-gray-500 whitespace-nowrap ml-4 mt-0.5 bg-gray-100 px-2 py-0.5 rounded">
                      {formatDateNumeric(edu.startDate)}
                      {(edu.startDate || edu.endDate) && ' — '}
                      {edu.endDate ? formatDateNumeric(edu.endDate) : i18n[lang].present}
                    </span>
                  </div>
                  {edu.description && (
                    <p className="mt-2 text-gray-700 leading-relaxed whitespace-pre-wrap">{edu.description}</p>
                  )}
                </div>
              ))}
            </div>
          </section>
        )}

        {/* Certifications (rendered in main column when present) */}
        {certifications.length > 0 && (
          <section className="mb-6">
            <h2 className="text-sm font-bold uppercase tracking-wider text-slate-800 pb-1.5 mb-3" style={{ borderBottom: '2px solid var(--accent, #1e293b)' }}>
              {i18n[lang].certifications}
            </h2>
            <ul className="space-y-3 list-none">
              {certifications.map((cert) => (
                <li key={cert.id}>
                  <div className="flex items-start justify-between">
                    <div>
                      <h3 className="font-bold text-gray-900">
                        {cert.name}
                        {cert.url && (
                          <a href={cert.url} target="_blank" rel="noopener noreferrer" className="ml-2 text-cyan-600 hover:text-cyan-800 break-all text-xs print:text-gray-900 print:no-underline">
                            <span aria-hidden="true"><i className="bi bi-box-arrow-up-right print:hidden"></i></span>
                            <span className="hidden print:inline"> ({cert.url})</span>
                          </a>
                        )}
                      </h3>
                      <p className="text-gray-600 font-medium text-sm">{cert.issuer}</p>
                    </div>
                    <span className="text-xs text-gray-500 whitespace-nowrap ml-4 mt-0.5 bg-gray-100 px-2 py-0.5 rounded">
                      {formatDateNumeric(cert.date)}
                    </span>
                  </div>
                </li>
              ))}
            </ul>
          </section>
        )}
      </main>
    </article>
  );
}
