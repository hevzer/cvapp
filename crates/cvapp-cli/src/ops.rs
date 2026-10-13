use cvapp::data::example::get_example_data;
use cvapp::models::{
    current_timestamp_secs, generate_id, Certification, CvProject, Education, Experience,
    PersistedState, ProjectsStore, TechnicalSkill, TemplateType,
};
use cvapp::services::linkedin::{parse_linkedin_json, parse_linkedin_zip};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::Path;

use crate::storage::{find_project, find_project_mut};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectSummary {
    pub id: String,
    pub name: String,
    pub candidate_name: String,
    pub target_role: String,
    pub template: String,
    pub is_active: bool,
    pub updated_at: u64,
}

pub fn list_projects(store: &ProjectsStore) -> Vec<ProjectSummary> {
    let active_id = store.active_project_id.as_deref().unwrap_or_default();
    store
        .projects
        .iter()
        .map(|p| ProjectSummary {
            id: p.id.clone(),
            name: p.name.clone(),
            candidate_name: p.candidate_name().to_string(),
            target_role: p.target_role().to_string(),
            template: p.template_display_name().to_string(),
            is_active: p.id == active_id,
            updated_at: p.updated_at,
        })
        .collect()
}

pub fn create_project(store: &mut ProjectsStore, name: &str, use_example: bool) -> String {
    let state = if use_example {
        PersistedState {
            resume_data: get_example_data(),
            active_template: TemplateType::Modern,
            ..Default::default()
        }
    } else {
        PersistedState::default()
    };

    let project = CvProject::new(name, state);
    let new_id = project.id.clone();
    store.projects.insert(0, project);
    store.active_project_id = Some(new_id.clone());
    new_id
}

pub fn duplicate_project(
    store: &mut ProjectsStore,
    query: Option<&str>,
    custom_name: Option<&str>,
) -> Result<String, String> {
    let original = find_project(store, query)?;
    let mut cloned = original.duplicate();
    if let Some(name) = custom_name {
        if !name.trim().is_empty() {
            cloned.name = name.trim().to_string();
        }
    }
    let new_id = cloned.id.clone();
    store.projects.insert(0, cloned);
    store.active_project_id = Some(new_id.clone());
    Ok(new_id)
}

pub fn rename_project(store: &mut ProjectsStore, query: Option<&str>, new_name: &str) -> Result<(), String> {
    let trimmed = new_name.trim();
    if trimmed.is_empty() {
        return Err("New project name cannot be empty.".to_string());
    }
    let p = find_project_mut(store, query)?;
    p.name = trimmed.to_string();
    p.updated_at = current_timestamp_secs();
    Ok(())
}

pub fn delete_project(store: &mut ProjectsStore, query: &str) -> Result<(), String> {
    let target = find_project(store, Some(query))?;
    let target_id = target.id.clone();

    store.projects.retain(|p| p.id != target_id);

    if store.projects.is_empty() {
        let fresh = CvProject::new("My Resume", PersistedState::default());
        store.active_project_id = Some(fresh.id.clone());
        store.projects.push(fresh);
    } else if store.active_project_id.as_deref() == Some(&target_id) {
        store.active_project_id = store.projects.first().map(|p| p.id.clone());
    }

    Ok(())
}

pub fn set_active_project(store: &mut ProjectsStore, query: &str) -> Result<String, String> {
    let target = find_project(store, Some(query))?;
    let id = target.id.clone();
    store.active_project_id = Some(id.clone());
    Ok(id)
}

#[derive(Default, Debug)]
pub struct UpdateProfileArgs {
    pub full_name: Option<String>,
    pub title: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    pub website: Option<String>,
    pub objective: Option<String>,
    pub driving_license: Option<String>,
}

pub fn update_personal_info(
    store: &mut ProjectsStore,
    query: Option<&str>,
    args: UpdateProfileArgs,
) -> Result<(), String> {
    let p = find_project_mut(store, query)?;
    let info = &mut p.data.resume_data.personal_info;

    if let Some(v) = args.full_name { info.full_name = v.trim().to_string(); }
    if let Some(v) = args.title { info.title = v.trim().to_string(); }
    if let Some(v) = args.email { info.email = v.trim().to_string(); }
    if let Some(v) = args.phone { info.phone = v.trim().to_string(); }
    if let Some(v) = args.location { info.location = v.trim().to_string(); }
    if let Some(v) = args.linkedin { info.linkedin = v.trim().to_string(); }
    if let Some(v) = args.github { info.github = v.trim().to_string(); }
    if let Some(v) = args.website { info.website = v.trim().to_string(); }
    if let Some(v) = args.objective { info.objective = v.trim().to_string(); }
    if let Some(v) = args.driving_license { info.driving_license = v.trim().to_string(); }

    p.updated_at = current_timestamp_secs();
    Ok(())
}

pub fn set_summary(store: &mut ProjectsStore, query: Option<&str>, text: &str) -> Result<(), String> {
    let p = find_project_mut(store, query)?;
    p.data.resume_data.summary = text.trim().to_string();
    p.updated_at = current_timestamp_secs();
    Ok(())
}

pub fn add_experience(
    store: &mut ProjectsStore,
    query: Option<&str>,
    company: &str,
    position: &str,
    start_date: &str,
    end_date: Option<&str>,
    current: bool,
    description: &str,
) -> Result<String, String> {
    let p = find_project_mut(store, query)?;
    let id = generate_id();
    p.data.resume_data.experience.push(Experience {
        id: id.clone(),
        company: company.trim().to_string(),
        position: position.trim().to_string(),
        start_date: start_date.trim().to_string(),
        end_date: end_date.unwrap_or_default().trim().to_string(),
        current,
        description: description.trim().to_string(),
    });
    p.updated_at = current_timestamp_secs();
    Ok(id)
}

pub fn add_education(
    store: &mut ProjectsStore,
    query: Option<&str>,
    institution: &str,
    degree: &str,
    field: &str,
    start_date: &str,
    end_date: Option<&str>,
    description: Option<&str>,
) -> Result<String, String> {
    let p = find_project_mut(store, query)?;
    let id = generate_id();
    p.data.resume_data.education.push(Education {
        id: id.clone(),
        institution: institution.trim().to_string(),
        degree: degree.trim().to_string(),
        field: field.trim().to_string(),
        start_date: start_date.trim().to_string(),
        end_date: end_date.unwrap_or_default().trim().to_string(),
        description: description.unwrap_or_default().trim().to_string(),
    });
    p.updated_at = current_timestamp_secs();
    Ok(id)
}

pub fn set_skills(
    store: &mut ProjectsStore,
    query: Option<&str>,
    technical_skills: Option<Vec<TechnicalSkill>>,
    soft_skills: Option<Vec<String>>,
) -> Result<(), String> {
    let p = find_project_mut(store, query)?;
    if let Some(tech) = technical_skills {
        p.data.resume_data.technical_skills = tech;
    }
    if let Some(soft) = soft_skills {
        p.data.resume_data.soft_skills = soft;
    }
    p.updated_at = current_timestamp_secs();
    Ok(())
}

pub fn add_technical_skill(
    store: &mut ProjectsStore,
    query: Option<&str>,
    name: &str,
    description: Option<&str>,
) -> Result<String, String> {
    let p = find_project_mut(store, query)?;
    let id = generate_id();
    p.data.resume_data.technical_skills.push(TechnicalSkill {
        id: id.clone(),
        name: name.trim().to_string(),
        description: description.unwrap_or_default().trim().to_string(),
    });
    p.updated_at = current_timestamp_secs();
    Ok(id)
}

pub fn add_certification(
    store: &mut ProjectsStore,
    query: Option<&str>,
    name: &str,
    issuer: &str,
    date: &str,
    url: Option<&str>,
) -> Result<String, String> {
    let p = find_project_mut(store, query)?;
    let id = generate_id();
    p.data.resume_data.certifications.push(Certification {
        id: id.clone(),
        name: name.trim().to_string(),
        issuer: issuer.trim().to_string(),
        date: date.trim().to_string(),
        url: url.unwrap_or_default().trim().to_string(),
    });
    p.updated_at = current_timestamp_secs();
    Ok(id)
}

pub fn set_languages(
    store: &mut ProjectsStore,
    query: Option<&str>,
    languages: Vec<String>,
) -> Result<(), String> {
    let p = find_project_mut(store, query)?;
    p.data.resume_data.languages = languages
        .into_iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    p.updated_at = current_timestamp_secs();
    Ok(())
}

pub fn inject_ats_keywords(
    store: &mut ProjectsStore,
    query: Option<&str>,
    new_keywords: Vec<String>,
) -> Result<usize, String> {
    let p = find_project_mut(store, query)?;
    let mut current_set: HashSet<String> = p
        .data
        .resume_data
        .hidden_keywords
        .iter()
        .map(|k| k.to_lowercase())
        .collect();

    let mut added_count = 0;
    for kw in new_keywords {
        let trimmed = kw.trim();
        if !trimmed.is_empty() && !current_set.contains(&trimmed.to_lowercase()) {
            current_set.insert(trimmed.to_lowercase());
            p.data.resume_data.hidden_keywords.push(trimmed.to_string());
            added_count += 1;
        }
    }

    p.updated_at = current_timestamp_secs();
    Ok(added_count)
}

pub fn clear_ats_keywords(store: &mut ProjectsStore, query: Option<&str>) -> Result<(), String> {
    let p = find_project_mut(store, query)?;
    p.data.resume_data.hidden_keywords.clear();
    p.updated_at = current_timestamp_secs();
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AtsFitAnalysis {
    pub match_score: f64,
    pub rating: String,
    pub matched_keywords: Vec<String>,
    pub missing_keywords: Vec<String>,
    pub recommendations: Vec<String>,
}

pub fn analyze_job_fit(project: &CvProject, job_description: &str) -> AtsFitAnalysis {
    let resume = &project.data.resume_data;

    // Collect all words and phrases from resume
    let mut resume_corpus = format!(
        "{} {} {} {}",
        resume.personal_info.title,
        resume.personal_info.objective,
        resume.summary,
        resume.hidden_keywords.join(" ")
    );

    for exp in &resume.experience {
        resume_corpus.push_str(&format!(" {} {} {}", exp.position, exp.company, exp.description));
    }
    for edu in &resume.education {
        resume_corpus.push_str(&format!(" {} {} {}", edu.degree, edu.field, edu.description));
    }
    for skill in &resume.technical_skills {
        resume_corpus.push_str(&format!(" {} {}", skill.name, skill.description));
    }
    for soft in &resume.soft_skills {
        resume_corpus.push_str(&format!(" {}", soft));
    }
    for cert in &resume.certifications {
        resume_corpus.push_str(&format!(" {} {}", cert.name, cert.issuer));
    }

    let resume_corpus_lower = resume_corpus.to_lowercase();

    // Extract potential tech and industry keywords from job description
    let stop_words: HashSet<&str> = [
        "a", "about", "above", "after", "again", "against", "all", "am", "an", "and", "any", "are",
        "aren't", "as", "at", "be", "because", "been", "before", "being", "below", "between",
        "both", "but", "by", "can't", "cannot", "could", "couldn't", "did", "didn't", "do",
        "does", "doesn't", "doing", "don't", "down", "during", "each", "few", "for", "from",
        "further", "had", "hadn't", "has", "hasn't", "have", "haven't", "having", "he", "he'd",
        "he'll", "he's", "her", "here", "here's", "hers", "herself", "him", "himself", "his",
        "how", "how's", "i", "i'd", "i'll", "i'm", "i've", "if", "in", "into", "is", "isn't",
        "it", "it's", "its", "itself", "let's", "me", "more", "most", "mustn't", "my", "myself",
        "no", "nor", "not", "of", "off", "on", "once", "only", "or", "other", "ought", "our",
        "ours", "ourselves", "out", "over", "own", "same", "shan't", "she", "she'd", "she'll",
        "she's", "should", "shouldn't", "so", "some", "such", "than", "that", "that's", "the",
        "their", "theirs", "them", "themselves", "then", "there", "there's", "these", "they",
        "they'd", "they'll", "they're", "they've", "this", "those", "through", "to", "too",
        "under", "until", "up", "very", "was", "wasn't", "we", "we'd", "we'll", "we're", "we've",
        "were", "weren't", "what", "what's", "when", "when's", "where", "where's", "which",
        "while", "who", "who's", "whom", "why", "why's", "with", "won't", "would", "wouldn't",
        "you", "you'd", "you'll", "you're", "you've", "your", "yours", "yourself", "yourselves",
        "job", "role", "work", "responsibilities", "requirements", "candidate", "experience",
        "required", "preferred", "looking", "ability", "skills", "team", "strong", "plus",
        "years", "degree", "opportunity", "company", "working", "knowledge", "must", "qualification"
    ].into_iter().collect();

    let mut candidate_terms = BTreeSet::new();

    // Scan for multi-word or standalone technical patterns
    let clean_jd: String = job_description
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '+' || c == '#' || c == '.' || c == '-' || c == '/' { c } else { ' ' })
        .collect();

    for raw_word in clean_jd.split_whitespace() {
        let clean = raw_word.trim_matches(|c: char| !c.is_alphanumeric() && c != '+' && c != '#');
        let lower = clean.to_lowercase();
        if clean.len() >= 2 && !stop_words.contains(lower.as_str()) && !clean.chars().all(|c| c.is_numeric()) {
            candidate_terms.insert(clean.to_string());
        }
    }

    let mut matched_keywords = Vec::new();
    let mut missing_keywords = Vec::new();

    for term in candidate_terms {
        let term_lower = term.to_lowercase();
        if resume_corpus_lower.contains(&term_lower) {
            matched_keywords.push(term);
        } else {
            missing_keywords.push(term);
        }
    }

    let total = matched_keywords.len() + missing_keywords.len();
    let match_score = if total == 0 {
        100.0
    } else {
        ((matched_keywords.len() as f64 / total as f64) * 1000.0).round() / 10.0
    };

    let rating = match match_score {
        s if s >= 80.0 => "Excellent Match (Tier 1 ATS Candidate)".to_string(),
        s if s >= 65.0 => "Strong Match (Passes Most Filters)".to_string(),
        s if s >= 45.0 => "Moderate Match (Recommend Keyword Optimization)".to_string(),
        _ => "Needs Optimization (High Risk of ATS Rejection)".to_string(),
    };

    let mut recommendations = Vec::new();
    if !missing_keywords.is_empty() {
        let top_missing = missing_keywords.iter().take(8).cloned().collect::<Vec<_>>().join(", ");
        recommendations.push(format!("Inject high-impact missing keywords: {}", top_missing));
    }
    if resume.hidden_keywords.is_empty() {
        recommendations.push("Inject ATS hidden keywords using 'cvapp ats inject' to boost keyword density without visual clutter.".to_string());
    }
    if resume.experience.iter().all(|e| !e.description.contains('%') && !e.description.chars().any(|c| c.is_numeric())) {
        recommendations.push("Add quantitative impact metrics (e.g., 'increased throughput by 35%') in experience bullets.".to_string());
    }
    if resume.summary.trim().is_empty() {
        recommendations.push("Add a targeted professional summary matching the job title and core requirements.".to_string());
    }

    AtsFitAnalysis {
        match_score,
        rating,
        matched_keywords,
        missing_keywords,
        recommendations,
    }
}

pub fn set_template(store: &mut ProjectsStore, query: Option<&str>, template_str: &str) -> Result<(), String> {
    let p = find_project_mut(store, query)?;
    p.data.active_template = TemplateType::from_str(template_str);
    p.updated_at = current_timestamp_secs();
    Ok(())
}

#[derive(Default, Debug)]
pub struct UpdateStyleArgs {
    pub accent_color: Option<String>,
    pub sidebar_color: Option<String>,
    pub font_family: Option<String>,
    pub text_scale: Option<f64>,
    pub spacing_scale: Option<f64>,
    pub font_scale: Option<f64>,
    pub hide_photo: Option<bool>,
}

pub fn set_style(store: &mut ProjectsStore, query: Option<&str>, args: UpdateStyleArgs) -> Result<(), String> {
    let p = find_project_mut(store, query)?;
    if let Some(c) = args.accent_color { p.data.accent_color = c; }
    if let Some(c) = args.sidebar_color { p.data.sidebar_color = c; }
    if let Some(f) = args.font_family { p.data.font_family = f; }
    if let Some(s) = args.text_scale { p.data.text_scale = s; }
    if let Some(s) = args.spacing_scale { p.data.spacing_scale = s; }
    if let Some(s) = args.font_scale { p.data.font_scale = s; }
    if let Some(h) = args.hide_photo { p.data.hide_photo = h; }
    p.updated_at = current_timestamp_secs();
    Ok(())
}

pub fn import_linkedin(
    store: &mut ProjectsStore,
    file_path: &Path,
    custom_name: Option<&str>,
) -> Result<String, String> {
    if !file_path.exists() {
        return Err(format!("File '{}' not found.", file_path.display()));
    }

    let ext = file_path.extension().and_then(|s| s.to_str()).unwrap_or_default().to_lowercase();
    let update = if ext == "zip" {
        let bytes = fs::read(file_path).map_err(|e| e.to_string())?;
        parse_linkedin_zip(&bytes)?
    } else if ext == "json" {
        let content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
        parse_linkedin_json(&content)?
    } else {
        return Err("Unsupported file format. Provide a .zip LinkedIn archive or .json dump.".to_string());
    };

    let project_name = custom_name
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            if !update.personal_info.full_name.is_empty() {
                format!("{}'s Resume", update.personal_info.full_name)
            } else {
                "LinkedIn Import".to_string()
            }
        });

    let mut state = PersistedState::default();
    update.apply_to(&mut state.resume_data);

    let project = CvProject::new(project_name, state);
    let id = project.id.clone();
    store.projects.insert(0, project);
    store.active_project_id = Some(id.clone());
    Ok(id)
}
