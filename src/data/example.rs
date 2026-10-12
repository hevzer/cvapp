use crate::models::{
    Certification, Education, Experience, Interest, PersonalInfo, ResumeData, TechnicalSkill,
    Volunteering,
};

pub fn get_example_data() -> ResumeData {
    ResumeData {
        personal_info: PersonalInfo {
            full_name: "Alexandra Chen".to_string(),
            title: "Senior Full-Stack Developer".to_string(),
            email: "alexandra.chen@email.com".to_string(),
            phone: "+1 (555) 234-5678".to_string(),
            location: "San Francisco, CA".to_string(),
            linkedin: "linkedin.com/in/alexandrachen".to_string(),
            github: "github.com/alexandrachen".to_string(),
            website: "alexandrachen.dev".to_string(),
            photo_url: "".to_string(),
            objective: "Seeking a challenging role as a Technical Lead or Staff Engineer where I can leverage my expertise in scalable cloud architecture and mentor engineering teams.".to_string(),
            driving_license: "Class C (US)".to_string(),
        },
        summary: "Passionate full-stack developer with 7+ years of experience building scalable web applications. Specialized in React, Node.js, and cloud architecture. Led teams of up to 12 engineers, delivering products that serve millions of users. Strong advocate for clean code, automated testing, and agile methodologies.".to_string(),
        experience: vec![
            Experience {
                id: "1".to_string(),
                company: "TechVision Inc.".to_string(),
                position: "Senior Full-Stack Developer".to_string(),
                start_date: "2021-03".to_string(),
                end_date: "".to_string(),
                current: true,
                description: "Lead a team of 8 engineers building a real-time analytics platform processing 5M+ events/day. Architected microservices migration reducing deployment time by 60%. Implemented CI/CD pipelines and automated testing achieving 95% code coverage.".to_string(),
            },
            Experience {
                id: "2".to_string(),
                company: "DataStream Solutions".to_string(),
                position: "Full-Stack Developer".to_string(),
                start_date: "2018-06".to_string(),
                end_date: "2021-02".to_string(),
                current: false,
                description: "Developed customer-facing dashboards using React and TypeScript. Built RESTful APIs with Node.js and PostgreSQL serving 100K+ daily active users. Optimized database queries reducing page load times by 40%.".to_string(),
            },
            Experience {
                id: "3".to_string(),
                company: "WebCraft Studios".to_string(),
                position: "Junior Developer".to_string(),
                start_date: "2016-09".to_string(),
                end_date: "2018-05".to_string(),
                current: false,
                description: "Built responsive web applications using React and Vue.js for enterprise clients. Collaborated with design teams to implement pixel-perfect UIs. Participated in code reviews and mentorship programs.".to_string(),
            },
        ],
        volunteering: vec![
            Volunteering {
                id: "1".to_string(),
                organization: "Code for America".to_string(),
                role: "Volunteer Software Developer".to_string(),
                start_date: "2020-01".to_string(),
                end_date: "".to_string(),
                current: true,
                description: "Contribute to civic-tech projects helping local governments modernize public services. Focus on accessibility audits and React component libraries reused across municipal apps.".to_string(),
            },
        ],
        education: vec![
            Education {
                id: "1".to_string(),
                institution: "University of California, Berkeley".to_string(),
                degree: "Bachelor of Science".to_string(),
                field: "Computer Science".to_string(),
                start_date: "2012-09".to_string(),
                end_date: "2016-06".to_string(),
                description: "Graduated with honors. Focus on distributed systems and machine learning. Teaching assistant for Data Structures & Algorithms.".to_string(),
            },
        ],
        certifications: vec![
            Certification {
                id: "1".to_string(),
                name: "AWS Certified Solutions Architect".to_string(),
                issuer: "Amazon Web Services".to_string(),
                date: "2023-05".to_string(),
                url: "aws.amazon.com/verification".to_string(),
            },
        ],
        technical_skills: vec![
            "React", "TypeScript", "Node.js", "Python", "PostgreSQL", "MongoDB", "AWS",
            "Docker", "Kubernetes", "GraphQL", "CI/CD",
        ]
        .into_iter()
        .map(|s| TechnicalSkill {
            id: uuid::Uuid::new_v4().to_string(),
            name: s.to_string(),
            description: String::new(),
        })
        .collect(),
        soft_skills: vec![
            "Leadership".to_string(),
            "Agile/Scrum".to_string(),
            "Cross-functional Collaboration".to_string(),
            "Mentorship".to_string(),
            "Problem Solving".to_string(),
        ],
        languages: vec![
            "English (Native)".to_string(),
            "Spanish (Professional)".to_string(),
            "Mandarin (Basic)".to_string(),
        ],
        interests: vec![
            Interest {
                id: "1".to_string(),
                name: "Photography".to_string(),
                description: "Street and architectural photography. Maintain a portfolio of black-and-white work shot on a Fujifilm X-T5.".to_string(),
            },
            Interest {
                id: "2".to_string(),
                name: "Trail running".to_string(),
                description: "Train year-round in the Marin Headlands. Completed two 50K ultra-marathons.".to_string(),
            },
            Interest {
                id: "3".to_string(),
                name: "Open-source contribution".to_string(),
                description: "Active maintainer of small TypeScript utility libraries; occasional PRs to React ecosystem tools.".to_string(),
            },
            Interest {
                id: "4".to_string(),
                name: "Chess".to_string(),
                description: "Online rapid rating ~1900. Enjoy studying classical endgames.".to_string(),
            },
        ],
        hidden_keywords: vec![],
        cv_language: "en".to_string(),
    }
}
