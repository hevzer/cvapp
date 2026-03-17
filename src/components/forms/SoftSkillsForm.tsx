'use client';

import { useResumeStore } from '@/store/useResumeStore';
import TagInput from '@/components/ui/TagInput';
import { i18n, LanguageCode } from '@/lib/i18n';

export default function SoftSkillsForm() {
  const softSkills = useResumeStore((s) => s.resumeData.softSkills);
  const setSoftSkills = useResumeStore((s) => s.setSoftSkills);
  const cvLanguage = useResumeStore((s) => s.resumeData.cvLanguage) as LanguageCode;

  return (
    <TagInput
      tags={softSkills}
      onChange={setSoftSkills}
      label={i18n[cvLanguage].softSkills}
      placeholder="e.g. Leadership, Communication..."
    />
  );
}
