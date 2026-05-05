import { useResumeStore } from '@/store/useResumeStore';
import { i18n, getSafeLanguage } from '@/lib/i18n';
import { formatDateNumeric } from '@/lib/date';
import type { PersonalInfo } from '@/types/resume';

export default function ATSMinimalist() {
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

  const accentHeader = "text-sm font-bold uppercase tracking-wider pb-1 mb-2";
  const accentHeaderStyle = { color: 'var(--accent, #1e293b)', borderBottom: '1px solid var(--accent, #d1d5db)' } as const;

  return (
    <article lang={lang} className="bg-white text-gray-900 p-8 max-w-[210mm] mx-auto text-[13px] leading-relaxed min-h-[297mm] max-h-[297mm] overflow-hidden">
      {/* Header */}
      <header className="text-center pb-4 mb-5" style={{ borderBottom: '2px solid var(--accent, #1e293b)' }}>
        {!hidePhoto && personalInfo.photoUrl && (
          <div className="flex justify-center mb-3">
            <img
              src={personalInfo.photoUrl}
              alt={personalInfo.fullName}
              className="w-16 h-16 rounded-full object-cover"
            />
          </div>
        )}
        {personalInfo.fullName && (
          <h1 className="text-2xl font-bold tracking-wide uppercase text-gray-900">
            {personalInfo.fullName}
          </h1>
        )}
        {personalInfo.title && (
          <p className="text-sm font-medium text-gray-600 mt-1 tracking-wider uppercase">
            {personalInfo.title}
          </p>
        )}
        <ul className="flex flex-wrap items-center justify-center gap-x-4 gap-y-1 mt-3 text-xs text-gray-600 list-none">
          {personalInfo.email && (
            <li><span className="sr-only">Email: </span>{personalInfo.email}</li>
          )}
          {personalInfo.phone && (
            <li><span className="sr-only">Phone: </span>{personalInfo.phone}</li>
          )}
          {personalInfo.location && (
            <li><span className="sr-only">Location: </span>{personalInfo.location}</li>
          )}
          {personalInfo.linkedin && (
            <li><span className="sr-only">LinkedIn: </span>{personalInfo.linkedin}</li>
          )}
          {personalInfo.github && (
            <li><span className="sr-only">GitHub: </span>{personalInfo.github}</li>
          )}
          {personalInfo.website && (
            <li><span className="sr-only">Website: </span>{personalInfo.website}</li>
          )}
          {personalInfo.drivingLicense && (
            <li>{i18n[lang].drivingLicense}: {personalInfo.drivingLicense}</li>
          )}
        </ul>
      </header>

      {/* Objective */}
      {personalInfo.objective && (
        <section className="mb-5 text-center">
          <p className="text-gray-800 font-medium italic">"{personalInfo.objective}"</p>
        </section>
      )}

      {/* Summary */}
      {summary && (
        <section className="mb-5">
          <h2 className={accentHeader} style={accentHeaderStyle}>
            {i18n[lang].profile}
          </h2>
          <p className="text-gray-700 leading-relaxed whitespace-pre-wrap">{summary}</p>
        </section>
      )}

      {/* Experience */}
      {experience.length > 0 && (
        <section className="mb-5">
          <h2 className="text-sm font-bold uppercase tracking-wider pb-1 mb-3" style={accentHeaderStyle}>
            {i18n[lang].experience}
          </h2>
          <div className="space-y-4">
            {experience.map((exp) => (
              <div key={exp.id}>
                <div className="flex items-start justify-between">
                  <div>
                    <h3 className="font-bold text-gray-900">{exp.position}</h3>
                    <p className="text-gray-600 font-medium">{exp.company}</p>
                  </div>
                  <span className="text-gray-600 min-w-[130px] text-right">
                    {formatDateNumeric(exp.startDate)}
                    {(exp.startDate || exp.endDate || exp.current) && ' — '}
                    {exp.current ? i18n[lang].present : formatDateNumeric(exp.endDate)}
                  </span>
                </div>
                {exp.description && (
                  <p className="mt-1.5 text-gray-700 leading-relaxed whitespace-pre-wrap">{exp.description}</p>
                )}
              </div>
            ))}
          </div>
        </section>
      )}

      {/* Education */}
      {education.length > 0 && (
        <section className="mb-5">
          <h2 className="text-sm font-bold uppercase tracking-wider text-gray-800 border-b border-gray-300 pb-1 mb-3">
            {i18n[lang].education}
          </h2>
          <div className="space-y-3">
            {education.map((edu) => (
              <div key={edu.id}>
                <div className="flex items-start justify-between">
                  <div>
                    <h3 className="font-bold text-gray-900">{edu.degree}</h3>
                    {edu.field && <p className="text-gray-700 text-sm font-medium">{edu.field}</p>}
                    <p className="text-gray-600 font-medium">{edu.institution}</p>
                  </div>
                  <span className="text-gray-600 min-w-[130px] text-right">
                    {formatDateNumeric(edu.startDate)}
                    {(edu.startDate || edu.endDate) && ' — '}
                    {edu.endDate ? formatDateNumeric(edu.endDate) : i18n[lang].present}
                  </span>
                </div>
                {edu.description && (
                  <p className="mt-1 text-gray-700 leading-relaxed whitespace-pre-wrap">{edu.description}</p>
                )}
              </div>
            ))}
          </div>
        </section>
      )}

      {/* Certifications */}
      {certifications.length > 0 && (
        <section className="mb-5">
          <h2 className="text-sm font-bold uppercase tracking-wider text-gray-800 border-b border-gray-300 pb-1 mb-3">
            {i18n[lang].certifications}
          </h2>
          <div className="space-y-3">
            {certifications.map((cert) => (
              <div key={cert.id}>
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
                    <p className="text-gray-600 font-medium">{cert.issuer}</p>
                  </div>
                  <span className="text-gray-600 min-w-[130px] text-right">
                    {formatDateNumeric(cert.date)}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </section>
      )}

      {/* Volunteering */}
      {volunteering.length > 0 && (
        <section className="mb-5">
          <h2 className="text-sm font-bold uppercase tracking-wider pb-1 mb-3" style={accentHeaderStyle}>
            {i18n[lang].volunteering}
          </h2>
          <div className="space-y-4">
            {volunteering.map((v) => (
              <div key={v.id}>
                <div className="flex items-start justify-between">
                  <div>
                    <h3 className="font-bold text-gray-900">{v.role}</h3>
                    <p className="text-gray-600 font-medium">{v.organization}</p>
                  </div>
                  <span className="text-gray-600 min-w-[130px] text-right">
                    {formatDateNumeric(v.startDate)}
                    {(v.startDate || v.endDate || v.current) && ' — '}
                    {v.current ? i18n[lang].present : formatDateNumeric(v.endDate)}
                  </span>
                </div>
                {v.description && (
                  <p className="mt-1.5 text-gray-700 leading-relaxed whitespace-pre-wrap">{v.description}</p>
                )}
              </div>
            ))}
          </div>
        </section>
      )}

      {/* Technical Skills */}
      {technicalSkills.length > 0 && (
        <section className="mb-4">
          <h2 className={accentHeader} style={accentHeaderStyle}>
            {i18n[lang].technicalSkills}
          </h2>
          <ul className="flex flex-wrap gap-x-3 gap-y-1 text-gray-700 list-none">
            {technicalSkills.map((s, i) => (
              <li key={typeof s === 'string' ? `s-${i}` : s.id}>
                {typeof s === 'string' ? s : s.name}
              </li>
            ))}
          </ul>
        </section>
      )}

      {/* Soft Skills */}
      {softSkills.length > 0 && (
        <section className="mb-5">
          <h2 className={accentHeader} style={accentHeaderStyle}>
            {i18n[lang].softSkills}
          </h2>
          <ul className="flex flex-wrap gap-x-3 gap-y-1 text-gray-700 list-none">
            {softSkills.map((s, i) => (
              <li key={`soft-${i}`}>
                {typeof s === 'string' ? s : (s as { name: string }).name}
              </li>
            ))}
          </ul>
        </section>
      )}

      {/* Languages */}
      {languages.length > 0 && (
        <section className="mb-5">
          <h2 className={accentHeader} style={accentHeaderStyle}>
            {i18n[lang].languages}
          </h2>
          <ul className="flex flex-wrap gap-x-3 gap-y-1 text-gray-700 list-none">
            {languages.map((l, i) => {
              const display = typeof l === 'string'
                ? l
                : (() => {
                    const o = l as { name: string; level?: string };
                    return `${o.name}${o.level ? ` (${o.level})` : ''}`;
                  })();
              return <li key={`lang-${i}`}>{display}</li>;
            })}
          </ul>
        </section>
      )}

      {/* Interests */}
      {interests.length > 0 && (
        <section className="mb-5">
          <h2 className="text-sm font-bold uppercase tracking-wider pb-1 mb-3" style={accentHeaderStyle}>
            {i18n[lang].interests}
          </h2>
          <div className="space-y-2">
            {interests.map((it, i) => {
              const o = typeof it === 'string' ? { id: String(i), name: it, description: '' } : it;
              return (
                <div key={o.id}>
                  <h3 className="font-bold text-gray-900">{o.name}</h3>
                  {o.description && (
                    <p className="text-gray-700 leading-relaxed whitespace-pre-wrap">{o.description}</p>
                  )}
                </div>
              );
            })}
          </div>
        </section>
      )}
    </article>
  );
}
