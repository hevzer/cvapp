use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use cvapp::models::TechnicalSkill;
use std::fs;
use std::path::{Path, PathBuf};

use crate::export::{export_html, export_json, export_markdown, export_text};
use crate::ops::{
    add_certification, add_education, add_experience, add_technical_skill, analyze_job_fit,
    clear_ats_keywords, create_project, delete_project, duplicate_project, import_linkedin,
    inject_ats_keywords, list_projects, rename_project, set_active_project, set_languages,
    set_skills, set_style, set_summary, set_template, update_personal_info, UpdateProfileArgs,
    UpdateStyleArgs,
};
use crate::storage::{find_project, load_store, save_store};

#[derive(Parser, Debug)]
#[command(
    name = "cvapp",
    version = "0.2.0",
    about = "CVapp CLI for AI Agents & Developers",
    long_about = "A high-performance CLI tool for inspecting, modifying, ATS-optimizing, and exporting professional CVs and resumes."
)]
pub struct Cli {
    /// Custom path to the CVapp database JSON file (.cvapp/projects.json or resume.json)
    #[arg(short = 'f', long = "file", global = true)]
    pub file: Option<PathBuf>,

    /// Output responses in machine-readable JSON format
    #[arg(long = "json", global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage resume projects and profiles
    Project(ProjectArgs),

    /// Edit resume content, experience, education, and skills
    Resume(ResumeArgs),

    /// Manage ATS keywords, analyze match fit, and score against job descriptions
    Ats(AtsArgs),

    /// Change template layout architecture and visual styling
    Template(TemplateArgs),

    /// Export resume to Markdown, ATS plain text, standalone HTML, or JSON
    Export(ExportArgs),

    /// Import profile archives from LinkedIn or raw JSON
    Import(ImportArgs),
}

// ======================== PROJECT COMMANDS ========================

#[derive(Args, Debug)]
pub struct ProjectArgs {
    #[command(subcommand)]
    pub command: ProjectSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum ProjectSubcommand {
    /// List all resume projects
    List,

    /// Show metadata and summary for a project
    Show {
        /// Project ID or name (defaults to active project)
        id_or_name: Option<String>,
    },

    /// Create a new resume project
    Create {
        /// Project name (e.g. 'Software Engineer - Backend')
        name: String,
        /// Initialize with realistic example career data
        #[arg(long)]
        example: bool,
    },

    /// Duplicate a project to tailor variations for specific job postings
    Duplicate {
        /// Project ID or name to duplicate (defaults to active project)
        id_or_name: Option<String>,
        /// Custom name for the duplicated project
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Rename an existing project
    Rename {
        /// Project ID or name to rename
        id_or_name: String,
        /// New project name
        new_name: String,
    },

    /// Delete a project
    Delete {
        /// Project ID or name to delete
        id_or_name: String,
    },

    /// Select and switch the active working project
    Select {
        /// Project ID or name to activate
        id_or_name: String,
    },
}

// ======================== RESUME COMMANDS ========================

#[derive(Args, Debug)]
pub struct ResumeArgs {
    #[command(subcommand)]
    pub command: ResumeSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum ResumeSubcommand {
    /// Display resume content in terminal
    Show {
        /// Project ID or name (defaults to active project)
        id_or_name: Option<String>,
        /// Output as GitHub-flavored Markdown (default)
        #[arg(long)]
        markdown: bool,
        /// Output as ATS-friendly plain text
        #[arg(long)]
        text: bool,
    },

    /// Update personal contact information and title
    SetProfile {
        /// Project ID or name (defaults to active project)
        #[arg(short = 'p', long = "project")]
        id_or_name: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        phone: Option<String>,
        #[arg(long)]
        location: Option<String>,
        #[arg(long)]
        linkedin: Option<String>,
        #[arg(long)]
        github: Option<String>,
        #[arg(long)]
        website: Option<String>,
        #[arg(long)]
        objective: Option<String>,
    },

    /// Update the professional summary
    SetSummary {
        /// Project ID or name (defaults to active project)
        #[arg(short = 'p', long = "project")]
        id_or_name: Option<String>,
        /// Executive summary text
        text: String,
    },

    /// Add a work experience entry
    AddExperience {
        /// Project ID or name (defaults to active project)
        #[arg(short = 'p', long = "project")]
        id_or_name: Option<String>,
        #[arg(long)]
        company: String,
        #[arg(long)]
        position: String,
        #[arg(long)]
        start: String,
        #[arg(long)]
        end: Option<String>,
        #[arg(long)]
        current: bool,
        #[arg(long)]
        desc: String,
    },

    /// Add an education entry
    AddEducation {
        /// Project ID or name (defaults to active project)
        #[arg(short = 'p', long = "project")]
        id_or_name: Option<String>,
        #[arg(long)]
        institution: String,
        #[arg(long)]
        degree: String,
        #[arg(long)]
        field: String,
        #[arg(long)]
        start: String,
        #[arg(long)]
        end: Option<String>,
        #[arg(long)]
        desc: Option<String>,
    },

    /// Add a single technical skill
    AddSkill {
        /// Project ID or name (defaults to active project)
        #[arg(short = 'p', long = "project")]
        id_or_name: Option<String>,
        #[arg(long)]
        name: String,
        #[arg(long)]
        desc: Option<String>,
    },

    /// Batch update technical and soft skills (comma-separated)
    SetSkills {
        /// Project ID or name (defaults to active project)
        #[arg(short = 'p', long = "project")]
        id_or_name: Option<String>,
        /// Comma-separated list of technical skills (e.g. 'Rust, TypeScript, Docker')
        #[arg(long)]
        tech: Option<String>,
        /// Comma-separated list of soft skills (e.g. 'Leadership, Mentorship, Communication')
        #[arg(long)]
        soft: Option<String>,
    },

    /// Add a certification
    AddCert {
        /// Project ID or name (defaults to active project)
        #[arg(short = 'p', long = "project")]
        id_or_name: Option<String>,
        #[arg(long)]
        name: String,
        #[arg(long)]
        issuer: String,
        #[arg(long)]
        date: String,
        #[arg(long)]
        url: Option<String>,
    },

    /// Set languages spoken
    SetLanguages {
        /// Project ID or name (defaults to active project)
        #[arg(short = 'p', long = "project")]
        id_or_name: Option<String>,
        /// Comma-separated list of languages (e.g. 'English (Native), French (Fluent)')
        languages: Vec<String>,
    },
}

// ======================== ATS COMMANDS ========================

#[derive(Args, Debug)]
pub struct AtsArgs {
    #[command(subcommand)]
    pub command: AtsSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum AtsSubcommand {
    /// List all hidden ATS keywords currently injected into the resume
    List {
        /// Project ID or name (defaults to active project)
        id_or_name: Option<String>,
    },

    /// Inject hidden ATS keywords to increase match density
    Inject {
        /// Project ID or name (defaults to active project)
        #[arg(short = 'p', long = "project")]
        id_or_name: Option<String>,
        /// Keywords to inject
        keywords: Vec<String>,
    },

    /// Clear all hidden ATS keywords
    Clear {
        /// Project ID or name (defaults to active project)
        id_or_name: Option<String>,
    },

    /// Analyze resume match against a job description
    Analyze {
        /// Project ID or name (defaults to active project)
        #[arg(short = 'p', long = "project")]
        id_or_name: Option<String>,
        /// Direct job description text
        #[arg(long = "job-text")]
        job_text: Option<String>,
        /// Path to a text or markdown file containing the job posting
        #[arg(long = "job-file")]
        job_file: Option<PathBuf>,
    },
}

// ======================== TEMPLATE COMMANDS ========================

#[derive(Args, Debug)]
pub struct TemplateArgs {
    #[command(subcommand)]
    pub command: TemplateSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum TemplateSubcommand {
    /// Set template layout architecture
    Set {
        /// Project ID or name (defaults to active project)
        #[arg(short = 'p', long = "project")]
        id_or_name: Option<String>,
        /// Template architecture ('ats', 'modern', 'timeline', 'timelineTwoColumn')
        template: String,
    },

    /// Customize visual styling parameters
    Style {
        /// Project ID or name (defaults to active project)
        #[arg(short = 'p', long = "project")]
        id_or_name: Option<String>,
        #[arg(long = "accent")]
        accent_color: Option<String>,
        #[arg(long = "font")]
        font_family: Option<String>,
        #[arg(long = "text-scale")]
        text_scale: Option<f64>,
        #[arg(long = "spacing-scale")]
        spacing_scale: Option<f64>,
    },
}

// ======================== EXPORT COMMANDS ========================

#[derive(Args, Debug)]
pub struct ExportArgs {
    #[command(subcommand)]
    pub command: ExportSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum ExportSubcommand {
    /// Export project or resume as JSON
    Json {
        /// Project ID or name (defaults to active project)
        id_or_name: Option<String>,
        /// Output file path (prints to stdout if omitted)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Only export the resumeData payload
        #[arg(long)]
        resume_only: bool,
    },

    /// Export as clean GitHub-Flavored Markdown
    Markdown {
        /// Project ID or name (defaults to active project)
        id_or_name: Option<String>,
        /// Output file path (prints to stdout if omitted)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Export as ATS-compliant plain text
    Text {
        /// Project ID or name (defaults to active project)
        id_or_name: Option<String>,
        /// Output file path (prints to stdout if omitted)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Export as standalone, styled HTML (suitable for browser printing to PDF)
    Html {
        /// Project ID or name (defaults to active project)
        id_or_name: Option<String>,
        /// Output file path (prints to stdout if omitted)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

// ======================== IMPORT COMMANDS ========================

#[derive(Args, Debug)]
pub struct ImportArgs {
    #[command(subcommand)]
    pub command: ImportSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum ImportSubcommand {
    /// Import LinkedIn archive (.zip or .json)
    Linkedin {
        /// Path to LinkedIn .zip archive or .json dump
        file: PathBuf,
        /// Custom name for the imported project
        #[arg(short, long)]
        name: Option<String>,
    },
}

// ======================== DISPATCHER ========================

pub fn run_cli(cli: Cli) -> Result<(), String> {
    let explicit_path = cli.file.as_deref();

    match cli.command {
        Commands::Project(args) => match args.command {
            ProjectSubcommand::List => {
                let (_, store) = load_store(explicit_path)?;
                let list = list_projects(&store);
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&list).unwrap());
                } else {
                    println!("\n{}", "CVapp Resume Projects:".bold().cyan());
                    println!("{:-<80}", "");
                    for p in list {
                        let active_tag = if p.is_active {
                            " [ACTIVE]".green().bold()
                        } else {
                            "".normal()
                        };
                        println!(
                            "• {} (ID: {}){}",
                            p.name.bold(),
                            p.id.dimmed(),
                            active_tag
                        );
                        println!(
                            "  Candidate: {} | Role: {} | Template: {}",
                            p.candidate_name.yellow(),
                            p.target_role,
                            p.template.magenta()
                        );
                    }
                    println!("{:-<80}\n", "");
                }
            }

            ProjectSubcommand::Show { id_or_name } => {
                let (_, store) = load_store(explicit_path)?;
                let p = find_project(&store, id_or_name.as_deref())?;
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(p).unwrap());
                } else {
                    println!("\n{} {}", "Project:".bold().cyan(), p.name.bold());
                    println!("ID: {}", p.id.dimmed());
                    println!("Candidate: {}", p.candidate_name().yellow());
                    println!("Target Role: {}", p.target_role());
                    println!("Template: {}", p.template_display_name().magenta());
                    println!("Experience entries: {}", p.data.resume_data.experience.len());
                    println!("Technical skills: {}", p.data.resume_data.technical_skills.len());
                    println!("ATS Keywords: {}", p.data.resume_data.hidden_keywords.len());
                    println!();
                }
            }

            ProjectSubcommand::Create { name, example } => {
                let (path, mut store) = load_store(explicit_path)?;
                let new_id = create_project(&mut store, &name, example);
                save_store(&path, &store)?;
                println!("{} Created project '{}' (ID: {})", "✓".green().bold(), name, new_id.dimmed());
            }

            ProjectSubcommand::Duplicate { id_or_name, name } => {
                let (path, mut store) = load_store(explicit_path)?;
                let new_id = duplicate_project(&mut store, id_or_name.as_deref(), name.as_deref())?;
                save_store(&path, &store)?;
                println!("{} Duplicated project into new ID: {}", "✓".green().bold(), new_id.dimmed());
            }

            ProjectSubcommand::Rename { id_or_name, new_name } => {
                let (path, mut store) = load_store(explicit_path)?;
                rename_project(&mut store, Some(&id_or_name), &new_name)?;
                save_store(&path, &store)?;
                println!("{} Renamed project to '{}'", "✓".green().bold(), new_name);
            }

            ProjectSubcommand::Delete { id_or_name } => {
                let (path, mut store) = load_store(explicit_path)?;
                delete_project(&mut store, &id_or_name)?;
                save_store(&path, &store)?;
                println!("{} Deleted project '{}'", "✓".green().bold(), id_or_name);
            }

            ProjectSubcommand::Select { id_or_name } => {
                let (path, mut store) = load_store(explicit_path)?;
                let active_id = set_active_project(&mut store, &id_or_name)?;
                save_store(&path, &store)?;
                println!("{} Activated project ID: {}", "✓".green().bold(), active_id.dimmed());
            }
        },

        Commands::Resume(args) => match args.command {
            ResumeSubcommand::Show { id_or_name, text, .. } => {
                let (_, store) = load_store(explicit_path)?;
                let p = find_project(&store, id_or_name.as_deref())?;
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&p.data.resume_data).unwrap());
                } else if text {
                    println!("{}", export_text(p));
                } else {
                    println!("{}", export_markdown(p));
                }
            }

            ResumeSubcommand::SetProfile { id_or_name, name, title, email, phone, location, linkedin, github, website, objective } => {
                let (path, mut store) = load_store(explicit_path)?;
                update_personal_info(&mut store, id_or_name.as_deref(), UpdateProfileArgs {
                    full_name: name,
                    title,
                    email,
                    phone,
                    location,
                    linkedin,
                    github,
                    website,
                    objective,
                    driving_license: None,
                })?;
                save_store(&path, &store)?;
                println!("{} Updated candidate profile.", "✓".green().bold());
            }

            ResumeSubcommand::SetSummary { id_or_name, text } => {
                let (path, mut store) = load_store(explicit_path)?;
                set_summary(&mut store, id_or_name.as_deref(), &text)?;
                save_store(&path, &store)?;
                println!("{} Updated professional summary.", "✓".green().bold());
            }

            ResumeSubcommand::AddExperience { id_or_name, company, position, start, end, current, desc } => {
                let (path, mut store) = load_store(explicit_path)?;
                let id = add_experience(&mut store, id_or_name.as_deref(), &company, &position, &start, end.as_deref(), current, &desc)?;
                save_store(&path, &store)?;
                println!("{} Added experience: {} at {} (ID: {})", "✓".green().bold(), position, company, id.dimmed());
            }

            ResumeSubcommand::AddEducation { id_or_name, institution, degree, field, start, end, desc } => {
                let (path, mut store) = load_store(explicit_path)?;
                let id = add_education(&mut store, id_or_name.as_deref(), &institution, &degree, &field, &start, end.as_deref(), desc.as_deref())?;
                save_store(&path, &store)?;
                println!("{} Added education: {}, {} (ID: {})", "✓".green().bold(), degree, field, id.dimmed());
            }

            ResumeSubcommand::AddSkill { id_or_name, name, desc } => {
                let (path, mut store) = load_store(explicit_path)?;
                let id = add_technical_skill(&mut store, id_or_name.as_deref(), &name, desc.as_deref())?;
                save_store(&path, &store)?;
                println!("{} Added technical skill '{}' (ID: {})", "✓".green().bold(), name, id.dimmed());
            }

            ResumeSubcommand::SetSkills { id_or_name, tech, soft } => {
                let (path, mut store) = load_store(explicit_path)?;
                let tech_vec = tech.map(|s| {
                    s.split(',')
                        .map(|item| TechnicalSkill {
                            id: cvapp::models::generate_id(),
                            name: item.trim().to_string(),
                            description: String::new(),
                        })
                        .filter(|t| !t.name.is_empty())
                        .collect()
                });
                let soft_vec = soft.map(|s| {
                    s.split(',')
                        .map(|item| item.trim().to_string())
                        .filter(|str_item| !str_item.is_empty())
                        .collect()
                });

                set_skills(&mut store, id_or_name.as_deref(), tech_vec, soft_vec)?;
                save_store(&path, &store)?;
                println!("{} Updated skills.", "✓".green().bold());
            }

            ResumeSubcommand::AddCert { id_or_name, name, issuer, date, url } => {
                let (path, mut store) = load_store(explicit_path)?;
                let id = add_certification(&mut store, id_or_name.as_deref(), &name, &issuer, &date, url.as_deref())?;
                save_store(&path, &store)?;
                println!("{} Added certification '{}' (ID: {})", "✓".green().bold(), name, id.dimmed());
            }

            ResumeSubcommand::SetLanguages { id_or_name, languages } => {
                let (path, mut store) = load_store(explicit_path)?;
                let flat: Vec<String> = languages
                    .iter()
                    .flat_map(|s| s.split(',').map(|i| i.trim().to_string()))
                    .filter(|s| !s.is_empty())
                    .collect();
                set_languages(&mut store, id_or_name.as_deref(), flat)?;
                save_store(&path, &store)?;
                println!("{} Updated languages.", "✓".green().bold());
            }
        },

        Commands::Ats(args) => match args.command {
            AtsSubcommand::List { id_or_name } => {
                let (_, store) = load_store(explicit_path)?;
                let p = find_project(&store, id_or_name.as_deref())?;
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&p.data.resume_data.hidden_keywords).unwrap());
                } else {
                    println!("\n{} ({} keywords):", "ATS Hidden Keywords".bold().cyan(), p.data.resume_data.hidden_keywords.len());
                    if p.data.resume_data.hidden_keywords.is_empty() {
                        println!("  {}", "(None injected yet. Use 'cvapp ats inject <keywords>' to add.)".dimmed());
                    } else {
                        for kw in &p.data.resume_data.hidden_keywords {
                            println!("  • {}", kw);
                        }
                    }
                    println!();
                }
            }

            AtsSubcommand::Inject { id_or_name, keywords } => {
                let (path, mut store) = load_store(explicit_path)?;
                let flat: Vec<String> = keywords
                    .iter()
                    .flat_map(|s| s.split(',').map(|i| i.trim().to_string()))
                    .filter(|s| !s.is_empty())
                    .collect();

                let count = inject_ats_keywords(&mut store, id_or_name.as_deref(), flat)?;
                save_store(&path, &store)?;
                println!("{} Injected {} new ATS keywords.", "✓".green().bold(), count);
            }

            AtsSubcommand::Clear { id_or_name } => {
                let (path, mut store) = load_store(explicit_path)?;
                clear_ats_keywords(&mut store, id_or_name.as_deref())?;
                save_store(&path, &store)?;
                println!("{} Cleared all ATS hidden keywords.", "✓".green().bold());
            }

            AtsSubcommand::Analyze { id_or_name, job_text, job_file } => {
                let jd = match (job_text, job_file) {
                    (Some(t), _) => t,
                    (None, Some(f)) => fs::read_to_string(&f)
                        .map_err(|e| format!("Failed to read job file '{}': {}", f.display(), e))?,
                    (None, None) => return Err("Provide either --job-text or --job-file.".to_string()),
                };

                let (_, store) = load_store(explicit_path)?;
                let p = find_project(&store, id_or_name.as_deref())?;
                let analysis = analyze_job_fit(p, &jd);

                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&analysis).unwrap());
                } else {
                    println!("\n{}", "=== ATS Resume Match Analysis ===".bold().cyan());
                    let score_color = if analysis.match_score >= 80.0 {
                        format!("{}%", analysis.match_score).green().bold()
                    } else if analysis.match_score >= 60.0 {
                        format!("{}%", analysis.match_score).yellow().bold()
                    } else {
                        format!("{}%", analysis.match_score).red().bold()
                    };

                    println!("Match Score: {}", score_color);
                    println!("Rating:      {}", analysis.rating.bold());
                    println!("\nMatched Keywords ({}):", analysis.matched_keywords.len().to_string().green());
                    println!("  {}", analysis.matched_keywords.join(", ").dimmed());

                    println!("\nMissing Keywords ({}):", analysis.missing_keywords.len().to_string().red());
                    let preview_missing = analysis.missing_keywords.iter().take(15).cloned().collect::<Vec<_>>().join(", ");
                    println!("  {}", preview_missing.yellow());

                    println!("\nRecommendations:");
                    for rec in &analysis.recommendations {
                        println!("  {} {}", "→".cyan(), rec);
                    }
                    println!();
                }
            }
        },

        Commands::Template(args) => match args.command {
            TemplateSubcommand::Set { id_or_name, template } => {
                let (path, mut store) = load_store(explicit_path)?;
                set_template(&mut store, id_or_name.as_deref(), &template)?;
                save_store(&path, &store)?;
                println!("{} Set template to '{}'.", "✓".green().bold(), template);
            }

            TemplateSubcommand::Style { id_or_name, accent_color, font_family, text_scale, spacing_scale } => {
                let (path, mut store) = load_store(explicit_path)?;
                set_style(&mut store, id_or_name.as_deref(), UpdateStyleArgs {
                    accent_color,
                    font_family,
                    text_scale,
                    spacing_scale,
                    ..Default::default()
                })?;
                save_store(&path, &store)?;
                println!("{} Updated template styling parameters.", "✓".green().bold());
            }
        },

        Commands::Export(args) => match args.command {
            ExportSubcommand::Json { id_or_name, output, resume_only } => {
                let (_, store) = load_store(explicit_path)?;
                let p = find_project(&store, id_or_name.as_deref())?;
                let content = export_json(p, resume_only)?;
                write_or_print(output.as_deref(), &content)?;
            }

            ExportSubcommand::Markdown { id_or_name, output } => {
                let (_, store) = load_store(explicit_path)?;
                let p = find_project(&store, id_or_name.as_deref())?;
                let content = export_markdown(p);
                write_or_print(output.as_deref(), &content)?;
            }

            ExportSubcommand::Text { id_or_name, output } => {
                let (_, store) = load_store(explicit_path)?;
                let p = find_project(&store, id_or_name.as_deref())?;
                let content = export_text(p);
                write_or_print(output.as_deref(), &content)?;
            }

            ExportSubcommand::Html { id_or_name, output } => {
                let (_, store) = load_store(explicit_path)?;
                let p = find_project(&store, id_or_name.as_deref())?;
                let content = export_html(p);
                write_or_print(output.as_deref(), &content)?;
            }
        },

        Commands::Import(args) => match args.command {
            ImportSubcommand::Linkedin { file, name } => {
                let (path, mut store) = load_store(explicit_path)?;
                let new_id = import_linkedin(&mut store, &file, name.as_deref())?;
                save_store(&path, &store)?;
                println!("{} Successfully imported LinkedIn archive! New Project ID: {}", "✓".green().bold(), new_id.dimmed());
            }
        },
    }

    Ok(())
}

fn write_or_print(output_path: Option<&Path>, content: &str) -> Result<(), String> {
    if let Some(path) = output_path {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(path, content).map_err(|e| format!("Failed to write to '{}': {}", path.display(), e))?;
        println!("{} Exported to '{}'", "✓".green().bold(), path.display());
    } else {
        println!("{}", content);
    }
    Ok(())
}
