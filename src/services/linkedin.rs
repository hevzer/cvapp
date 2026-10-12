use crate::models::{
    generate_id, Education, Experience, PersonalInfo, ResumeData, TechnicalSkill,
};
use serde_json::Value;
use std::io::{Cursor, Read};

pub fn parse_linkedin_json(content: &str) -> Result<PartialResumeUpdate, String> {
    let json: Value = serde_json::from_str(content).map_err(|e| e.to_string())?;
    extract_from_json_value(&json)
}

pub fn parse_linkedin_zip(bytes: &[u8]) -> Result<PartialResumeUpdate, String> {
    let reader = Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(reader).map_err(|e| e.to_string())?;

    let mut profile_val: Option<Value> = None;
    let mut positions_val: Option<Value> = None;
    let mut education_val: Option<Value> = None;
    let mut skills_val: Option<Value> = None;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let name = match file.name() {
            Ok(n) => n.to_lowercase(),
            Err(_) => continue,
        };
        if file.is_dir() {
            continue;
        }

        let mut content = String::new();
        if file.read_to_string(&mut content).is_ok() {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                if name.contains("profile") {
                    profile_val = Some(json);
                } else if name.contains("position") {
                    positions_val = Some(json);
                } else if name.contains("education") {
                    education_val = Some(json);
                } else if name.contains("skill") {
                    skills_val = Some(json);
                }
            }
        }
    }

    let mut update = PartialResumeUpdate::default();

    if let Some(pv) = profile_val {
        let p_obj = if pv.is_array() {
            pv.get(0).cloned().unwrap_or(Value::Null)
        } else {
            pv
        };
        update.personal_info.full_name = get_str_any(
            &p_obj,
            &["fullName", "name", "First Name", "Last Name"],
        );
        let first = get_str(&p_obj, "First Name");
        let last = get_str(&p_obj, "Last Name");
        if !first.is_empty() && !last.is_empty() {
            update.personal_info.full_name = format!("{} {}", first, last);
        }
        update.personal_info.title = get_str_any(
            &p_obj,
            &["Headline", "headline", "title", "Title"],
        );
        update.personal_info.email = get_str_any(
            &p_obj,
            &["Email Address", "email", "emailAddress"],
        );
        update.personal_info.location = get_str_any(&p_obj, &["Location", "location", "address"]);
        update.personal_info.linkedin = get_str_any(
            &p_obj,
            &["Profile URL", "url", "linkedin", "linkedIn"],
        );
        update.personal_info.website = get_str_any(&p_obj, &["Website", "website", "portfolio"]);
        update.summary = get_str_any(&p_obj, &["Summary", "summary", "About", "about"]);
    }

    if let Some(Value::Array(positions)) = positions_val {
        for p in positions {
            let company = get_str_any(&p, &["Company Name", "company", "companyName"]);
            let pos = get_str_any(&p, &["Title", "title", "position"]);
            let start = get_str_any(&p, &["Started On", "startDate"]);
            let end = get_str_any(&p, &["Finished On", "endDate"]);
            let desc = get_str_any(&p, &["Description", "description"]);
            if !company.is_empty() || !pos.is_empty() {
                update.experience.push(Experience {
                    id: generate_id(),
                    company,
                    position: pos,
                    start_date: start,
                    end_date: end.clone(),
                    current: end.is_empty(),
                    description: desc,
                });
            }
        }
    }

    if let Some(Value::Array(educations)) = education_val {
        for e in educations {
            let inst = get_str_any(&e, &["School Name", "school", "institution"]);
            let deg = get_str_any(&e, &["Degree Name", "degree"]);
            let fld = get_str_any(&e, &["Field Of Study", "field", "fieldOfStudy"]);
            let start = get_str_any(&e, &["Start Date", "startDate"]);
            let end = get_str_any(&e, &["End Date", "endDate"]);
            let desc = get_str_any(&e, &["Notes", "description", "activities"]);
            if !inst.is_empty() || !deg.is_empty() {
                update.education.push(Education {
                    id: generate_id(),
                    institution: inst,
                    degree: deg,
                    field: fld,
                    start_date: start,
                    end_date: end,
                    description: desc,
                });
            }
        }
    }

    if let Some(Value::Array(skills)) = skills_val {
        for s in skills {
            let skill_name = if let Some(str_val) = s.as_str() {
                str_val.to_string()
            } else {
                get_str_any(&s, &["Name", "name", "Skill", "skill"])
            };
            if !skill_name.is_empty() {
                update.technical_skills.push(TechnicalSkill {
                    id: generate_id(),
                    name: skill_name,
                    description: String::new(),
                });
            }
        }
    }

    Ok(update)
}

fn extract_from_json_value(json: &Value) -> Result<PartialResumeUpdate, String> {
    let mut update = PartialResumeUpdate::default();

    let full_name = get_str_any(json, &["fullName", "name"]);
    let first = get_str(json, "firstName");
    let last = get_str(json, "lastName");
    update.personal_info.full_name = if !first.is_empty() || !last.is_empty() {
        format!("{} {}", first, last).trim().to_string()
    } else {
        full_name
    };

    update.personal_info.title = get_str_any(json, &["headline", "title"]);
    update.personal_info.email = get_str_any(json, &["email", "emailAddress"]);
    update.personal_info.location = get_str_any(json, &["location", "address"]);
    update.personal_info.linkedin = get_str_any(json, &["linkedin", "linkedIn"]);
    update.personal_info.website = get_str_any(json, &["website", "portfolio"]);
    update.summary = get_str_any(json, &["summary", "about", "objective"]);

    if let Some(positions) = json
        .get("positions")
        .or_else(|| json.get("experience"))
        .and_then(|v| v.as_array())
    {
        for p in positions {
            let company = get_str_any(p, &["companyName", "company"]);
            let pos = get_str_any(p, &["title", "position"]);
            let start = get_date_str(p, "startDate");
            let end = get_date_str(p, "endDate");
            let is_curr = p.get("isCurrent")
                .or_else(|| p.get("current"))
                .and_then(|v| v.as_bool())
                .unwrap_or(end.is_empty());
            let desc = get_str(p, "description");
            update.experience.push(Experience {
                id: generate_id(),
                company,
                position: pos,
                start_date: start,
                end_date: end,
                current: is_curr,
                description: desc,
            });
        }
    }

    if let Some(educations) = json.get("education").and_then(|v| v.as_array()) {
        for e in educations {
            let inst = get_str_any(e, &["schoolName", "school", "institution"]);
            let deg = get_str_any(e, &["degreeName", "degree"]);
            let fld = get_str_any(e, &["fieldOfStudy", "field"]);
            let start = get_date_str(e, "startDate");
            let end = get_date_str(e, "endDate");
            let desc = get_str_any(e, &["description", "notes"]);
            update.education.push(Education {
                id: generate_id(),
                institution: inst,
                degree: deg,
                field: fld,
                start_date: start,
                end_date: end,
                description: desc,
            });
        }
    }

    if let Some(skills) = json.get("skills").and_then(|v| v.as_array()) {
        for s in skills {
            let name = if let Some(txt) = s.as_str() {
                txt.to_string()
            } else {
                get_str_any(s, &["name", "Name", "skill", "Skill"])
            };
            if !name.is_empty() {
                update.technical_skills.push(TechnicalSkill {
                    id: generate_id(),
                    name,
                    description: String::new(),
                });
            }
        }
    }

    Ok(update)
}

fn get_date_str(val: &Value, key: &str) -> String {
    if let Some(v) = val.get(key) {
        if let Some(s) = v.as_str() {
            return s.to_string();
        }
        if let Some(obj) = v.as_object() {
            let year = obj.get("year").and_then(|y| y.as_i64()).unwrap_or(0);
            let month = obj.get("month").and_then(|m| m.as_i64()).unwrap_or(0);
            if year > 0 {
                if month > 0 {
                    return format!("{:04}-{:02}", year, month);
                } else {
                    return format!("{:04}", year);
                }
            }
        }
    }
    String::new()
}

fn get_str(val: &Value, key: &str) -> String {
    val.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

fn get_str_any(val: &Value, keys: &[&str]) -> String {
    for k in keys {
        if let Some(v) = val.get(*k).and_then(|v| v.as_str()) {
            if !v.is_empty() {
                return v.to_string();
            }
        }
    }
    String::new()
}

#[derive(Clone, Debug, Default)]
pub struct PartialResumeUpdate {
    pub personal_info: PersonalInfo,
    pub summary: String,
    pub experience: Vec<Experience>,
    pub education: Vec<Education>,
    pub technical_skills: Vec<TechnicalSkill>,
}

impl PartialResumeUpdate {
    pub fn apply_to(self, current: &mut ResumeData) {
        if !self.personal_info.full_name.is_empty() {
            current.personal_info.full_name = self.personal_info.full_name;
        }
        if !self.personal_info.title.is_empty() {
            current.personal_info.title = self.personal_info.title;
        }
        if !self.personal_info.email.is_empty() {
            current.personal_info.email = self.personal_info.email;
        }
        if !self.personal_info.location.is_empty() {
            current.personal_info.location = self.personal_info.location;
        }
        if !self.personal_info.linkedin.is_empty() {
            current.personal_info.linkedin = self.personal_info.linkedin;
        }
        if !self.personal_info.website.is_empty() {
            current.personal_info.website = self.personal_info.website;
        }
        if !self.summary.is_empty() {
            current.summary = self.summary;
        }
        if !self.experience.is_empty() {
            current.experience = self.experience;
        }
        if !self.education.is_empty() {
            current.education = self.education;
        }
        if !self.technical_skills.is_empty() {
            current.technical_skills = self.technical_skills;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_linkedin_json_positions_and_skills() {
        let json_str = r#"{
            "firstName": "John",
            "lastName": "Doe",
            "headline": "Senior Rust Engineer",
            "summary": "Rust enthusiast and systems architect",
            "positions": [
                {
                    "companyName": "Acme Corp",
                    "title": "Lead Dev",
                    "description": "Building microservices",
                    "startDate": { "year": 2021, "month": 3 },
                    "endDate": null,
                    "isCurrent": true
                }
            ],
            "skills": [
                { "name": "Rust" },
                { "name": "WebAssembly" }
            ]
        }"#;

        let update = parse_linkedin_json(json_str).expect("Should parse successfully");
        assert_eq!(update.personal_info.full_name, "John Doe");
        assert_eq!(update.personal_info.title, "Senior Rust Engineer");
        assert_eq!(update.summary, "Rust enthusiast and systems architect");
        assert_eq!(update.experience.len(), 1);
        assert_eq!(update.experience[0].company, "Acme Corp");
        assert_eq!(update.experience[0].start_date, "2021-03");
        assert!(update.experience[0].current);
        assert_eq!(update.technical_skills.len(), 2);
        assert_eq!(update.technical_skills[0].name, "Rust");
        assert_eq!(update.technical_skills[1].name, "WebAssembly");
    }
}

