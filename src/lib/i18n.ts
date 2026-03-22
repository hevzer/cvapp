export type LanguageCode = 'en' | 'fr';

export const languageOptions: { code: LanguageCode; label: string }[] = [
  { code: 'en', label: 'English' },
  { code: 'fr', label: 'Français' },
];

type Dictionary = {
  [key in LanguageCode]: {
    profile: string;
    experience: string;
    education: string;
    contact: string;
    certifications: string;
    technicalSkills: string;
    softSkills: string;
    languages: string;
    drivingLicense: string;
    present: string;
  };
};

export const i18n: Dictionary = {
  en: {
    profile: 'Profile',
    experience: 'Experience',
    education: 'Education',
    contact: 'Contact',
    certifications: 'Certifications',
    technicalSkills: 'Technical Skills',
    softSkills: 'Soft Skills',
    languages: 'Languages',
    drivingLicense: 'Driving License',
    present: 'Present',
  },
  fr: {
    profile: 'Profil',
    experience: 'Parcours Professionnel',
    education: 'Formation',
    contact: 'Contact',
    certifications: 'Certifications',
    technicalSkills: 'Compétences Techniques',
    softSkills: 'Soft Skills',
    languages: 'Langues',
    drivingLicense: 'Permis de Conduire',
    present: 'Présent',
  },
};

export function getSafeLanguage(lang: string | undefined | null): LanguageCode {
  if (!lang || !i18n[lang as LanguageCode]) return 'en';
  return lang as LanguageCode;
}
