'use client';

import { useResumeStore } from '@/store/useResumeStore';
import { i18n, getSafeLanguage } from '@/lib/i18n';
import 'bootstrap-icons/font/bootstrap-icons.css';

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

export default function ModernCreative() {
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
    <div className="bg-white text-gray-900 max-w-[210mm] mx-auto text-[13px] leading-relaxed min-h-[297mm] flex">
      {/* Left Sidebar */}
      <div className="w-[35%] text-white p-6 flex flex-col" style={{ backgroundColor: 'var(--sidebar, #1e293b)' }}>
        {/* Photo */}
        {personalInfo.photoUrl && (
          <div className="flex justify-center mb-5">
            <img
              src={personalInfo.photoUrl}
              alt={personalInfo.fullName}
              className="w-24 h-24 rounded-full object-cover ring-3 ring-slate-600"
            />
          </div>
        )}
        {/* Name & Title */}
        <div className="mb-6">
          {personalInfo.fullName && (
            <h1 className="text-xl font-bold leading-tight">{personalInfo.fullName}</h1>
          )}
          {personalInfo.title && (
            <p className="text-sm text-slate-300 mt-1 font-medium">{personalInfo.title}</p>
          )}
        </div>

        {/* Contact Info */}
        {(personalInfo.email || personalInfo.phone || personalInfo.location || personalInfo.linkedin || personalInfo.website || personalInfo.drivingLicense) && (
          <div className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
              {i18n[lang].contact}
            </h2>
            <div className="space-y-2">
              {personalInfo.email && (
                <div className="flex items-start gap-2">
                  <i className="bi bi-envelope text-slate-400 mt-0.5 text-xs"></i>
                  <span className="text-sm text-slate-200 break-all">{personalInfo.email}</span>
                </div>
              )}
              {personalInfo.phone && (
                <div className="flex items-start gap-2">
                  <i className="bi bi-telephone text-slate-400 mt-0.5 text-xs"></i>
                  <span className="text-sm text-slate-200">{personalInfo.phone}</span>
                </div>
              )}
              {personalInfo.location && (
                <div className="flex items-start gap-2">
                  <i className="bi bi-geo-alt text-slate-400 mt-0.5 text-xs"></i>
                  <span className="text-sm text-slate-200">{personalInfo.location}</span>
                </div>
              )}
              {personalInfo.linkedin && (
                <div className="flex items-start gap-2">
                  <i className="bi bi-linkedin text-slate-400 mt-0.5 text-xs"></i>
                  <span className="text-sm text-slate-200 break-all">{personalInfo.linkedin}</span>
                </div>
              )}
              {personalInfo.github && (
                <div className="flex items-start gap-2">
                  <i className="bi bi-github text-slate-400 mt-0.5 text-xs"></i>
                  <span className="text-sm text-slate-200 break-all">{personalInfo.github}</span>
                </div>
              )}
              {personalInfo.website && (
                <div className="flex items-start gap-2">
                  <i className="bi bi-globe text-slate-400 mt-0.5 text-xs"></i>
                  <span className="text-sm text-slate-200 break-all">{personalInfo.website}</span>
                </div>
              )}
              {personalInfo.drivingLicense && (
                <div className="flex items-start gap-2">
                  <i className="bi bi-car-front text-slate-400 mt-0.5 text-xs"></i>
                  <span className="text-sm text-slate-200">{i18n[lang].drivingLicense}: {personalInfo.drivingLicense}</span>
                </div>
              )}
            </div>
          </div>
        )}

        {/* Technical Skills */}
        {technicalSkills.length > 0 && (
          <div className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
              {i18n[lang].technicalSkills}
            </h2>
            <div className="flex flex-wrap gap-1.5">
              {technicalSkills.map((skill, i) => (
                <span
                  key={i}
                  className="px-2 py-0.5 bg-slate-700 text-slate-200 text-[10px] rounded font-medium"
                >
                  {typeof skill === 'string' ? skill : (skill as any).name}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* Soft Skills */}
        {softSkills.length > 0 && (
          <div className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
              {i18n[lang].softSkills}
            </h2>
            <ul className="space-y-1.5">
              {softSkills.map((skill, i) => (
                <li key={i} className="text-xs text-slate-300 flex items-center gap-2">
                  <i className="bi bi-check2 text-xs" style={{ color: 'var(--accent, #6366f1)' }}></i>
                  {typeof skill === 'string' ? skill : (skill as any).name}
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* Languages */}
        {languages.length > 0 && (
          <div className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
              {i18n[lang].languages}
            </h2>
            <div className="flex flex-wrap gap-1.5">
              {languages.map((l, i) => (
                <span
                  key={i}
                  className="px-2 py-0.5 bg-slate-700 text-slate-200 text-xs rounded font-medium"
                >
                  {typeof l === 'string' ? l : `${(l as any).name}${ (l as any).level ? ` (${(l as any).level})` : '' }`}
                </span>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Right Main Content */}
      <div className="w-[65%] p-8">
        {/* Objective */}
        {personalInfo.objective && (
          <div className="mb-6">
            <p className="text-slate-600 font-medium italic text-sm leading-relaxed pl-4 py-1" style={{ borderLeft: '4px solid var(--accent, #6366f1)' }}>
              "{personalInfo.objective}"
            </p>
          </div>
        )}
        {/* Summary */}
        {summary && (
          <div className="mb-6">
            <h2 className="text-sm font-bold uppercase tracking-wider text-slate-800 pb-1.5 mb-3" style={{ borderBottom: '2px solid var(--accent, #1e293b)' }}>
              {i18n[lang].profile}
            </h2>
            <p className="text-gray-700 leading-relaxed">{summary}</p>
          </div>
        )}

        {/* Experience */}
        {experience.length > 0 && (
          <div className="mb-6">
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
                      {formatDate(exp.startDate, lang)}
                      {(exp.startDate || exp.endDate || exp.current) && ' — '}
                      {exp.current ? i18n[lang].present : formatDate(exp.endDate, lang)}
                    </span>
                  </div>
                  {exp.description && (
                    <p className="mt-2 text-gray-700 leading-relaxed">{exp.description}</p>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Education */}
        {education.length > 0 && (
          <div className="mb-6">
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
                      {formatDate(edu.startDate, lang)}
                      {(edu.startDate || edu.endDate) && ' — '}
                      {edu.endDate ? formatDate(edu.endDate, lang) : i18n[lang].present}
                    </span>
                  </div>
                  {edu.description && (
                    <p className="mt-2 text-gray-700 leading-relaxed">{edu.description}</p>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

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
