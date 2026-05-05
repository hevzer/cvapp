export type LanguageCode = 'en' | 'fr';

export const languageOptions: { code: LanguageCode; label: string }[] = [
  { code: 'en', label: 'English' },
  { code: 'fr', label: 'Français' },
];

type Dictionary = {
  [key in LanguageCode]: {
    profile: string;
    experience: string;
    volunteering: string;
    education: string;
    contact: string;
    certifications: string;
    technicalSkills: string;
    softSkills: string;
    languages: string;
    interests: string;
    drivingLicense: string;
    present: string;
  };
};

export const i18n: Dictionary = {
  en: {
    profile: 'Profile',
    experience: 'Experience',
    volunteering: 'Volunteering',
    education: 'Education',
    contact: 'Contact',
    certifications: 'Certifications',
    technicalSkills: 'Technical Skills',
    softSkills: 'Soft Skills',
    languages: 'Languages',
    interests: 'Interests',
    drivingLicense: 'Driving License',
    present: 'Present',
  },
  fr: {
    profile: 'Profil',
    experience: 'Parcours Professionnel',
    volunteering: 'Bénévolat',
    education: 'Formation',
    contact: 'Contact',
    certifications: 'Certifications',
    technicalSkills: 'Compétences Techniques',
    softSkills: 'Soft Skills',
    languages: 'Langues',
    interests: "Centres d'intérêt",
    drivingLicense: 'Permis de Conduire',
    present: 'Présent',
  },
};

export function getSafeLanguage(lang: string | undefined | null): LanguageCode {
  if (!lang || !i18n[lang as LanguageCode]) return 'en';
  return lang as LanguageCode;
}
