import type { ResumeData } from '@/types/resume';

export const exampleData: ResumeData = {
  personalInfo: {
    fullName: 'Alexandra Chen',
    title: 'Senior Full-Stack Developer',
    email: 'alexandra.chen@email.com',
    phone: '+1 (555) 234-5678',
    location: 'San Francisco, CA',
    linkedin: 'linkedin.com/in/alexandrachen',
    github: 'github.com/alexandrachen',
    website: 'alexandrachen.dev',
    photoUrl: '',
    objective: 'Seeking a challenging role as a Technical Lead or Staff Engineer where I can leverage my expertise in scalable cloud architecture and mentor engineering teams.',
    drivingLicense: 'Class C (US)',
  },
  summary:
    'Passionate full-stack developer with 7+ years of experience building scalable web applications. Specialized in React, Node.js, and cloud architecture. Led teams of up to 12 engineers, delivering products that serve millions of users. Strong advocate for clean code, automated testing, and agile methodologies.',
  experience: [
    {
      id: '1',
      company: 'TechVision Inc.',
      position: 'Senior Full-Stack Developer',
      startDate: '2021-03',
      endDate: '',
      current: true,
      description:
        'Lead a team of 8 engineers building a real-time analytics platform processing 5M+ events/day. Architected microservices migration reducing deployment time by 60%. Implemented CI/CD pipelines and automated testing achieving 95% code coverage.',
    },
    {
      id: '2',
      company: 'DataStream Solutions',
      position: 'Full-Stack Developer',
      startDate: '2018-06',
      endDate: '2021-02',
      current: false,
      description:
        'Developed customer-facing dashboards using React and TypeScript. Built RESTful APIs with Node.js and PostgreSQL serving 100K+ daily active users. Optimized database queries reducing page load times by 40%.',
    },
    {
      id: '3',
      company: 'WebCraft Studios',
      position: 'Junior Developer',
      startDate: '2016-09',
      endDate: '2018-05',
      current: false,
      description:
        'Built responsive web applications using React and Vue.js for enterprise clients. Collaborated with design teams to implement pixel-perfect UIs. Participated in code reviews and mentorship programs.',
    },
  ],
  education: [
    {
      id: '1',
      institution: 'University of California, Berkeley',
      degree: 'Bachelor of Science',
      field: 'Computer Science',
      startDate: '2012-09',
      endDate: '2016-06',
      description:
        'Graduated with honors. Focus on distributed systems and machine learning. Teaching assistant for Data Structures & Algorithms.',
    },
  ],
  technicalSkills: [
    'React',
    'TypeScript',
    'Node.js',
    'Python',
    'PostgreSQL',
    'MongoDB',
    'AWS',
    'Docker',
    'Kubernetes',
    'GraphQL',
    'CI/CD',
  ],
  softSkills: [
    'Leadership',
    'Agile/Scrum',
    'Cross-functional Collaboration',
    'Mentorship',
    'Problem Solving',
  ],
  languages: [
    'English (Native)',
    'Spanish (Professional)',
    'Mandarin (Basic)',
  ],
  hiddenKeywords: [],
  cvLanguage: 'en',
};
