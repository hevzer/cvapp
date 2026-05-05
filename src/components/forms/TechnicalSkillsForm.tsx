import { useResumeStore } from '@/store/useResumeStore';
import { i18n, getSafeLanguage } from '@/lib/i18n';
import { TechnicalSkill } from '@/types/resume';
import MaterialInput from '@/components/ui/MaterialInput';
import MaterialTextarea from '@/components/ui/MaterialTextarea';

export default function TechnicalSkillsForm() {
  const technicalSkills = useResumeStore((s) => s.resumeData.technicalSkills);
  const addTechnicalSkill = useResumeStore((s) => s.addTechnicalSkill);
  const removeTechnicalSkill = useResumeStore((s) => s.removeTechnicalSkill);
  const updateTechnicalSkill = useResumeStore((s) => s.updateTechnicalSkill);
  const cvLanguage = getSafeLanguage(useResumeStore((s) => s.resumeData.cvLanguage));

  // Migration to objects happens at the store boundary (setResumeData / loadExampleData / importLinkedInData / onRehydrateStorage), so by here every entry is a TechnicalSkill object.
  const normalizedSkills = technicalSkills as TechnicalSkill[];

  return (
    <div className="space-y-4">
      {normalizedSkills.map((skill, index) => (
        <div
          key={skill.id}
          className="p-5 border border-transparent shadow-md dark:border-slate-700/50 rounded-2xl bg-white dark:bg-slate-800/60 space-y-4 relative group"
        >
          <div className="flex items-center justify-between mb-2">
            <span className="text-[11px] font-bold tracking-widest uppercase text-slate-400 dark:text-slate-500">
              {i18n[cvLanguage].technicalSkills} #{index + 1}
            </span>
            <button
              type="button"
              onClick={() => removeTechnicalSkill(skill.id)}
              className="text-[10px] tracking-wider uppercase text-red-500 hover:bg-red-50 dark:hover:bg-red-500/10 px-3 py-1.5 rounded-full font-bold transition-colors"
            >
              Remove
            </button>
          </div>
          
          <MaterialInput
            type="text"
            label="Skill Name (e.g. React, Python)"
            value={skill.name}
            onChange={(e) => updateTechnicalSkill(skill.id, { name: e.target.value })}
          />
          
          <MaterialTextarea
            label="Details & Proficiencies (Optional)"
            value={skill.description}
            onChange={(e) => updateTechnicalSkill(skill.id, { description: e.target.value })}
            rows={2}
          />
        </div>
      ))}
      
      <button
        type="button"
        onClick={addTechnicalSkill}
        className="w-full py-3.5 border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-full text-[13px] font-bold tracking-wider uppercase text-slate-500 hover:bg-slate-50 hover:text-cyan-600 hover:border-cyan-300 dark:hover:bg-slate-800 transition-colors"
      >
        + Add {i18n[cvLanguage].technicalSkills}
      </button>
    </div>
  );
}
