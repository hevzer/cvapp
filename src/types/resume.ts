export interface PersonalInfo {
  fullName: string;
  title: string;
  email: string;
  phone: string;
  location: string;
  linkedin: string;
  github: string;
  website: string;
  photoUrl: string;
  objective: string;
  drivingLicense: string;
}

export interface Experience {
  id: string;
  company: string;
  position: string;
  startDate: string;
  endDate: string;
  current: boolean;
  description: string;
}

export interface Education {
  id: string;
  institution: string;
  degree: string;
  field: string;
  startDate: string;
  endDate: string;
  description: string;
}

export interface ResumeData {
  personalInfo: PersonalInfo;
  summary: string;
  experience: Experience[];
  education: Education[];
  technicalSkills: string[];
  softSkills: string[];
  languages: string[];
  hiddenKeywords: string[];
  cvLanguage: string;
}

export type TemplateType = 'ats' | 'modern' | 'timeline' | 'timelineTwoColumn';
