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

export default function TimelineTwoColumn() {
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
    <div className="bg-white text-gray-900 w-full max-w-[210mm] mx-auto text-[11.5px] leading-[1.5] min-h-[297mm] max-h-[297mm] overflow-hidden flex shadow-2xl print:shadow-none print:m-0">
      {/* Left Sidebar */}
      <div className="w-[33%] text-white p-6 flex flex-col self-stretch" style={{ backgroundColor: 'var(--sidebar, #1e293b)' }}>
        {/* Photo */}
        {personalInfo.photoUrl && (
          <div className="flex justify-center mb-4">
            <img
              src={personalInfo.photoUrl}
              alt={personalInfo.fullName}
              className="w-24 h-24 rounded-full object-cover" style={{ boxShadow: '0 0 0 4px var(--accent, #6366f1)33' }}
            />
          </div>
        )}
        
        {/* Name & Title */}
        <div className="mb-5 text-center">
          {personalInfo.fullName && (
            <h1 className="text-xl font-bold leading-tight tracking-tight mb-1">{personalInfo.fullName}</h1>
          )}
          {personalInfo.title && (
            <p className="text-xs font-medium uppercase tracking-wider" style={{ color: 'var(--accent, #6366f1)' }}>{personalInfo.title}</p>
          )}
        </div>

        {/* Contact Info */}
        {(personalInfo.email || personalInfo.phone || personalInfo.location || personalInfo.linkedin || personalInfo.website || personalInfo.drivingLicense) && (
          <div className="mb-5">
            <h2 className="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
              {i18n[lang].contact}
            </h2>
            <div className="space-y-2">
              {personalInfo.email && (
                <div className="flex items-start gap-2.5">
                  <i className="bi bi-envelope-fill mt-[1px] text-[11px]" style={{ color: 'var(--accent, #6366f1)' }}></i>
                  <span className="text-[11px] text-slate-200 break-all leading-tight">{personalInfo.email}</span>
                </div>
              )}
              {personalInfo.phone && (
                <div className="flex items-start gap-2.5">
                  <i className="bi bi-telephone-fill mt-[1px] text-[11px]" style={{ color: 'var(--accent, #6366f1)' }}></i>
                  <span className="text-[11px] text-slate-200 leading-tight">{personalInfo.phone}</span>
                </div>
              )}
              {personalInfo.location && (
                <div className="flex items-start gap-2.5">
                  <i className="bi bi-geo-alt-fill mt-[1px] text-[11px]" style={{ color: 'var(--accent, #6366f1)' }}></i>
                  <span className="text-[11px] text-slate-200 leading-tight">{personalInfo.location}</span>
                </div>
              )}
              {personalInfo.linkedin && (
                <div className="flex items-start gap-2.5">
                  <i className="bi bi-linkedin mt-[1px] text-[11px]" style={{ color: 'var(--accent, #6366f1)' }}></i>
                  <span className="text-[11px] text-slate-200 break-all leading-tight">{personalInfo.linkedin}</span>
                </div>
              )}
              {personalInfo.github && (
                <div className="flex items-start gap-2.5">
                  <i className="bi bi-github mt-[1px] text-[11px]" style={{ color: 'var(--accent, #6366f1)' }}></i>
                  <span className="text-[11px] text-slate-200 break-all leading-tight">{personalInfo.github}</span>
                </div>
              )}
              {personalInfo.website && (
                <div className="flex items-start gap-2.5">
                  <i className="bi bi-globe mt-[1px] text-[11px]" style={{ color: 'var(--accent, #6366f1)' }}></i>
                  <span className="text-[11px] text-slate-200 break-all leading-tight">{personalInfo.website}</span>
                </div>
              )}
              {personalInfo.drivingLicense && (
                <div className="flex items-start gap-2.5">
                  <i className="bi bi-car-front-fill mt-[1px] text-[11px]" style={{ color: 'var(--accent, #6366f1)' }}></i>
                  <span className="text-[11px] text-slate-200 leading-tight">{i18n[lang].drivingLicense}: {personalInfo.drivingLicense}</span>
                </div>
              )}
            </div>
          </div>
        )}

        {/* Summary (Profile) */}
        {summary && (
          <div className="mb-5">
            <h2 className="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
              {i18n[lang].profile}
            </h2>
            <p className="text-slate-300 text-[11px] leading-relaxed whitespace-pre-wrap">
              {summary}
            </p>
          </div>
        )}

        {/* Technical Skills */}
        {technicalSkills.length > 0 && (
          <div className="mb-5">
            <h2 className="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
              {i18n[lang].technicalSkills}
            </h2>
            <div className="flex flex-wrap gap-1">
              {technicalSkills.map((skill, i) => (
                <span
                  key={i}
                  className="px-1.5 py-0.5 bg-slate-800 border border-slate-700 text-slate-200 text-[10px] rounded font-medium"
                >
                  {typeof skill === 'string' ? skill : (skill as any).name}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* Soft Skills */}
        {softSkills.length > 0 && (
          <div className="mb-5">
            <h2 className="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
              {i18n[lang].softSkills}
            </h2>
            <ul className="space-y-1">
              {softSkills.map((skill, i) => (
                <li key={i} className="text-[11px] text-slate-300 flex items-center gap-1.5">
                  <i className="bi bi-check-circle-fill text-[9px]" style={{ color: 'var(--accent, #6366f1)' }}></i>
                  {typeof skill === 'string' ? skill : (skill as any).name}
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* Languages */}
        {languages.length > 0 && (
          <div className="mb-5">
            <h2 className="text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-2.5 border-b border-slate-700 pb-1">
              {i18n[lang].languages}
            </h2>
            <div className="flex flex-col gap-1">
              {languages.map((l, i) => (
                <div key={i} className="flex items-center gap-1.5 text-slate-300">
                  <i className="bi bi-chat-left-text-fill text-[9px]" style={{ color: 'var(--accent, #6366f1)' }}></i>
                  <span className="text-[11px] font-medium">
                    {typeof l === 'string' ? l : `${(l as any).name}${ (l as any).level ? ` (${(l as any).level})` : '' }`}
                  </span>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Right Main Content */}
      <div className="w-[67%] p-6 bg-slate-50 h-[297mm]">
        
        {/* Objective */}
        {personalInfo.objective && (
          <div className="mb-6">
            <p className="text-slate-700 italic text-[11px] leading-relaxed pl-3 py-0.5 border-l-2" style={{ borderColor: 'var(--accent, #6366f1)' }}>
              "{personalInfo.objective}"
            </p>
          </div>
        )}

        {/* Experience Timeline */}
        {experience.length > 0 && (
          <div className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-900 mb-3.5 flex items-center gap-2">
              <div className="w-5 h-[2px]" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
              {i18n[lang].experience}
            </h2>
            <div className="relative ml-[5px] space-y-4" style={{ borderLeft: '2px solid var(--accent-light, #c7d2fe)' }}>
              {experience.map((exp) => (
                <div key={exp.id} className="relative pl-5">
                  {/* Timeline Dot */}
                  <div className="absolute -left-[7px] top-1.5 w-3 h-3 rounded-full ring-4 ring-slate-50" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
                  
                  <div className="flex flex-col gap-0.5 mb-1">
                    <h3 className="font-bold text-slate-900 text-[12.5px]">{exp.position}</h3>
                    <div className="flex justify-between items-center pr-2">
                      <span className="font-semibold text-[11.5px]" style={{ color: 'var(--accent, #6366f1)' }}>{exp.company}</span>
                      <span className="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                        {formatDate(exp.startDate, lang)}
                        {(exp.startDate || exp.endDate || exp.current) && ' — '}
                        {exp.current ? i18n[lang].present : formatDate(exp.endDate, lang)}
                      </span>
                    </div>
                  </div>
                  {exp.description && (
                    <p className="text-slate-600 text-[11px] leading-snug mt-1 whitespace-pre-wrap">{exp.description}</p>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Education Timeline */}
        {education.length > 0 && (
          <div className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-900 mb-3.5 flex items-center gap-2">
              <div className="w-5 h-[2px]" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
              {i18n[lang].education}
            </h2>
            <div className="relative ml-[5px] space-y-4" style={{ borderLeft: '2px solid var(--accent-light, #c7d2fe)' }}>
              {education.map((edu) => (
                <div key={edu.id} className="relative pl-5">
                  <div className="absolute -left-[7px] top-1.5 w-3 h-3 rounded-full ring-4 ring-slate-50" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
                  <div className="flex flex-col gap-0.5 mb-1">
                    <h3 className="font-bold text-slate-900 text-[12.5px]">{edu.degree}</h3>
                    {edu.field && <p className="text-[11.5px] font-medium text-slate-700">{edu.field}</p>}
                    <div className="flex justify-between items-center pr-2">
                      <span className="font-semibold text-[11.5px]" style={{ color: 'var(--accent, #6366f1)' }}>{edu.institution}</span>
                      <span className="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                        {formatDate(edu.startDate, lang)}
                        {(edu.startDate || edu.endDate) && ' — '}
                        {edu.endDate ? formatDate(edu.endDate, lang) : i18n[lang].present}
                      </span>
                    </div>
                  </div>
                  {edu.description && (
                    <p className="text-slate-600 text-[11px] leading-snug mt-1 whitespace-pre-wrap">{edu.description}</p>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Certifications Timeline */}
        {certifications.length > 0 && (
          <div className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-900 mb-3.5 flex items-center gap-2">
              <div className="w-5 h-[2px]" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
              {i18n[lang].certifications}
            </h2>
            <div className="relative ml-[5px] space-y-4" style={{ borderLeft: '2px solid var(--accent-light, #c7d2fe)' }}>
              {certifications.map((cert) => (
                <div key={cert.id} className="relative pl-5">
                  <div className="absolute -left-[7px] top-1.5 w-3 h-3 rounded-full ring-4 ring-slate-50" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
                  <div className="flex flex-col gap-0.5 mb-1">
                    <h3 className="font-bold text-slate-900 text-[12.5px]">
                      {cert.name}
                      {cert.url && (
                        <a href={cert.url} target="_blank" rel="noopener noreferrer" className="ml-2 text-cyan-600 hover:text-cyan-800 break-all text-xs print:text-gray-900 print:no-underline">
                          <i className="bi bi-box-arrow-up-right print:hidden"></i>
                          <span className="hidden print:inline"> ({cert.url})</span>
                        </a>
                      )}
                    </h3>
                    <div className="flex justify-between items-center pr-2">
                      <span className="font-semibold text-[11.5px]" style={{ color: 'var(--accent, #6366f1)' }}>{cert.issuer}</span>
                      <span className="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                        {formatDate(cert.date, lang)}
                      </span>
                    </div>
                  </div>
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
