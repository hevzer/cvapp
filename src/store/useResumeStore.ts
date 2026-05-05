import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type {
  ResumeData,
  PersonalInfo,
  Experience,
  Education,
  Certification,
  TechnicalSkill,
  Volunteering,
  Interest,
  TemplateType,
} from '@/types/resume';
import { exampleData } from '@/data/exampleData';

function normalizeTechnicalSkills(
  skills: (TechnicalSkill | string)[] | undefined,
): TechnicalSkill[] {
  if (!skills) return [];
  return skills.map((s) =>
    typeof s === 'string'
      ? { id: crypto.randomUUID(), name: s, description: '' }
      : s,
  );
}

function normalizeInterests(interests: (Interest | string)[] | undefined): Interest[] {
  if (!interests) return [];
  return interests.map((i) =>
    typeof i === 'string'
      ? { id: crypto.randomUUID(), name: i, description: '' }
      : i,
  );
}

function normalizeResumeData(data: ResumeData): ResumeData {
  return {
    ...data,
    technicalSkills: normalizeTechnicalSkills(data.technicalSkills),
    volunteering: data.volunteering ?? [],
    interests: normalizeInterests(data.interests),
  };
}

const emptyResume: ResumeData = {
  personalInfo: {
    fullName: '',
    title: '',
    email: '',
    phone: '',
    location: '',
    linkedin: '',
    github: '',
    website: '',
    photoUrl: '',
    objective: '',
    drivingLicense: '',
  },
  summary: '',
  experience: [],
  volunteering: [],
  education: [],
  certifications: [],
  technicalSkills: [],
  softSkills: [],
  languages: [],
  interests: [],
  hiddenKeywords: [],
  cvLanguage: 'en',
};

interface ResumeStore {
  resumeData: ResumeData;
  activeTemplate: TemplateType;
  darkMode: boolean;
  textScale: number;
  spacingScale: number;
  fontScale: number;
  accentColor: string;
  sidebarColor: string;
  fontFamily: string;
  hidePhoto: boolean;

  // Personal Info
  updatePersonalInfo: (info: Partial<PersonalInfo>) => void;

  // Summary
  updateSummary: (summary: string) => void;

  // Experience
  addExperience: () => void;
  removeExperience: (id: string) => void;
  updateExperience: (id: string, data: Partial<Experience>) => void;

  // Volunteering
  addVolunteering: () => void;
  removeVolunteering: (id: string) => void;
  updateVolunteering: (id: string, data: Partial<Volunteering>) => void;

  // Education
  addEducation: () => void;
  removeEducation: (id: string) => void;
  updateEducation: (id: string, data: Partial<Education>) => void;

  // Certifications
  addCertification: () => void;
  removeCertification: (id: string) => void;
  updateCertification: (id: string, data: Partial<Certification>) => void;

  // Skills & Languages
  addTechnicalSkill: () => void;
  removeTechnicalSkill: (id: string) => void;
  updateTechnicalSkill: (id: string, data: Partial<TechnicalSkill>) => void;
  setTechnicalSkills: (skills: (TechnicalSkill | string)[]) => void;
  setSoftSkills: (skills: string[]) => void;
  setLanguages: (languages: string[]) => void;

  // Interests
  addInterest: () => void;
  removeInterest: (id: string) => void;
  updateInterest: (id: string, data: Partial<Interest>) => void;

  // Hidden Keywords
  setHiddenKeywords: (keywords: string[]) => void;

  // Settings
  setActiveTemplate: (template: TemplateType) => void;
  setCvLanguage: (lang: string) => void;
  setTextScale: (scale: number) => void;
  setSpacingScale: (scale: number) => void;
  setFontScale: (scale: number) => void;
  setAccentColor: (color: string) => void;
  setSidebarColor: (color: string) => void;
  setFontFamily: (font: string) => void;
  setHidePhoto: (hide: boolean) => void;

  // Dark mode
  toggleDarkMode: () => void;

  // Data actions
  loadExampleData: () => void;
  clearAllData: () => void;
  importLinkedInData: (data: Partial<ResumeData>) => void;
  setResumeData: (data: ResumeData) => void;
}

export const useResumeStore = create<ResumeStore>()(
  persist(
    (set) => ({
      resumeData: emptyResume,
      activeTemplate: 'ats',
      darkMode: false,
      textScale: 1,
      spacingScale: 1,
      fontScale: 1,
      accentColor: '#6366f1',
      sidebarColor: '#1e293b',
      fontFamily: 'Inter',
      hidePhoto: false,

      updatePersonalInfo: (info) =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            personalInfo: { ...state.resumeData.personalInfo, ...info },
          },
        })),

      updateSummary: (summary) =>
        set((state) => ({
          resumeData: { ...state.resumeData, summary },
        })),

      addExperience: () =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            experience: [
              ...state.resumeData.experience,
              {
                id: crypto.randomUUID(),
                company: '',
                position: '',
                startDate: '',
                endDate: '',
                current: false,
                description: '',
              },
            ],
          },
        })),

      removeExperience: (id) =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            experience: state.resumeData.experience.filter((e) => e.id !== id),
          },
        })),

      updateExperience: (id, data) =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            experience: state.resumeData.experience.map((e) =>
              e.id === id ? { ...e, ...data } : e
            ),
          },
        })),

      addVolunteering: () =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            volunteering: [
              ...(state.resumeData.volunteering ?? []),
              {
                id: crypto.randomUUID(),
                organization: '',
                role: '',
                startDate: '',
                endDate: '',
                current: false,
                description: '',
              },
            ],
          },
        })),

      removeVolunteering: (id) =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            volunteering: (state.resumeData.volunteering ?? []).filter((v) => v.id !== id),
          },
        })),

      updateVolunteering: (id, data) =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            volunteering: (state.resumeData.volunteering ?? []).map((v) =>
              v.id === id ? { ...v, ...data } : v
            ),
          },
        })),

      addEducation: () =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            education: [
              ...state.resumeData.education,
              {
                id: crypto.randomUUID(),
                institution: '',
                degree: '',
                field: '',
                startDate: '',
                endDate: '',
                description: '',
              },
            ],
          },
        })),

      removeEducation: (id) =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            education: state.resumeData.education.filter((e) => e.id !== id),
          },
        })),

      updateEducation: (id, data) =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            education: state.resumeData.education.map((e) =>
              e.id === id ? { ...e, ...data } : e
            ),
          },
        })),

      addCertification: () =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            certifications: [
              ...state.resumeData.certifications,
              {
                id: crypto.randomUUID(),
                name: '',
                issuer: '',
                date: '',
                url: '',
              },
            ],
          },
        })),

      removeCertification: (id) =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            certifications: state.resumeData.certifications.filter((c) => c.id !== id),
          },
        })),

      updateCertification: (id, data) =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            certifications: state.resumeData.certifications.map((c) =>
              c.id === id ? { ...c, ...data } : c
            ),
          },
        })),

      addTechnicalSkill: () =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            technicalSkills: [
              ...state.resumeData.technicalSkills,
              { id: crypto.randomUUID(), name: '', description: '' },
            ],
          },
        })),

      removeTechnicalSkill: (id) =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            technicalSkills: state.resumeData.technicalSkills.filter(
              (skill) => typeof skill === 'string' || skill.id !== id
            ),
          },
        })),

      updateTechnicalSkill: (id, data) =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            technicalSkills: state.resumeData.technicalSkills.map((skill) =>
              typeof skill !== 'string' && skill.id === id ? { ...skill, ...data } : skill
            ),
          },
        })),

      setTechnicalSkills: (technicalSkills) =>
        set((state) => ({
          resumeData: { ...state.resumeData, technicalSkills },
        })),

      setSoftSkills: (softSkills) =>
        set((state) => ({
          resumeData: { ...state.resumeData, softSkills },
        })),

      setLanguages: (languages) =>
        set((state) => ({
          resumeData: { ...state.resumeData, languages },
        })),

      addInterest: () =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            interests: [
              ...normalizeInterests(state.resumeData.interests),
              { id: crypto.randomUUID(), name: '', description: '' },
            ],
          },
        })),

      removeInterest: (id) =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            interests: normalizeInterests(state.resumeData.interests).filter((i) => i.id !== id),
          },
        })),

      updateInterest: (id, data) =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            interests: normalizeInterests(state.resumeData.interests).map((i) =>
              i.id === id ? { ...i, ...data } : i,
            ),
          },
        })),

      setHiddenKeywords: (keywords) =>
        set((state) => ({
          resumeData: { ...state.resumeData, hiddenKeywords: keywords },
        })),

      setActiveTemplate: (template) => set({ activeTemplate: template }),

      setTextScale: (textScale) => set({ textScale }),

      setSpacingScale: (spacingScale) => set({ spacingScale }),

      setFontScale: (fontScale) => set({ fontScale }),

      setAccentColor: (accentColor) => set({ accentColor }),

      setSidebarColor: (sidebarColor) => set({ sidebarColor }),

      setFontFamily: (fontFamily) => set({ fontFamily }),

      setHidePhoto: (hidePhoto) => set({ hidePhoto }),

      setCvLanguage: (cvLanguage) =>
        set((state) => ({
          resumeData: { ...state.resumeData, cvLanguage },
        })),

      toggleDarkMode: () =>
        set((state) => ({ darkMode: !state.darkMode })),

      loadExampleData: () => set({ resumeData: normalizeResumeData(exampleData) }),

      clearAllData: () => set({ resumeData: emptyResume }),

      importLinkedInData: (data) =>
        set((state) => ({
          resumeData: normalizeResumeData({
            ...state.resumeData,
            ...data,
            personalInfo: {
              ...state.resumeData.personalInfo,
              ...(data.personalInfo || {}),
            },
          }),
        })),
      setResumeData: (resumeData) => set({ resumeData: normalizeResumeData(resumeData) }),
    }),
    {
      name: 'cvapp-storage',
      onRehydrateStorage: () => (state) => {
        if (state?.resumeData) {
          state.resumeData = normalizeResumeData(state.resumeData);
        }
      },
    }
  )
);
