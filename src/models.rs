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

fn default_scale() -> f64 {
    1.0
}
fn default_accent_color() -> String {
    "#6366f1".to_string()
}
fn default_sidebar_color() -> String {
    "#1e293b".to_string()
}
fn default_font_family() -> String {
    "Inter".to_string()
}

pub fn current_timestamp_secs() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        (js_sys::Date::now() / 1000.0) as u64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistedState {
    #[serde(default)]
    pub resume_data: ResumeData,
    #[serde(default)]
    pub active_template: TemplateType,
    #[serde(default)]
    pub dark_mode: Option<bool>,
    #[serde(default = "default_scale")]
    pub text_scale: f64,
    #[serde(default = "default_scale")]
    pub spacing_scale: f64,
    #[serde(default = "default_scale")]
    pub font_scale: f64,
    #[serde(default = "default_accent_color")]
    pub accent_color: String,
    #[serde(default = "default_sidebar_color")]
    pub sidebar_color: String,
    #[serde(default = "default_font_family")]
    pub font_family: String,
    #[serde(default)]
    pub hide_photo: bool,
}

impl Default for PersistedState {
    fn default() -> Self {
        Self {
            resume_data: ResumeData::default(),
            active_template: TemplateType::Ats,
            dark_mode: None,
            text_scale: 1.0,
            spacing_scale: 1.0,
            font_scale: 1.0,
            accent_color: default_accent_color(),
            sidebar_color: default_sidebar_color(),
            font_family: default_font_family(),
            hide_photo: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CvProject {
    pub id: String,
    pub name: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub data: PersistedState,
}

impl CvProject {
    pub fn new(name: impl Into<String>, data: PersistedState) -> Self {
        let now = current_timestamp_secs();
        Self {
            id: generate_id(),
            name: name.into(),
            created_at: now,
            updated_at: now,
            data,
        }
    }

    pub fn duplicate(&self) -> Self {
        let now = current_timestamp_secs();
        Self {
            id: generate_id(),
            name: format!("{} (Copy)", self.name),
            created_at: now,
            updated_at: now,
            data: self.data.clone(),
        }
    }

    pub fn candidate_name(&self) -> &str {
        let name = self.data.resume_data.personal_info.full_name.trim();
        if name.is_empty() {
            "Untitled Candidate"
        } else {
            name
        }
    }

    pub fn target_role(&self) -> &str {
        let role = self.data.resume_data.personal_info.title.trim();
        if role.is_empty() {
            "Professional Resume"
        } else {
            role
        }
    }

    pub fn template_display_name(&self) -> &'static str {
        match self.data.active_template {
            TemplateType::Ats => "ATS Minimalist",
            TemplateType::Modern => "Modern Creative",
            TemplateType::Timeline => "Timeline Creative",
            TemplateType::TimelineTwoColumn => "Two-Column Timeline",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectsStore {
    pub active_project_id: Option<String>,
    pub projects: Vec<CvProject>,
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

    #[test]
    fn test_cv_project_creation_and_duplication() {
        let p = CvProject::new("Backend Dev", PersistedState::default());
        assert_eq!(p.name, "Backend Dev");
        assert!(!p.id.is_empty());

        let dup = p.duplicate();
        assert_eq!(dup.name, "Backend Dev (Copy)");
        assert_ne!(p.id, dup.id);
    }

    #[test]
    fn test_projects_store_serialization() {
        let p = CvProject::new("Design Lead", PersistedState::default());
        let id = p.id.clone();
        let store = ProjectsStore {
            active_project_id: Some(id.clone()),
            projects: vec![p],
        };
        let json = serde_json::to_string(&store).unwrap();
        let parsed: ProjectsStore = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.active_project_id, Some(id));
        assert_eq!(parsed.projects.len(), 1);
        assert_eq!(parsed.projects[0].name, "Design Lead");
    }
}

