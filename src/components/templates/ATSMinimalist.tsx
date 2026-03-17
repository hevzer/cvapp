'use client';

import { useResumeStore } from '@/store/useResumeStore';
import { i18n, LanguageCode } from '@/lib/i18n';

function formatDate(date: string) {
  if (!date) return '';
  const [year, month] = date.split('-');
  const monthNames = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
  return `${monthNames[parseInt(month) - 1]} ${year}`;
}

export default function ATSMinimalist() {
  const {
    personalInfo,
    summary,
    experience,
    education,
    technicalSkills,
    softSkills,
    languages,
    hiddenKeywords,
    cvLanguage,
  } = useResumeStore((s) => s.resumeData);
  const lang = (i18n[cvLanguage as LanguageCode] ? cvLanguage : 'en') as LanguageCode;

  const hasContent =
    personalInfo.fullName ||
    summary ||
    experience.length > 0 ||
    education.length > 0 ||
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
    <div className="bg-white text-gray-900 p-8 max-w-[210mm] mx-auto font-['Inter',sans-serif] text-[13px] leading-relaxed min-h-[297mm]">
      {/* Header */}
      <div className="text-center border-b-2 border-gray-800 pb-4 mb-5">
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
          <h2 className="text-sm font-bold uppercase tracking-wider text-gray-800 border-b border-gray-300 pb-1 mb-2">
            {i18n[lang].profile}
          </h2>
          <p className="text-gray-700 leading-relaxed">{summary}</p>
        </div>
      )}

      {/* Experience */}
      {experience.length > 0 && (
        <div className="mb-5">
          <h2 className="text-sm font-bold uppercase tracking-wider text-gray-800 border-b border-gray-300 pb-1 mb-3">
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
                  <span className="text-xs text-gray-500 whitespace-nowrap ml-4">
                    {formatDate(exp.startDate)}
                    {(exp.startDate || exp.endDate || exp.current) && ' — '}
                    {exp.current ? i18n[lang].present : formatDate(exp.endDate)}
                  </span>
                </div>
                {exp.description && (
                  <p className="mt-1.5 text-gray-700 leading-relaxed">{exp.description}</p>
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
                    <h3 className="font-bold text-gray-900">
                      {edu.degree}
                      {edu.field && ` in ${edu.field}`}
                    </h3>
                    <p className="text-gray-600 font-medium">{edu.institution}</p>
                  </div>
                  <span className="text-xs text-gray-500 whitespace-nowrap ml-4">
                    {formatDate(edu.startDate)}
                    {(edu.startDate || edu.endDate) && ' — '}
                    {formatDate(edu.endDate)}
                  </span>
                </div>
                {edu.description && (
                  <p className="mt-1 text-gray-700 leading-relaxed">{edu.description}</p>
                )}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Technical Skills */}
      {technicalSkills.length > 0 && (
        <div className="mb-4">
          <h2 className="text-sm font-bold uppercase tracking-wider text-gray-800 border-b border-gray-300 pb-1 mb-2">
            {i18n[lang].technicalSkills}
          </h2>
          <p className="text-gray-700">{technicalSkills.join(' • ')}</p>
        </div>
      )}

      {/* Soft Skills */}
      {softSkills.length > 0 && (
        <div className="mb-4">
          <h2 className="text-sm font-bold uppercase tracking-wider text-gray-800 border-b border-gray-300 pb-1 mb-2">
            {i18n[lang].softSkills}
          </h2>
          <p className="text-gray-700">{softSkills.join(' • ')}</p>
        </div>
      )}

      {/* Languages */}
      {languages.length > 0 && (
        <div className="mb-5">
          <h2 className="text-sm font-bold uppercase tracking-wider text-gray-800 border-b border-gray-300 pb-1 mb-2">
            {i18n[lang].languages}
          </h2>
          <p className="text-gray-700">{languages.join(' • ')}</p>
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
