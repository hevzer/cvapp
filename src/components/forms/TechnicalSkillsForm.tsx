'use client';

import { useResumeStore } from '@/store/useResumeStore';
import TagInput from '@/components/ui/TagInput';
import { i18n, getSafeLanguage } from '@/lib/i18n';

export default function TechnicalSkillsForm() {
  const technicalSkills = useResumeStore((s) => s.resumeData.technicalSkills);
  const setTechnicalSkills = useResumeStore((s) => s.setTechnicalSkills);
  const cvLanguage = getSafeLanguage(useResumeStore((s) => s.resumeData.cvLanguage));

  return (
    <TagInput
      tags={technicalSkills || []}
      onChange={setTechnicalSkills}
      label={i18n[cvLanguage].technicalSkills}
      placeholder="e.g. React, Node.js, Python..."
    />
  );
}
