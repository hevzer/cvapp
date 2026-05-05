import { useResumeStore } from '@/store/useResumeStore';
import { i18n, getSafeLanguage } from '@/lib/i18n';
import type { PersonalInfo } from '@/types/resume';
import 'bootstrap-icons/font/bootstrap-icons.css';

function formatDate(date: string, lang: string) {
  if (!date) return '';
  const [year, month] = date.split('-');
  if (!month) return year;
  try {
    const d = new Date(parseInt(year), parseInt(month) - 1);
    return new Intl.DateTimeFormat(lang, { month: 'short', year: 'numeric' }).format(d);
  } catch {
    return `${month}/${year}`;
  }
}

export default function TimelineCreative() {
  const resume = useResumeStore((s) => s.resumeData);
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
  const hiddenKeywords = resume.hiddenKeywords || [];
  const cvLanguage = resume.cvLanguage;

  const lang = getSafeLanguage(cvLanguage);

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
    <div className="bg-slate-50 text-slate-900 w-full max-w-[210mm] mx-auto text-[11.5px] leading-[1.5] min-h-[297mm] max-h-[297mm] overflow-hidden print:w-[210mm] print:-m-4">
      {/* Header Section */}
      <header className="bg-white px-6 py-4 border-b border-slate-200">
        <div className="flex items-center gap-4">
          {personalInfo.photoUrl && (
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
            {/* Contact Grid */}
            <div className="grid grid-cols-2 gap-y-1.5 gap-x-4 mt-4 text-sm text-slate-600">
              {personalInfo.email && (
                <div className="flex items-center gap-2">
                  <i className="bi bi-envelope text-slate-400 text-xs text-center w-4"></i>
                  <span>{personalInfo.email}</span>
                </div>
              )}
              {personalInfo.phone && (
                <div className="flex items-center gap-2">
                  <i className="bi bi-telephone text-slate-400 text-xs text-center w-4"></i>
                  <span>{personalInfo.phone}</span>
                </div>
              )}
              {personalInfo.location && (
                <div className="flex items-center gap-2">
                  <i className="bi bi-geo-alt text-slate-400 text-xs text-center w-4"></i>
                  <span>{personalInfo.location}</span>
                </div>
              )}
              {personalInfo.linkedin && (
                <div className="flex items-center gap-2">
                  <i className="bi bi-linkedin text-slate-400 text-xs text-center w-4"></i>
                  <span>{personalInfo.linkedin}</span>
                </div>
              )}
              {personalInfo.github && (
                <div className="flex items-center gap-2">
                  <i className="bi bi-github text-slate-400 text-xs text-center w-4"></i>
                  <span>{personalInfo.github}</span>
                </div>
              )}
              {personalInfo.website && (
                <div className="flex items-center gap-2">
                  <i className="bi bi-globe text-slate-400 text-xs text-center w-4"></i>
                  <span>{personalInfo.website}</span>
                </div>
              )}
              {personalInfo.drivingLicense && (
                <div className="flex items-center gap-2">
                  <i className="bi bi-car-front text-slate-400 text-xs text-center w-4"></i>
                  <span>{i18n[lang].drivingLicense}: {personalInfo.drivingLicense}</span>
                </div>
              )}
            </div>
          </div>
        </div>
      </header>

      {/* Main Two-Column Layout */}
      <div className="flex px-6 py-5 gap-6">
        {/* Left Column (Timeline for Experience/Education) */}
        <div className="w-[62%]">
          {summary && (
            <div className="mb-5">
              <h2 className="text-[11.5px] font-bold text-slate-900 uppercase tracking-widest mb-2.5 flex items-center gap-2">
                <i className="bi bi-person-lines-fill" style={{ color: 'var(--accent, #6366f1)' }}></i>
                {i18n[lang].profile}
              </h2>
              <p className="text-slate-700 leading-[1.5] text-[11px] bg-white p-3 rounded-xl border border-slate-100 shadow-sm shadow-slate-200/50 whitespace-pre-wrap">
                {summary}
              </p>
            </div>
          )}

          {experience.length > 0 && (
            <div className="mb-5">
              <h2 className="text-[11.5px] font-bold text-slate-900 uppercase tracking-widest mb-2.5 flex items-center gap-2">
                <i className="bi bi-briefcase-fill" style={{ color: 'var(--accent, #6366f1)' }}></i>
                {i18n[lang].experience}
              </h2>
              <div className="relative ml-2.5 space-y-4" style={{ borderLeft: '2px solid var(--accent-light, #e0e7ff)' }}>
                {experience.map((exp) => (
                  <div key={exp.id} className="relative pl-5">
                    {/* Timeline Dot */}
                    <div className="absolute -left-[5.5px] top-1.5 w-2 h-2 rounded-full ring-4 ring-slate-50" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
                    
                    <div className="mb-1">
                      <h3 className="text-[12px] font-bold text-slate-900">{exp.position}</h3>
                      <div className="flex justify-between items-center mt-0.5">
                        <span className="font-semibold text-[11px]" style={{ color: 'var(--accent, #6366f1)' }}>{exp.company}</span>
                        <span className="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                          {formatDate(exp.startDate, lang)}
                          {(exp.startDate || exp.endDate || exp.current) && ' — '}
                          {exp.current ? i18n[lang].present : formatDate(exp.endDate, lang)}
                        </span>
                      </div>
                    </div>
                    {exp.description && (
                      <p className="text-slate-600 text-[11px] leading-[1.5] mt-1 whitespace-pre-wrap">{exp.description}</p>
                    )}
                  </div>
                ))}
              </div>
            </div>
          )}

          {volunteering.length > 0 && (
            <div className="mb-5">
              <h2 className="text-[11.5px] font-bold text-slate-900 uppercase tracking-widest mb-2.5 flex items-center gap-2">
                <i className="bi bi-heart-fill" style={{ color: 'var(--accent, #6366f1)' }}></i>
                {i18n[lang].volunteering}
              </h2>
              <div className="relative ml-2.5 space-y-4" style={{ borderLeft: '2px solid var(--accent-light, #e0e7ff)' }}>
                {volunteering.map((v) => (
                  <div key={v.id} className="relative pl-5">
                    <div className="absolute -left-[5.5px] top-1.5 w-2 h-2 rounded-full ring-4 ring-slate-50" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
                    <div className="mb-1">
                      <h3 className="text-[12px] font-bold text-slate-900">{v.role}</h3>
                      <div className="flex justify-between items-center mt-0.5">
                        <span className="font-semibold text-[11px]" style={{ color: 'var(--accent, #6366f1)' }}>{v.organization}</span>
                        <span className="text-[10px] font-medium text-slate-500 bg-white border border-slate-200 px-2 py-0.5 rounded-full shadow-sm">
                          {formatDate(v.startDate, lang)}
                          {(v.startDate || v.endDate || v.current) && ' — '}
                          {v.current ? i18n[lang].present : formatDate(v.endDate, lang)}
                        </span>
                      </div>
                    </div>
                    {v.description && (
                      <p className="text-slate-600 text-[11px] leading-[1.5] mt-1 whitespace-pre-wrap">{v.description}</p>
                    )}
                  </div>
                ))}
              </div>
            </div>
          )}

          {education.length > 0 && (
            <div>
              <h2 className="text-xs font-bold text-slate-900 uppercase tracking-widest mb-2.5 flex items-center gap-2">
                <i className="bi bi-mortarboard-fill" style={{ color: 'var(--accent, #6366f1)' }}></i>
                {i18n[lang].education}
              </h2>
              <div className="relative ml-2.5 space-y-4" style={{ borderLeft: '2px solid var(--accent-light, #e0e7ff)' }}>
                {education.map((edu) => (
                  <div key={edu.id} className="relative pl-5">
                    <div className="absolute -left-[5.5px] top-1.5 w-2 h-2 rounded-full ring-4 ring-slate-50" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
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
                          {formatDate(edu.startDate, lang)}
                          {(edu.startDate || edu.endDate) && ' — '}
                          {edu.endDate ? formatDate(edu.endDate, lang) : i18n[lang].present}
                        </span>
                      </div>
                    </div>
                    {edu.description && (
                      <p className="text-slate-600 text-[11px] leading-[1.5] mt-1.5 whitespace-pre-wrap">{edu.description}</p>
                    )}
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Certifications (Timeline) */}
          {certifications.length > 0 && (
            <div className="relative">
              <div className="flex items-center gap-2 mb-3">
                <div className="w-6 h-6 rounded flex items-center justify-center text-white" style={{ backgroundColor: 'var(--accent, #6366f1)' }}>
                  <i className="bi bi-award text-xs"></i>
                </div>
                <h2 className="text-[13px] font-bold text-slate-900 uppercase tracking-widest">
                  {i18n[lang].certifications}
                </h2>
              </div>
              <div className="relative ml-2.5 space-y-4" style={{ borderLeft: '2px solid var(--accent-light, #e0e7ff)' }}>
                {certifications.map((cert) => (
                  <div key={cert.id} className="relative pl-5">
                    <div className="absolute -left-[5.5px] top-1.5 w-2 h-2 rounded-full ring-4 ring-slate-50" style={{ backgroundColor: 'var(--accent, #6366f1)' }}></div>
                    <div className="mb-1">
                      <h3 className="text-[12.5px] font-bold text-slate-900">
                        {cert.name}
                        {cert.url && (
                          <a href={cert.url} target="_blank" rel="noopener noreferrer" className="ml-2 text-cyan-600 hover:text-cyan-800 break-all text-xs print:text-gray-900 print:no-underline">
                            <i className="bi bi-box-arrow-up-right print:hidden"></i>
                            <span className="hidden print:inline"> ({cert.url})</span>
                          </a>
                        )}
                      </h3>
                      <div className="flex justify-between items-center mt-0.5">
                        <span className="font-medium text-[11px]" style={{ color: 'var(--accent, #6366f1)' }}>{cert.issuer}</span>
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

        {/* Right Column (Objective, Skills, Languages) */}
        <div className="w-[38%] space-y-5">
          {personalInfo.objective && (
            <div>
              <p className="text-slate-700 italic text-[11px] bg-white/50 p-2 rounded border border-slate-200/50">
                "{personalInfo.objective}"
              </p>
            </div>
          )}

          {technicalSkills.length > 0 && (
            <div>
              <h2 className="text-[11px] font-bold text-slate-900 uppercase tracking-widest mb-2 border-b-2 border-slate-200 pb-1.5">
                {i18n[lang].technicalSkills}
              </h2>
              <div className="flex flex-wrap gap-1">
                {technicalSkills.map((skill, i) => (
                  <span
                    key={i}
                    className="px-2 py-0.5 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 text-[11px] rounded font-medium border border-gray-200 dark:border-gray-700"
                  >
                    {typeof skill === 'string' ? skill : skill.name}
                  </span>
                ))}
              </div>
            </div>
          )}

          {softSkills.length > 0 && (
            <div>
              <h2 className="text-[11px] font-bold text-slate-900 uppercase tracking-widest mb-2 border-b-2 border-slate-200 pb-1.5">
                {i18n[lang].softSkills}
              </h2>
              <ul className="space-y-1">
                {softSkills.map((skill, i) => (
                  <li key={i} className="text-[11px] text-slate-700 flex items-center gap-1.5">
                    <i className="bi bi-check2 text-xs" style={{ color: 'var(--accent, #6366f1)' }}></i>
                    {typeof skill === 'string' ? skill : (skill as { name: string }).name}
                  </li>
                ))}
              </ul>
            </div>
          )}

          {languages.length > 0 && (
            <div>
              <h2 className="text-[11px] font-bold text-slate-900 uppercase tracking-widest mb-2 border-b-2 border-slate-200 pb-1.5">
                {i18n[lang].languages}
              </h2>
              <div className="flex flex-col gap-1.5">
                {languages.map((langItem, i) => (
                  <div key={i} className="flex items-center gap-2 bg-white px-2.5 py-1.5 border border-slate-100 rounded shadow-sm shadow-slate-200/30">
                    <i className="bi bi-translate text-slate-400 text-[10px]"></i>
                    <span className="text-[11px] text-slate-700 font-medium">{langItem}</span>
                  </div>
                ))}
              </div>
            </div>
          )}

          {interests.length > 0 && (
            <div>
              <h2 className="text-[11px] font-bold text-slate-900 uppercase tracking-widest mb-2 border-b-2 border-slate-200 pb-1.5">
                {i18n[lang].interests}
              </h2>
              <div className="space-y-1.5">
                {interests.map((it, i) => {
                  const o = typeof it === 'string' ? { id: String(i), name: it, description: '' } : it;
                  return (
                    <div key={o.id}>
                      <h3 className="text-[11.5px] font-bold text-slate-900">{o.name}</h3>
                      {o.description && (
                        <p className="text-[10.5px] text-slate-600 leading-snug mt-0.5 whitespace-pre-wrap">{o.description}</p>
                      )}
                    </div>
                  );
                })}
              </div>
            </div>
          )}
        </div>
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
