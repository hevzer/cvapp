'use client';

import { useRef, useState } from 'react';
import { useResumeStore } from '@/store/useResumeStore';

export default function LinkedInImport() {
  const importLinkedInData = useResumeStore((s) => s.importLinkedInData);
  const fileRef = useRef<HTMLInputElement>(null);
  const [showGuide, setShowGuide] = useState(false);

  const handleFile = async (file: File) => {
    try {
      if (file.name.endsWith('.zip')) {
        const JSZip = (await import('jszip')).default;
        const zip = await JSZip.loadAsync(file);

        let profileData: Record<string, string> = {};
        let positions: Array<Record<string, string>> = [];
        let educationEntries: Array<Record<string, string>> = [];
        let skillEntries: string[] = [];

        for (const [filename, zipFile] of Object.entries(zip.files)) {
          if (zipFile.dir) continue;
          const lower = filename.toLowerCase();
          const content = await zipFile.async('text');

          try {
            const json = JSON.parse(content);

            if (lower.includes('profile')) {
              profileData = Array.isArray(json) ? json[0] || {} : json;
            } else if (lower.includes('position')) {
              positions = Array.isArray(json) ? json : [];
            } else if (lower.includes('education')) {
              educationEntries = Array.isArray(json) ? json : [];
            } else if (lower.includes('skill')) {
              if (Array.isArray(json)) {
                skillEntries = json
                  .map(
                    (s: Record<string, string>) =>
                      s.Name || s.name || s.Skill || s.skill || ''
                  )
                  .filter(Boolean);
              }
            }
          } catch {
            // Not a JSON file, skip
          }
        }

        importLinkedInData({
          personalInfo: {
            fullName:
              profileData['First Name'] && profileData['Last Name']
                ? `${profileData['First Name']} ${profileData['Last Name']}`
                : profileData.fullName || profileData.name || '',
            title:
              profileData.Headline || profileData.headline || profileData.title || '',
            email: profileData['Email Address'] || profileData.email || '',
            phone: '',
            location: profileData.Location || profileData.location || '',
            linkedin: profileData['Profile URL'] || profileData.url || '',
            website: profileData.Website || profileData.website || '',
            photoUrl: '',
            github: '',
            objective: '',
            drivingLicense: '',
          },
          summary:
            profileData.Summary || profileData.summary || profileData.About || '',
          experience: positions.map((p) => ({
            id: crypto.randomUUID(),
            company: p['Company Name'] || p.company || p.companyName || '',
            position: p.Title || p.title || p.position || '',
            startDate: p['Started On'] || p.startDate || '',
            endDate: p['Finished On'] || p.endDate || '',
            current: !p['Finished On'] && !p.endDate,
            description: p.Description || p.description || '',
          })),
          education: educationEntries.map((e) => ({
            id: crypto.randomUUID(),
            institution: e['School Name'] || e.school || e.institution || '',
            degree: e['Degree Name'] || e.degree || '',
            field: e['Field Of Study'] || e.field || e.fieldOfStudy || '',
            startDate: e['Start Date'] || e.startDate || '',
            endDate: e['End Date'] || e.endDate || '',
            description: e.Notes || e.description || e.activities || '',
          })),
          technicalSkills: skillEntries,
        });

        alert('LinkedIn data imported successfully!');
      } else if (file.name.endsWith('.json')) {
        const content = await file.text();
        const json = JSON.parse(content);

        importLinkedInData({
          personalInfo: {
            fullName:
              json.fullName ||
              json.name ||
              `${json.firstName || ''} ${json.lastName || ''}`.trim(),
            title: json.headline || json.title || '',
            email: json.email || json.emailAddress || '',
            phone: json.phone || '',
            location: json.location || json.address || '',
            linkedin: json.linkedin || json.linkedIn || '',
            website: json.website || json.portfolio || '',
            photoUrl: '',
            github: '',
            objective: '',
            drivingLicense: '',
          },
          summary: json.summary || json.about || json.objective || '',
          experience: (json.positions || json.experience || []).map(
            (p: Record<string, string>) => ({
              id: crypto.randomUUID(),
              company: p.companyName || p.company || '',
              position: p.title || p.position || '',
              startDate: p.startDate || '',
              endDate: p.endDate || '',
              current: !p.endDate,
              description: p.description || '',
            })
          ),
          education: (json.education || []).map(
            (e: Record<string, string>) => ({
              id: crypto.randomUUID(),
              institution: e.schoolName || e.school || e.institution || '',
              degree: e.degreeName || e.degree || '',
              field: e.fieldOfStudy || e.field || '',
              startDate: e.startDate || '',
              endDate: e.endDate || '',
              description: e.description || e.notes || '',
            })
          ),
          technicalSkills: json.skills
            ? json.skills
                .map((s: string | Record<string, string>) =>
                  typeof s === 'string' ? s : s.name || s.Name || ''
                )
                .filter(Boolean)
            : [],
        });

        alert('LinkedIn data imported successfully!');
      } else {
        alert('Please upload a .zip or .json file from LinkedIn data export.');
      }
    } catch (err) {
      console.error('Import error:', err);
      alert('Failed to parse LinkedIn data. Please check the file format.');
    }
  };

  return (
    <div className="space-y-2">
      <input
        ref={fileRef}
        type="file"
        accept=".zip,.json"
        onChange={(e) => {
          const file = e.target.files?.[0];
          if (file) handleFile(file);
          e.target.value = '';
        }}
        className="hidden"
      />

      {/* Import Button */}
      <button
        type="button"
        onClick={() => fileRef.current?.click()}
        className="w-full flex items-center justify-center gap-2 px-4 py-2.5 bg-[#0A66C2] hover:bg-[#004182] text-white text-sm font-semibold rounded-2xl active:scale-[0.98] transition-all duration-200 shadow-sm shadow-[#0A66C2]/20 hover:shadow-md hover:shadow-[#0A66C2]/25"
      >
        <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
          <path d="M19 0H5a5 5 0 00-5 5v14a5 5 0 005 5h14a5 5 0 005-5V5a5 5 0 00-5-5zM8 19H5v-9h3v9zM6.5 8.25A1.75 1.75 0 118.25 6.5 1.75 1.75 0 016.5 8.25zM20 19h-3v-4.74c0-1.42-.6-1.93-1.38-1.93A1.74 1.74 0 0014 14.19V19h-3v-9h2.9v1.3a3.11 3.11 0 012.7-1.4c1.55 0 3.36.86 3.36 3.66V19z" />
        </svg>
        Import from LinkedIn
      </button>

      {/* Guide Toggle */}
      <button
        type="button"
        onClick={() => setShowGuide(!showGuide)}
        className="w-full text-xs text-gray-500 dark:text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400 font-medium transition-colors flex items-center justify-center gap-1"
      >
        <svg
          className={`w-3.5 h-3.5 transition-transform duration-300 ${showGuide ? 'rotate-180' : ''}`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
        {showGuide ? 'Hide export guide' : 'How to export your LinkedIn data?'}
      </button>

      {/* Export Guide */}
      {showGuide && (
        <div className="p-4 bg-white/60 dark:bg-gray-800/60 backdrop-blur-lg border border-gray-200/50 dark:border-gray-700/50 rounded-xl text-xs space-y-3 animate-slide-up">
          <h4 className="font-bold text-gray-800 dark:text-gray-100 text-sm flex items-center gap-2">
            <span className="w-6 h-6 bg-[#0A66C2] rounded-md flex items-center justify-center text-white text-[10px] font-bold">in</span>
            Export your LinkedIn data
          </h4>

          <ol className="space-y-2.5 text-gray-600 dark:text-gray-400 leading-relaxed">
            <li className="flex gap-2.5">
              <span className="flex-shrink-0 w-5 h-5 bg-indigo-100 dark:bg-indigo-900/40 text-indigo-600 dark:text-indigo-400 rounded-full flex items-center justify-center text-[10px] font-bold mt-0.5">1</span>
              <span>Go to <strong className="text-gray-800 dark:text-gray-200">LinkedIn.com</strong> → Click your profile picture → <strong className="text-gray-800 dark:text-gray-200">Settings & Privacy</strong></span>
            </li>
            <li className="flex gap-2.5">
              <span className="flex-shrink-0 w-5 h-5 bg-indigo-100 dark:bg-indigo-900/40 text-indigo-600 dark:text-indigo-400 rounded-full flex items-center justify-center text-[10px] font-bold mt-0.5">2</span>
              <span>Under <strong className="text-gray-800 dark:text-gray-200">Data Privacy</strong>, click <strong className="text-gray-800 dark:text-gray-200">&quot;Get a copy of your data&quot;</strong></span>
            </li>
            <li className="flex gap-2.5">
              <span className="flex-shrink-0 w-5 h-5 bg-indigo-100 dark:bg-indigo-900/40 text-indigo-600 dark:text-indigo-400 rounded-full flex items-center justify-center text-[10px] font-bold mt-0.5">3</span>
              <span>Select <strong className="text-gray-800 dark:text-gray-200">&quot;Want something in particular?&quot;</strong> and pick the data you want (Profile, Positions, Education, Skills)</span>
            </li>
            <li className="flex gap-2.5">
              <span className="flex-shrink-0 w-5 h-5 bg-indigo-100 dark:bg-indigo-900/40 text-indigo-600 dark:text-indigo-400 rounded-full flex items-center justify-center text-[10px] font-bold mt-0.5">4</span>
              <span>Click <strong className="text-gray-800 dark:text-gray-200">&quot;Request archive&quot;</strong> — LinkedIn will email you a download link (usually takes ~10 minutes)</span>
            </li>
            <li className="flex gap-2.5">
              <span className="flex-shrink-0 w-5 h-5 bg-indigo-100 dark:bg-indigo-900/40 text-indigo-600 dark:text-indigo-400 rounded-full flex items-center justify-center text-[10px] font-bold mt-0.5">5</span>
              <span>Download the <strong className="text-gray-800 dark:text-gray-200">.zip file</strong> and upload it here using the button above</span>
            </li>
          </ol>

          <div className="flex items-start gap-2 p-2.5 bg-amber-50 dark:bg-amber-950/30 border border-amber-200/50 dark:border-amber-800/50 rounded-lg">
            <span className="text-amber-500 mt-0.5">💡</span>
            <p className="text-amber-700 dark:text-amber-400 leading-relaxed">
              <strong>Tip:</strong> LinkedIn exports CSV by default. The data will still be imported — CVapp handles both CSV and JSON formats within the ZIP.
            </p>
          </div>
        </div>
      )}
    </div>
  );
}
