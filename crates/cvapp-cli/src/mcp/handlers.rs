use cvapp::models::TechnicalSkill;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

use crate::export::{export_html, export_json, export_markdown, export_text};
use crate::ops::{
    add_certification, add_education, add_experience, analyze_job_fit, clear_ats_keywords,
    create_project, delete_project, duplicate_project, import_linkedin, inject_ats_keywords,
    list_projects, rename_project, set_active_project, set_languages, set_skills, set_style,
    set_summary, set_template, update_personal_info, UpdateProfileArgs, UpdateStyleArgs,
};
use crate::storage::{find_project, load_store, save_store};

pub struct McpContext {
    pub explicit_store_path: Option<PathBuf>,
}

impl McpContext {
    pub fn new(explicit_store_path: Option<PathBuf>) -> Self {
        Self { explicit_store_path }
    }

    fn get_store_path(&self) -> Option<&Path> {
        self.explicit_store_path.as_deref()
    }
}

pub fn get_tools_definitions() -> Value {
    json!([
        {
            "name": "cvapp_list_projects",
            "description": "Lists all resumes and projects in the CVapp database with their ID, name, candidate name, target role, template, and active status.",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_get_project",
            "description": "Retrieves the full project data and resume content for a specified project ID or name (or active resume if omitted).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": {
                        "type": "string",
                        "description": "Project ID or name. If omitted, the currently active project is returned."
                    }
                },
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_create_project",
            "description": "Creates a new resume project, optionally populated with example data.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name for the new resume project (e.g., 'Senior Rust Engineer', 'Google Application')."
                    },
                    "use_example_data": {
                        "type": "boolean",
                        "description": "Whether to initialize with pre-filled realistic example career data."
                    }
                },
                "required": ["name"],
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_duplicate_project",
            "description": "Duplicates an existing resume into a new project, perfect for tailoring variations for specific job applications.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": {
                        "type": "string",
                        "description": "ID or name of the project to duplicate. Defaults to active project."
                    },
                    "new_name": {
                        "type": "string",
                        "description": "Custom name for the duplicated project (defaults to original name with ' (Copy)')."
                    }
                },
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_rename_project",
            "description": "Renames an existing resume project.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": {
                        "type": "string",
                        "description": "ID or name of the project to rename."
                    },
                    "new_name": {
                        "type": "string",
                        "description": "New project name."
                    }
                },
                "required": ["project_id", "new_name"],
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_delete_project",
            "description": "Deletes a resume project by ID or name.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": {
                        "type": "string",
                        "description": "ID or name of the project to delete."
                    }
                },
                "required": ["project_id"],
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_set_active_project",
            "description": "Switches the currently active resume project.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": {
                        "type": "string",
                        "description": "ID or name of the project to activate."
                    }
                },
                "required": ["project_id"],
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_update_personal_info",
            "description": "Updates personal contact information and professional title in a resume.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string", "description": "Project ID or name (defaults to active project)." },
                    "full_name": { "type": "string", "description": "Candidate's full name." },
                    "title": { "type": "string", "description": "Professional title / headline (e.g. 'Staff Software Engineer')." },
                    "email": { "type": "string", "description": "Contact email address." },
                    "phone": { "type": "string", "description": "Phone number." },
                    "location": { "type": "string", "description": "Location (e.g., 'San Francisco, CA' or 'Paris, France')." },
                    "linkedin": { "type": "string", "description": "LinkedIn profile URL or username." },
                    "github": { "type": "string", "description": "GitHub profile URL or username." },
                    "website": { "type": "string", "description": "Personal portfolio or website URL." },
                    "objective": { "type": "string", "description": "Short career objective statement." }
                },
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_update_summary",
            "description": "Updates the professional executive summary section of a resume.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string", "description": "Project ID or name (defaults to active project)." },
                    "summary": { "type": "string", "description": "Professional summary text." }
                },
                "required": ["summary"],
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_add_experience",
            "description": "Adds a work experience entry to the resume.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string", "description": "Project ID or name." },
                    "company": { "type": "string", "description": "Company or organization name." },
                    "position": { "type": "string", "description": "Job title / position held." },
                    "start_date": { "type": "string", "description": "Start date (e.g., 'Jan 2022')." },
                    "end_date": { "type": "string", "description": "End date (e.g., 'Dec 2024'). Leave empty if current role." },
                    "current": { "type": "boolean", "description": "True if this is the current job." },
                    "description": { "type": "string", "description": "Accomplishments and responsibilities, bulleted or multi-line." }
                },
                "required": ["company", "position", "start_date", "description"],
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_add_education",
            "description": "Adds an education degree or program to the resume.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string", "description": "Project ID or name." },
                    "institution": { "type": "string", "description": "School, college, or university name." },
                    "degree": { "type": "string", "description": "Degree type (e.g., 'B.S.', 'M.Sc.', 'Ph.D.')." },
                    "field": { "type": "string", "description": "Field of study (e.g., 'Computer Science')." },
                    "start_date": { "type": "string", "description": "Start date (e.g., '2018')." },
                    "end_date": { "type": "string", "description": "Graduation date (e.g., '2022')." },
                    "description": { "type": "string", "description": "Optional honors, coursework, or activities." }
                },
                "required": ["institution", "degree", "field", "start_date"],
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_set_skills",
            "description": "Updates technical and soft skills on the resume.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string", "description": "Project ID or name." },
                    "technical_skills": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "name": { "type": "string" },
                                "description": { "type": "string" }
                            },
                            "required": ["name"]
                        },
                        "description": "List of technical skills with optional details/category."
                    },
                    "soft_skills": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "List of soft skills or core competencies."
                    }
                },
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_add_certification",
            "description": "Adds a professional certification or license.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string", "description": "Project ID or name." },
                    "name": { "type": "string", "description": "Certification name (e.g., 'AWS Certified Solutions Architect')." },
                    "issuer": { "type": "string", "description": "Issuing organization (e.g., 'Amazon Web Services')." },
                    "date": { "type": "string", "description": "Issue date (e.g., '2023')." },
                    "url": { "type": "string", "description": "Verification URL." }
                },
                "required": ["name", "issuer", "date"],
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_set_languages",
            "description": "Sets the candidate's spoken/written languages.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string", "description": "Project ID or name." },
                    "languages": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "List of languages with proficiencies (e.g. ['English (Native)', 'French (C1)']).'}"
                    }
                },
                "required": ["languages"],
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_inject_ats_keywords",
            "description": "Injects hidden ATS keywords into the resume to maximize parsing score on applicant tracking systems without cluttering visual layout.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string", "description": "Project ID or name." },
                    "keywords": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "List of keywords to inject (e.g. ['Kubernetes', 'CI/CD', 'Rust', 'GraphQL', 'Distributed Systems'])."
                    }
                },
                "required": ["keywords"],
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_clear_ats_keywords",
            "description": "Clears all hidden ATS keywords from the resume.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string", "description": "Project ID or name." }
                },
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_analyze_job_fit",
            "description": "Analyzes how well the resume matches a target job description, computing an ATS match score (0-100%), matched keywords, missing keywords, and recommendations.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string", "description": "Project ID or name." },
                    "job_description": { "type": "string", "description": "Full text of the target job description / posting." }
                },
                "required": ["job_description"],
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_set_template",
            "description": "Changes the visual template architecture and styling parameters of a resume.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string", "description": "Project ID or name." },
                    "template": {
                        "type": "string",
                        "enum": ["ats", "modern", "timeline", "timelineTwoColumn"],
                        "description": "Template layout style: 'ats' (semantic single column), 'modern' (two column with color header), 'timeline' (timeline dots), 'timelineTwoColumn' (dark sidebar)."
                    },
                    "accent_color": { "type": "string", "description": "Hex color code for primary accents (e.g. '#6366f1', '#0ea5e9')." },
                    "font_family": { "type": "string", "description": "Font family name (e.g. 'Inter', 'Outfit', 'Lora')." }
                },
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_export_resume",
            "description": "Exports a resume formatted as Markdown, ATS plain text, standalone HTML, or raw JSON.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string", "description": "Project ID or name." },
                    "format": {
                        "type": "string",
                        "enum": ["markdown", "text", "html", "json"],
                        "description": "Export format: 'markdown' (clean MD), 'text' (ATS plain text), 'html' (standalone styled web page), 'json' (full data)."
                    }
                },
                "required": ["format"],
                "additionalProperties": false
            }
        },
        {
            "name": "cvapp_import_linkedin",
            "description": "Imports a LinkedIn profile archive (.zip file or .json dump) directly into a new CVapp project.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "file_path": { "type": "string", "description": "Path to the LinkedIn .zip archive or .json file." },
                    "project_name": { "type": "string", "description": "Optional name for the new imported project." }
                },
                "required": ["file_path"],
                "additionalProperties": false
            }
        }
    ])
}

pub fn handle_tool_call(ctx: &McpContext, name: &str, args: &Value) -> Result<String, String> {
    let (path, mut store) = load_store(ctx.get_store_path())?;

    let res = match name {
        "cvapp_list_projects" => {
            let projects = list_projects(&store);
            serde_json::to_string_pretty(&projects).map_err(|e| e.to_string())?
        }

        "cvapp_get_project" => {
            let query = args.get("project_id").and_then(|v| v.as_str());
            let project = find_project(&store, query)?;
            serde_json::to_string_pretty(project).map_err(|e| e.to_string())?
        }

        "cvapp_create_project" => {
            let name = args
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing required parameter 'name'.".to_string())?;
            let use_example = args
                .get("use_example_data")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let new_id = create_project(&mut store, name, use_example);
            save_store(&path, &store)?;
            format!("Created project '{}' with ID: {}", name, new_id)
        }

        "cvapp_duplicate_project" => {
            let query = args.get("project_id").and_then(|v| v.as_str());
            let new_name = args.get("new_name").and_then(|v| v.as_str());

            let new_id = duplicate_project(&mut store, query, new_name)?;
            save_store(&path, &store)?;
            format!("Duplicated project into new ID: {}", new_id)
        }

        "cvapp_rename_project" => {
            let query = args
                .get("project_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing 'project_id' parameter.".to_string())?;
            let new_name = args
                .get("new_name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing 'new_name' parameter.".to_string())?;

            rename_project(&mut store, Some(query), new_name)?;
            save_store(&path, &store)?;
            format!("Renamed project to '{}'", new_name)
        }

        "cvapp_delete_project" => {
            let query = args
                .get("project_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing 'project_id' parameter.".to_string())?;

            delete_project(&mut store, query)?;
            save_store(&path, &store)?;
            format!("Deleted project '{}'", query)
        }

        "cvapp_set_active_project" => {
            let query = args
                .get("project_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing 'project_id' parameter.".to_string())?;

            let id = set_active_project(&mut store, query)?;
            save_store(&path, &store)?;
            format!("Activated project: {}", id)
        }

        "cvapp_update_personal_info" => {
            let query = args.get("project_id").and_then(|v| v.as_str());
            let p_args = UpdateProfileArgs {
                full_name: args.get("full_name").and_then(|v| v.as_str()).map(|s| s.to_string()),
                title: args.get("title").and_then(|v| v.as_str()).map(|s| s.to_string()),
                email: args.get("email").and_then(|v| v.as_str()).map(|s| s.to_string()),
                phone: args.get("phone").and_then(|v| v.as_str()).map(|s| s.to_string()),
                location: args.get("location").and_then(|v| v.as_str()).map(|s| s.to_string()),
                linkedin: args.get("linkedin").and_then(|v| v.as_str()).map(|s| s.to_string()),
                github: args.get("github").and_then(|v| v.as_str()).map(|s| s.to_string()),
                website: args.get("website").and_then(|v| v.as_str()).map(|s| s.to_string()),
                objective: args.get("objective").and_then(|v| v.as_str()).map(|s| s.to_string()),
                driving_license: None,
            };

            update_personal_info(&mut store, query, p_args)?;
            save_store(&path, &store)?;
            "Updated personal information successfully.".to_string()
        }

        "cvapp_update_summary" => {
            let query = args.get("project_id").and_then(|v| v.as_str());
            let summary = args
                .get("summary")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing 'summary' parameter.".to_string())?;

            set_summary(&mut store, query, summary)?;
            save_store(&path, &store)?;
            "Updated professional summary successfully.".to_string()
        }

        "cvapp_add_experience" => {
            let query = args.get("project_id").and_then(|v| v.as_str());
            let company = args.get("company").and_then(|v| v.as_str()).unwrap_or_default();
            let position = args.get("position").and_then(|v| v.as_str()).unwrap_or_default();
            let start = args.get("start_date").and_then(|v| v.as_str()).unwrap_or_default();
            let end = args.get("end_date").and_then(|v| v.as_str());
            let current = args.get("current").and_then(|v| v.as_bool()).unwrap_or(false);
            let desc = args.get("description").and_then(|v| v.as_str()).unwrap_or_default();

            let id = add_experience(&mut store, query, company, position, start, end, current, desc)?;
            save_store(&path, &store)?;
            format!("Added experience entry: ID {}", id)
        }

        "cvapp_add_education" => {
            let query = args.get("project_id").and_then(|v| v.as_str());
            let inst = args.get("institution").and_then(|v| v.as_str()).unwrap_or_default();
            let deg = args.get("degree").and_then(|v| v.as_str()).unwrap_or_default();
            let field = args.get("field").and_then(|v| v.as_str()).unwrap_or_default();
            let start = args.get("start_date").and_then(|v| v.as_str()).unwrap_or_default();
            let end = args.get("end_date").and_then(|v| v.as_str());
            let desc = args.get("description").and_then(|v| v.as_str());

            let id = add_education(&mut store, query, inst, deg, field, start, end, desc)?;
            save_store(&path, &store)?;
            format!("Added education entry: ID {}", id)
        }

        "cvapp_set_skills" => {
            let query = args.get("project_id").and_then(|v| v.as_str());
            let tech = args.get("technical_skills").and_then(|v| v.as_array()).map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        let name = item.get("name").and_then(|n| n.as_str())?;
                        let desc = item.get("description").and_then(|d| d.as_str()).unwrap_or_default();
                        Some(TechnicalSkill {
                            id: cvapp::models::generate_id(),
                            name: name.to_string(),
                            description: desc.to_string(),
                        })
                    })
                    .collect()
            });

            let soft = args.get("soft_skills").and_then(|v| v.as_array()).map(|arr| {
                arr.iter()
                    .filter_map(|item| item.as_str().map(|s| s.to_string()))
                    .collect()
            });

            set_skills(&mut store, query, tech, soft)?;
            save_store(&path, &store)?;
            "Updated skills successfully.".to_string()
        }

        "cvapp_add_certification" => {
            let query = args.get("project_id").and_then(|v| v.as_str());
            let name = args.get("name").and_then(|v| v.as_str()).unwrap_or_default();
            let issuer = args.get("issuer").and_then(|v| v.as_str()).unwrap_or_default();
            let date = args.get("date").and_then(|v| v.as_str()).unwrap_or_default();
            let url = args.get("url").and_then(|v| v.as_str());

            let id = add_certification(&mut store, query, name, issuer, date, url)?;
            save_store(&path, &store)?;
            format!("Added certification: ID {}", id)
        }

        "cvapp_set_languages" => {
            let query = args.get("project_id").and_then(|v| v.as_str());
            let langs: Vec<String> = args
                .get("languages")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|i| i.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();

            set_languages(&mut store, query, langs)?;
            save_store(&path, &store)?;
            "Updated languages successfully.".to_string()
        }

        "cvapp_inject_ats_keywords" => {
            let query = args.get("project_id").and_then(|v| v.as_str());
            let kw: Vec<String> = args
                .get("keywords")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|i| i.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();

            let count = inject_ats_keywords(&mut store, query, kw)?;
            save_store(&path, &store)?;
            format!("Injected {} new ATS keywords.", count)
        }

        "cvapp_clear_ats_keywords" => {
            let query = args.get("project_id").and_then(|v| v.as_str());
            clear_ats_keywords(&mut store, query)?;
            save_store(&path, &store)?;
            "Cleared ATS keywords.".to_string()
        }

        "cvapp_analyze_job_fit" => {
            let query = args.get("project_id").and_then(|v| v.as_str());
            let jd = args
                .get("job_description")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing 'job_description' parameter.".to_string())?;

            let project = find_project(&store, query)?;
            let analysis = analyze_job_fit(project, jd);
            serde_json::to_string_pretty(&analysis).map_err(|e| e.to_string())?
        }

        "cvapp_set_template" => {
            let query = args.get("project_id").and_then(|v| v.as_str());
            if let Some(tpl) = args.get("template").and_then(|v| v.as_str()) {
                set_template(&mut store, query, tpl)?;
            }

            let style_args = UpdateStyleArgs {
                accent_color: args.get("accent_color").and_then(|v| v.as_str()).map(|s| s.to_string()),
                font_family: args.get("font_family").and_then(|v| v.as_str()).map(|s| s.to_string()),
                ..Default::default()
            };
            set_style(&mut store, query, style_args)?;
            save_store(&path, &store)?;
            "Updated template and styling.".to_string()
        }

        "cvapp_export_resume" => {
            let query = args.get("project_id").and_then(|v| v.as_str());
            let format_str = args
                .get("format")
                .and_then(|v| v.as_str())
                .unwrap_or("markdown");

            let project = find_project(&store, query)?;
            match format_str {
                "markdown" => export_markdown(project),
                "text" => export_text(project),
                "html" => export_html(project),
                "json" => export_json(project, false)?,
                other => return Err(format!("Unknown format: '{}'", other)),
            }
        }

        "cvapp_import_linkedin" => {
            let file_path = args
                .get("file_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing 'file_path' parameter.".to_string())?;
            let custom_name = args.get("project_name").and_then(|v| v.as_str());

            let new_id = import_linkedin(&mut store, Path::new(file_path), custom_name)?;
            save_store(&path, &store)?;
            format!("Imported LinkedIn profile into new project ID: {}", new_id)
        }

        other => return Err(format!("Unknown tool: {}", other)),
    };

    Ok(res)
}

pub fn get_resources_definitions(ctx: &McpContext) -> Value {
    let mut resources = vec![
        json!({
            "uri": "cvapp://templates",
            "name": "CVapp Templates Guide",
            "description": "Architectures and guidelines for ATS Minimalist, Modern Creative, Timeline Creative, and Two-Column Timeline layouts.",
            "mimeType": "text/markdown"
        })
    ];

    if let Ok((_, store)) = load_store(ctx.get_store_path()) {
        resources.push(json!({
            "uri": "cvapp://projects",
            "name": "Active CVapp Projects Store",
            "description": "Complete list of resume projects with IDs and candidate metadata.",
            "mimeType": "application/json"
        }));

        for p in &store.projects {
            resources.push(json!({
                "uri": format!("cvapp://projects/{}", p.id),
                "name": format!("Resume: {}", p.name),
                "description": format!("Live resume data for candidate {} ({})", p.candidate_name(), p.target_role()),
                "mimeType": "text/markdown"
            }));
        }
    }

    json!({ "resources": resources })
}

pub fn handle_resource_read(ctx: &McpContext, uri: &str) -> Result<Value, String> {
    if uri == "cvapp://templates" {
        let content = r#"# CVapp Template Architecture Guide

1. **ATS Minimalist (`ats`)**
   - Single-column, strict semantic HTML hierarchy.
   - 100% compliant with Applicant Tracking Systems (Workday, Taleo, Greenhouse, iCIMS).
   - Recommended for enterprise, government, finance, and online portal submissions.

2. **Modern Creative (`modern`)**
   - Two-column header with color accent bar.
   - Clean profile, experience timeline, and right sidebar for skills and education.
   - Best for software engineering, tech startups, product management, and creative roles.

3. **Timeline Creative (`timeline`)**
   - Vertical timeline connectors highlighting career progression.
   - Compact multi-column distribution fitting single-page A4.

4. **Two-Column Timeline (`timelineTwoColumn`)**
   - Dark left sidebar with contact details, photo, and skill badges.
   - Clean main area with structured timeline experience.
"#;
        return Ok(json!({
            "contents": [{
                "uri": uri,
                "mimeType": "text/markdown",
                "text": content
            }]
        }));
    }

    let (_, store) = load_store(ctx.get_store_path())?;

    if uri == "cvapp://projects" {
        let projects = list_projects(&store);
        let text = serde_json::to_string_pretty(&projects).map_err(|e| e.to_string())?;
        return Ok(json!({
            "contents": [{
                "uri": uri,
                "mimeType": "application/json",
                "text": text
            }]
        }));
    }

    if let Some(id) = uri.strip_prefix("cvapp://projects/") {
        let project = find_project(&store, Some(id))?;
        let md = export_markdown(project);
        return Ok(json!({
            "contents": [{
                "uri": uri,
                "mimeType": "text/markdown",
                "text": md
            }]
        }));
    }

    Err(format!("Resource '{}' not found.", uri))
}

pub fn get_prompts_definitions() -> Value {
    json!({
        "prompts": [
            {
                "name": "tailor_resume_for_job",
                "description": "Guides the agent to analyze a target job description and tailor a resume's summary, experience, and ATS keywords to maximize interview conversion.",
                "arguments": [
                    {
                        "name": "job_description",
                        "description": "Full text of the target job description",
                        "required": true
                    },
                    {
                        "name": "project_id",
                        "description": "Project ID or name to tailor (or active resume if omitted)",
                        "required": false
                    }
                ]
            }
        ]
    })
}

pub fn handle_prompt_get(ctx: &McpContext, name: &str, args: &Value) -> Result<Value, String> {
    if name == "tailor_resume_for_job" {
        let jd = args.get("job_description").and_then(|v| v.as_str()).unwrap_or_default();
        let query = args.get("project_id").and_then(|v| v.as_str());

        let (_, store) = load_store(ctx.get_store_path())?;
        let project = find_project(&store, query)?;
        let current_resume = export_markdown(project);

        let prompt_message = format!(
            "Please analyze the target job description against the candidate's current resume, perform ATS keyword analysis, and suggest or apply specific improvements:\n\n### TARGET JOB DESCRIPTION:\n{}\n\n### CURRENT RESUME ({}):\n{}\n\n### GOAL:\n1. Run cvapp_analyze_job_fit to identify missing keywords.\n2. Duplicate this project (using cvapp_duplicate_project) so the base resume remains intact.\n3. Inject the top missing keywords (using cvapp_inject_ats_keywords).\n4. Rewrite the professional summary and relevant experience bullet points to emphasize matching impact and technologies.",
            jd, project.name, current_resume
        );

        return Ok(json!({
            "messages": [
                {
                    "role": "user",
                    "content": {
                        "type": "text",
                        "text": prompt_message
                    }
                }
            ]
        }));
    }

    Err(format!("Prompt '{}' not found.", name))
}
