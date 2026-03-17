'use client';

import { useResumeStore } from '@/store/useResumeStore';
import { i18n, LanguageCode } from '@/lib/i18n';
import 'bootstrap-icons/font/bootstrap-icons.css';

function formatDate(date: string) {
  if (!date) return '';
  const [year, month] = date.split('-');
  const monthNames = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
  return `${monthNames[parseInt(month) - 1]} ${year}`;
}

export default function TimelineTwoColumn() {
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
    <div className="bg-white text-gray-900 w-full max-w-[210mm] mx-auto font-['Inter',sans-serif] text-[11.5px] leading-[1.5] min-h-[297mm] max-h-[297mm] overflow-hidden flex shadow-2xl print:shadow-none print:m-0">
      {/* Left Sidebar */}
      <div className="w-[33%] bg-[#1e293b] text-white p-6 flex flex-col self-stretch">
        {/* Photo */}
        {personalInfo.photoUrl && (
          <div className="flex justify-center mb-4">
            <img
              src={personalInfo.photoUrl}
              alt={personalInfo.fullName}
              className="w-24 h-24 rounded-full object-cover ring-4 ring-indigo-500/30"
            />
          </div>
        )}
        
        {/* Name & Title */}
        <div className="mb-5 text-center">
          {personalInfo.fullName && (
            <h1 className="text-xl font-bold leading-tight tracking-tight mb-1">{personalInfo.fullName}</h1>
          )}
          {personalInfo.title && (
            <p className="text-xs text-indigo-300 font-medium uppercase tracking-wider">{personalInfo.title}</p>
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
                  <i className="bi bi-envelope-fill text-indigo-400 mt-[1px] text-[11px]"></i>
                  <span className="text-[11px] text-slate-200 break-all leading-tight">{personalInfo.email}</span>
                </div>
              )}
              {personalInfo.phone && (
                <div className="flex items-start gap-2.5">
                  <i className="bi bi-telephone-fill text-indigo-400 mt-[1px] text-[11px]"></i>
                  <span className="text-[11px] text-slate-200 leading-tight">{personalInfo.phone}</span>
                </div>
              )}
              {personalInfo.location && (
                <div className="flex items-start gap-2.5">
                  <i className="bi bi-geo-alt-fill text-indigo-400 mt-[1px] text-[11px]"></i>
                  <span className="text-[11px] text-slate-200 leading-tight">{personalInfo.location}</span>
                </div>
              )}
              {personalInfo.linkedin && (
                <div className="flex items-start gap-2.5">
                  <i className="bi bi-linkedin text-indigo-400 mt-[1px] text-[11px]"></i>
                  <span className="text-[11px] text-slate-200 break-all leading-tight">{personalInfo.linkedin}</span>
                </div>
              )}
              {personalInfo.github && (
                <div className="flex items-start gap-2.5">
                  <i className="bi bi-github text-indigo-400 mt-[1px] text-[11px]"></i>
                  <span className="text-[11px] text-slate-200 break-all leading-tight">{personalInfo.github}</span>
                </div>
              )}
              {personalInfo.website && (
                <div className="flex items-start gap-2.5">
                  <i className="bi bi-globe text-indigo-400 mt-[1px] text-[11px]"></i>
                  <span className="text-[11px] text-slate-200 break-all leading-tight">{personalInfo.website}</span>
                </div>
              )}
              {personalInfo.drivingLicense && (
                <div className="flex items-start gap-2.5">
                  <i className="bi bi-car-front-fill text-indigo-400 mt-[1px] text-[11px]"></i>
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
            <p className="text-slate-300 text-[11px] leading-relaxed">
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
                  {skill}
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
                  <i className="bi bi-check-circle-fill text-indigo-400 text-[9px]"></i>
                  {skill}
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
              {languages.map((langItem, i) => (
                <div key={i} className="flex items-center gap-1.5 text-slate-300">
                  <i className="bi bi-chat-left-text-fill text-indigo-400 text-[9px]"></i>
                  <span className="text-[11px] font-medium">{langItem}</span>
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
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-900 mb-2.5 flex items-center gap-2">
              <div className="w-5 h-[2px] bg-indigo-500"></div>
              Objective
            </h2>
            <p className="text-slate-700 italic text-[11px] leading-relaxed">
              "{personalInfo.objective}"
            </p>
          </div>
        )}

        {/* Experience Timeline */}
        {experience.length > 0 && (
          <div className="mb-6">
            <h2 className="text-xs font-bold uppercase tracking-widest text-slate-900 mb-3.5 flex items-center gap-2">
              <div className="w-5 h-[2px] bg-indigo-500"></div>
              {i18n[lang].experience}
            </h2>
            <div className="relative border-l-2 border-indigo-200 ml-[5px] space-y-4">
              {experience.map((exp) => (
                <div key={exp.id} className="relative pl-5">
                  {/* Timeline Dot */}
                  <div className="absolute -left-[7px] top-1.5 w-3 h-3 rounded-full bg-indigo-500 ring-4 ring-slate-50"></div>
                  
                  <div className="flex flex-col gap-0.5 mb-1">
                    <h3 className="font-bold text-slate-900 text-[12.5px]">{exp.position}</h3>
                    <div className="flex justify-between items-center pr-2">
                      <span className="text-indigo-600 font-semibold text-[11.5px]">{exp.company}</span>
                      <span className="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                        {formatDate(exp.startDate)}
                        {(exp.startDate || exp.endDate || exp.current) && ' — '}
                        {exp.current ? i18n[lang].present : formatDate(exp.endDate)}
                      </span>
                    </div>
                  </div>
                  {exp.description && (
                    <p className="text-slate-600 text-[11px] leading-snug mt-1">{exp.description}</p>
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
              <div className="w-5 h-[2px] bg-indigo-500"></div>
              {i18n[lang].education}
            </h2>
            <div className="relative border-l-2 border-indigo-200 ml-[5px] space-y-4">
              {education.map((edu) => (
                <div key={edu.id} className="relative pl-5">
                  <div className="absolute -left-[7px] top-1.5 w-3 h-3 rounded-full bg-indigo-500 ring-4 ring-slate-50"></div>
                  <div className="flex flex-col gap-0.5 mb-1">
                    <h3 className="font-bold text-slate-900 text-[12.5px]">
                      {edu.degree}
                      {edu.field && ` in ${edu.field}`}
                    </h3>
                    <div className="flex justify-between items-center pr-2">
                      <span className="text-indigo-600 font-semibold text-[11.5px]">{edu.institution}</span>
                      <span className="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                        {formatDate(edu.startDate)}
                        {(edu.startDate || edu.endDate) && ' — '}
                        {formatDate(edu.endDate)}
                      </span>
                    </div>
                  </div>
                  {edu.description && (
                    <p className="text-slate-600 text-[11px] leading-snug mt-1">{edu.description}</p>
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
