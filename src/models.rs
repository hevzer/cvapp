use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalInfo {
    #[serde(default)]
    pub full_name: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub linkedin: String,
    #[serde(default)]
    pub github: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub photo_url: String,
    #[serde(default)]
    pub objective: String,
    #[serde(default)]
    pub driving_license: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Experience {
    #[serde(default = "generate_id")]
    pub id: String,
    #[serde(default)]
    pub company: String,
    #[serde(default)]
    pub position: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
    #[serde(default)]
    pub current: bool,
    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Education {
    #[serde(default = "generate_id")]
    pub id: String,
    #[serde(default)]
    pub institution: String,
    #[serde(default)]
    pub degree: String,
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Certification {
    #[serde(default = "generate_id")]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub issuer: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub url: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TechnicalSkill {
    #[serde(default = "generate_id")]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volunteering {
    #[serde(default = "generate_id")]
    pub id: String,
    #[serde(default)]
    pub organization: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
    #[serde(default)]
    pub current: bool,
    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interest {
    #[serde(default = "generate_id")]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResumeData {
    #[serde(default)]
    pub personal_info: PersonalInfo,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub experience: Vec<Experience>,
    #[serde(default)]
    pub volunteering: Vec<Volunteering>,
    #[serde(default)]
    pub education: Vec<Education>,
    #[serde(default)]
    pub certifications: Vec<Certification>,
    #[serde(default)]
    pub technical_skills: Vec<TechnicalSkill>,
    #[serde(default)]
    pub soft_skills: Vec<String>,
    #[serde(default)]
    pub languages: Vec<String>,
    #[serde(default)]
    pub interests: Vec<Interest>,
    #[serde(default)]
    pub hidden_keywords: Vec<String>,
    #[serde(default = "default_cv_language")]
    pub cv_language: String,
}

impl Default for ResumeData {
    fn default() -> Self {
        Self {
            personal_info: PersonalInfo::default(),
            summary: String::new(),
            experience: Vec::new(),
            volunteering: Vec::new(),
            education: Vec::new(),
            certifications: Vec::new(),
            technical_skills: Vec::new(),
            soft_skills: Vec::new(),
            languages: Vec::new(),
            interests: Vec::new(),
            hidden_keywords: Vec::new(),
            cv_language: default_cv_language(),
        }
    }
}

fn default_cv_language() -> String {
    "en".to_string()
}

pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TemplateType {
    #[default]
    Ats,
    Modern,
    Timeline,
    TimelineTwoColumn,
}

impl TemplateType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ats => "ats",
            Self::Modern => "modern",
            Self::Timeline => "timeline",
            Self::TimelineTwoColumn => "timelineTwoColumn",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "modern" => Self::Modern,
            "timeline" => Self::Timeline,
            "timelineTwoColumn" => Self::TimelineTwoColumn,
            _ => Self::Ats,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_type_serialization() {
        assert_eq!(serde_json::to_string(&TemplateType::Ats).unwrap(), "\"ats\"");
        assert_eq!(serde_json::to_string(&TemplateType::Modern).unwrap(), "\"modern\"");
        assert_eq!(serde_json::to_string(&TemplateType::Timeline).unwrap(), "\"timeline\"");
        assert_eq!(serde_json::to_string(&TemplateType::TimelineTwoColumn).unwrap(), "\"timelineTwoColumn\"");

        let t: TemplateType = serde_json::from_str("\"timelineTwoColumn\"").unwrap();
        assert_eq!(t, TemplateType::TimelineTwoColumn);
    }

    #[test]
    fn test_resume_data_serialization_roundtrip() {
        let resume = ResumeData::default();
        let json = serde_json::to_string(&resume).unwrap();
        let parsed: ResumeData = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.cv_language, "en");
    }
}

