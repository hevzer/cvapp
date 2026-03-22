'use client';

import { useResumeStore } from '@/store/useResumeStore';
import { i18n, getSafeLanguage } from '@/lib/i18n';

function formatDate(date: string, lang: string) {
  if (!date) return '';
  const [year, month] = date.split('-');
  if (!month) return year;
  try {
    const d = new Date(parseInt(year), parseInt(month) - 1);
    return new Intl.DateTimeFormat(lang, { month: 'short', year: 'numeric' }).format(d);
  } catch (e) {
    return `${month}/${year}`;
  }
}

export default function ATSMinimalist() {
  const resume = useResumeStore((s) => s.resumeData);
  const personalInfo = resume.personalInfo || ({} as any);
  const summary = resume.summary || '';
  const experience = resume.experience || [];
  const education = resume.education || [];
  const certifications = resume.certifications || [];
  const technicalSkills = resume.technicalSkills || [];
  const softSkills = resume.softSkills || [];
  const languages = resume.languages || [];
  const hiddenKeywords = resume.hiddenKeywords || [];
  const cvLanguage = resume.cvLanguage;

  const lang = getSafeLanguage(cvLanguage);

  const hasContent =
    personalInfo.fullName ||
    summary ||
    experience.length > 0 ||
    education.length > 0 ||
    certifications.length > 0 ||
    technicalSkills.length > 0 ||
    softSkills.length > 0 ||
    languages.length > 0;

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
    <div className="bg-white text-gray-900 p-8 max-w-[210mm] mx-auto text-[13px] leading-relaxed min-h-[297mm] max-h-[297mm] overflow-hidden">
      {/* Header */}
      <div className="text-center pb-4 mb-5" style={{ borderBottom: '2px solid var(--accent, #1e293b)' }}>
        {personalInfo.photoUrl && (
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
        <div className="flex flex-wrap items-center justify-center gap-x-4 gap-y-1 mt-3 text-xs text-gray-600">
          {personalInfo.email && <span>{personalInfo.email}</span>}
          {personalInfo.phone && <span>{personalInfo.phone}</span>}
          {personalInfo.location && <span>{personalInfo.location}</span>}
          {personalInfo.linkedin && <span>{personalInfo.linkedin}</span>}
          {personalInfo.github && <span>{personalInfo.github}</span>}
          {personalInfo.website && <span>{personalInfo.website}</span>}
          {personalInfo.drivingLicense && (
            <span>{i18n[lang].drivingLicense}: {personalInfo.drivingLicense}</span>
          )}
        </div>
      </div>

      {/* Objective */}
      {personalInfo.objective && (
        <div className="mb-5 text-center">
          <p className="text-gray-800 font-medium italic">"{personalInfo.objective}"</p>
        </div>
      )}

      {/* Summary */}
      {summary && (
        <div className="mb-5">
          <h2 className="text-sm font-bold uppercase tracking-wider pb-1 mb-2" style={{ color: 'var(--accent, #1e293b)', borderBottom: '1px solid var(--accent, #d1d5db)' }}>
            {i18n[lang].profile}
          </h2>
          <p className="text-gray-700 leading-relaxed whitespace-pre-wrap">{summary}</p>
        </div>
      )}

      {/* Experience */}
      {experience.length > 0 && (
        <div className="mb-5">
          <h2 className="text-sm font-bold uppercase tracking-wider pb-1 mb-3" style={{ color: 'var(--accent, #1e293b)', borderBottom: '1px solid var(--accent, #d1d5db)' }}>
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
                    {formatDate(exp.startDate, lang)}
                    {(exp.startDate || exp.endDate || exp.current) && ' — '}
                    {exp.current ? i18n[lang].present : formatDate(exp.endDate, lang)}
                  </span>
                </div>
                {exp.description && (
                  <p className="mt-1.5 text-gray-700 leading-relaxed whitespace-pre-wrap">{exp.description}</p>
                )}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Education */}
      {education.length > 0 && (
        <div className="mb-5">
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
                    {formatDate(edu.startDate, lang)}
                    {(edu.startDate || edu.endDate) && ' — '}
                    {edu.endDate ? formatDate(edu.endDate, lang) : i18n[lang].present}
                  </span>
                </div>
                {edu.description && (
                  <p className="mt-1 text-gray-700 leading-relaxed whitespace-pre-wrap">{edu.description}</p>
                )}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Certifications */}
      {certifications.length > 0 && (
        <div className="mb-5">
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
                          <i className="bi bi-box-arrow-up-right print:hidden"></i>
                          <span className="hidden print:inline"> ({cert.url})</span>
                        </a>
                      )}
                    </h3>
                    <p className="text-gray-600 font-medium">{cert.issuer}</p>
                  </div>
                  <span className="text-gray-600 min-w-[130px] text-right">
                    {formatDate(cert.date, lang)}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Technical Skills */}
      {technicalSkills.length > 0 && (
        <div className="mb-4">
          <h2 className="text-sm font-bold uppercase tracking-wider pb-1 mb-2" style={{ color: 'var(--accent, #1e293b)', borderBottom: '1px solid var(--accent, #d1d5db)' }}>
            {i18n[lang].technicalSkills}
          </h2>
          <p className="text-gray-700">{technicalSkills.map(s => typeof s === 'string' ? s : (s as any).name).join(' • ')}</p>
        </div>
      )}

      {/* Soft Skills */}
      {softSkills.length > 0 && (
        <div className="mb-5">
          <h2 className="text-sm font-bold uppercase tracking-wider pb-1 mb-2" style={{ color: 'var(--accent, #1e293b)', borderBottom: '1px solid var(--accent, #d1d5db)' }}>
            {i18n[lang].softSkills}
          </h2>
          <p className="text-gray-700">{softSkills.map(s => typeof s === 'string' ? s : (s as any).name).join(' • ')}</p>
        </div>
      )}

      {/* Languages */}
      {languages.length > 0 && (
        <div className="mb-5">
          <h2 className="text-sm font-bold uppercase tracking-wider pb-1 mb-2" style={{ color: 'var(--accent, #1e293b)', borderBottom: '1px solid var(--accent, #d1d5db)' }}>
            {i18n[lang].languages}
          </h2>
          <p className="text-gray-700">{languages.map(l => typeof l === 'string' ? l : `${(l as any).name}${ (l as any).level ? ` (${(l as any).level})` : '' }`).join(' • ')}</p>
        </div>
      )}

      {/* Hidden ATS Keywords */}
      {hiddenKeywords.length > 0 && (
        <div
          aria-hidden="true"
          style={{
            fontSize: 0,
            lineHeight: 0,
            color: 'transparent',
            height: 0,
            overflow: 'hidden',
            position: 'absolute',
            width: 0,
          }}
        >
          {hiddenKeywords.join(' ')}
        </div>
      )}
    </div>
  );
}
