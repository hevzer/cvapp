'use client';

import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type {
  ResumeData,
  PersonalInfo,
  Experience,
  Education,
  TemplateType,
} from '@/types/resume';
import { exampleData } from '@/data/exampleData';

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
  education: [],
  technicalSkills: [],
  softSkills: [],
  languages: [],
  hiddenKeywords: [],
  cvLanguage: 'en',
};

interface ResumeStore {
  resumeData: ResumeData;
  activeTemplate: TemplateType;
  darkMode: boolean;
  textScale: number;

  // Personal Info
  updatePersonalInfo: (info: Partial<PersonalInfo>) => void;

  // Summary
  updateSummary: (summary: string) => void;

  // Experience
  addExperience: () => void;
  removeExperience: (id: string) => void;
  updateExperience: (id: string, data: Partial<Experience>) => void;

  // Education
  addEducation: () => void;
  removeEducation: (id: string) => void;
  updateEducation: (id: string, data: Partial<Education>) => void;

  // Skills & Languages
  setTechnicalSkills: (skills: string[]) => void;
  setSoftSkills: (skills: string[]) => void;
  setLanguages: (languages: string[]) => void;

  // Hidden Keywords
  setHiddenKeywords: (keywords: string[]) => void;

  // Settings
  setActiveTemplate: (template: TemplateType) => void;
  setCvLanguage: (lang: string) => void;
  setTextScale: (scale: number) => void;

  // Dark mode
  toggleDarkMode: () => void;

  // Data actions
  loadExampleData: () => void;
  clearAllData: () => void;
  importLinkedInData: (data: Partial<ResumeData>) => void;
}

export const useResumeStore = create<ResumeStore>()(
  persist(
    (set) => ({
      resumeData: emptyResume,
      activeTemplate: 'ats',
      darkMode: false,
      textScale: 1,

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

      setHiddenKeywords: (keywords) =>
        set((state) => ({
          resumeData: { ...state.resumeData, hiddenKeywords: keywords },
        })),

      setActiveTemplate: (template) => set({ activeTemplate: template }),

      setTextScale: (textScale) => set({ textScale }),

      setCvLanguage: (cvLanguage) =>
        set((state) => ({
          resumeData: { ...state.resumeData, cvLanguage },
        })),

      toggleDarkMode: () =>
        set((state) => ({ darkMode: !state.darkMode })),

      loadExampleData: () => set({ resumeData: exampleData }),

      clearAllData: () => set({ resumeData: emptyResume }),

      importLinkedInData: (data) =>
        set((state) => ({
          resumeData: {
            ...state.resumeData,
            ...data,
            personalInfo: {
              ...state.resumeData.personalInfo,
              ...(data.personalInfo || {}),
            },
          },
        })),
    }),
    {
      name: 'cvapp-storage',
    }
  )
);
