#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dictionary {
    pub profile: &'static str,
    pub experience: &'static str,
    pub volunteering: &'static str,
    pub education: &'static str,
    pub contact: &'static str,
    pub certifications: &'static str,
    pub technical_skills: &'static str,
    pub soft_skills: &'static str,
    pub languages: &'static str,
    pub interests: &'static str,
    pub driving_license: &'static str,
    pub present: &'static str,
}

pub const EN: Dictionary = Dictionary {
    profile: "Profile",
    experience: "Experience",
    volunteering: "Volunteering",
    education: "Education",
    contact: "Contact",
    certifications: "Certifications",
    technical_skills: "Technical Skills",
    soft_skills: "Soft Skills",
    languages: "Languages",
    interests: "Interests",
    driving_license: "Driving License",
    present: "Present",
};

pub const FR: Dictionary = Dictionary {
    profile: "Profil",
    experience: "Parcours Professionnel",
    volunteering: "Bénévolat",
    education: "Formation",
    contact: "Contact",
    certifications: "Certifications",
    technical_skills: "Compétences Techniques",
    soft_skills: "Soft Skills",
    languages: "Langues",
    interests: "Centres d'intérêt",
    driving_license: "Permis de Conduire",
    present: "Présent",
};

pub fn get_dictionary(lang: &str) -> &'static Dictionary {
    match lang.to_lowercase().as_str() {
        "fr" => &FR,
        _ => &EN,
    }
}

pub struct LanguageOption {
    pub code: &'static str,
    pub label: &'static str,
}

pub const LANGUAGE_OPTIONS: &[LanguageOption] = &[
    LanguageOption {
        code: "en",
        label: "English",
    },
    LanguageOption {
        code: "fr",
        label: "Français",
    },
];
