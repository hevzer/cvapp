'use client';

import { useResumeStore } from '@/store/useResumeStore';
import TagInput from '@/components/ui/TagInput';
import { i18n, LanguageCode } from '@/lib/i18n';

export default function TechnicalSkillsForm() {
  const technicalSkills = useResumeStore((s) => s.resumeData.technicalSkills);
  const setTechnicalSkills = useResumeStore((s) => s.setTechnicalSkills);
  const cvLanguage = useResumeStore((s) => s.resumeData.cvLanguage) as LanguageCode;

  return (
    <TagInput
      tags={technicalSkills}
      onChange={setTechnicalSkills}
      label={i18n[cvLanguage].technicalSkills}
      placeholder="e.g. React, Node.js, Python..."
    />
  );
}
