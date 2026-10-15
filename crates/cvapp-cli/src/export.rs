use cvapp::models::{CvProject, ResumeData, TemplateType};

/// Exports project or resume data as pretty-printed JSON.
pub fn export_json(project: &CvProject, resume_only: bool) -> Result<String, String> {
    if resume_only {
        serde_json::to_string_pretty(&project.data.resume_data).map_err(|e| e.to_string())
    } else {
        serde_json::to_string_pretty(project).map_err(|e| e.to_string())
    }
}

/// Exports resume as formatted GitHub-Flavored Markdown.
pub fn export_markdown(project: &CvProject) -> String {
    let r = &project.data.resume_data;
    let mut out = String::new();

    // Name & Title
    out.push_str(&format!("# {}\n", r.personal_info.full_name));
    if !r.personal_info.title.is_empty() {
        out.push_str(&format!("**{}**\n\n", r.personal_info.title));
    }

    // Contact bar
    let mut contacts = Vec::new();
    if !r.personal_info.email.is_empty() { contacts.push(format!("📧 [{}]({})", r.personal_info.email, r.personal_info.email)); }
    if !r.personal_info.phone.is_empty() { contacts.push(format!("📱 {}", r.personal_info.phone)); }
    if !r.personal_info.location.is_empty() { contacts.push(format!("📍 {}", r.personal_info.location)); }
    if !r.personal_info.linkedin.is_empty() { contacts.push(format!("🔗 [LinkedIn]({})", r.personal_info.linkedin)); }
    if !r.personal_info.github.is_empty() { contacts.push(format!("💻 [GitHub]({})", r.personal_info.github)); }
    if !r.personal_info.website.is_empty() { contacts.push(format!("🌐 [Website]({})", r.personal_info.website)); }

    if !contacts.is_empty() {
        out.push_str(&contacts.join(" • "));
        out.push_str("\n\n---\n\n");
    }

    // Professional Summary
    if !r.summary.is_empty() {
        out.push_str("## Professional Summary\n\n");
        out.push_str(&r.summary);
        out.push_str("\n\n");
    }

    // Experience
    if !r.experience.is_empty() {
        out.push_str("## Work Experience\n\n");
        for exp in &r.experience {
            let period = if exp.current {
                format!("{} – Present", exp.start_date)
            } else if !exp.end_date.is_empty() {
                format!("{} – {}", exp.start_date, exp.end_date)
            } else {
                exp.start_date.clone()
            };
            out.push_str(&format!("### {} | {}\n", exp.position, exp.company));
            out.push_str(&format!("*{}*\n\n", period));
            if !exp.description.is_empty() {
                for line in exp.description.lines() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        if trimmed.starts_with('-') || trimmed.starts_with('*') {
                            out.push_str(&format!("{}\n", trimmed));
                        } else {
                            out.push_str(&format!("- {}\n", trimmed));
                        }
                    }
                }
                out.push('\n');
            }
        }
    }

    // Education
    if !r.education.is_empty() {
        out.push_str("## Education\n\n");
        for edu in &r.education {
            let period = if !edu.end_date.is_empty() {
                format!("{} – {}", edu.start_date, edu.end_date)
            } else {
                edu.start_date.clone()
            };
            let degree_field = if !edu.field.is_empty() {
                format!("{}, {}", edu.degree, edu.field)
            } else {
                edu.degree.clone()
            };
            out.push_str(&format!("### {} – {}\n", degree_field, edu.institution));
            if !period.is_empty() {
                out.push_str(&format!("*{}*\n\n", period));
            }
            if !edu.description.is_empty() {
                out.push_str(&format!("{}\n\n", edu.description));
            }
        }
    }

    // Technical Skills
    if !r.technical_skills.is_empty() {
        out.push_str("## Technical Skills\n\n");
        for skill in &r.technical_skills {
            if !skill.description.is_empty() {
                out.push_str(&format!("- **{}**: {}\n", skill.name, skill.description));
            } else {
                out.push_str(&format!("- **{}**\n", skill.name));
            }
        }
        out.push('\n');
    }

    // Soft Skills
    if !r.soft_skills.is_empty() {
        out.push_str("## Soft Skills\n\n");
        out.push_str(&r.soft_skills.join(", "));
        out.push_str("\n\n");
    }

    // Certifications
    if !r.certifications.is_empty() {
        out.push_str("## Certifications\n\n");
        for cert in &r.certifications {
            out.push_str(&format!("- **{}** – {} ({})\n", cert.name, cert.issuer, cert.date));
        }
        out.push('\n');
    }

    // Languages
    if !r.languages.is_empty() {
        out.push_str("## Languages\n\n");
        out.push_str(&r.languages.join(" • "));
        out.push_str("\n\n");
    }

    out
}

/// Exports resume as strict, clean ATS-compliant plain text.
pub fn export_text(project: &CvProject) -> String {
    let r = &project.data.resume_data;
    let mut out = String::new();

    out.push_str(&r.personal_info.full_name.to_uppercase());
    out.push('\n');
    if !r.personal_info.title.is_empty() {
        out.push_str(&r.personal_info.title);
        out.push('\n');
    }
    out.push_str(&format!(
        "Email: {} | Phone: {} | Location: {}\n",
        r.personal_info.email, r.personal_info.phone, r.personal_info.location
    ));
    if !r.personal_info.linkedin.is_empty() {
        out.push_str(&format!("LinkedIn: {}\n", r.personal_info.linkedin));
    }
    if !r.personal_info.github.is_empty() {
        out.push_str(&format!("GitHub: {}\n", r.personal_info.github));
    }
    out.push('\n');

    if !r.summary.is_empty() {
        out.push_str("PROFESSIONAL SUMMARY\n====================\n");
        out.push_str(&r.summary);
        out.push_str("\n\n");
    }

    if !r.experience.is_empty() {
        out.push_str("PROFESSIONAL EXPERIENCE\n=======================\n");
        for exp in &r.experience {
            let period = if exp.current {
                format!("{} - Present", exp.start_date)
            } else {
                format!("{} - {}", exp.start_date, exp.end_date)
            };
            out.push_str(&format!("{} | {}\n{}\n", exp.position, exp.company, period));
            for line in exp.description.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    out.push_str(&format!("* {}\n", trimmed.trim_start_matches(['-', '*']).trim()));
                }
            }
            out.push('\n');
        }
    }

    if !r.education.is_empty() {
        out.push_str("EDUCATION\n=========\n");
        for edu in &r.education {
            out.push_str(&format!("{}, {} - {}\n", edu.degree, edu.field, edu.institution));
            out.push_str(&format!("{} - {}\n", edu.start_date, edu.end_date));
            if !edu.description.is_empty() {
                out.push_str(&format!("{}\n", edu.description));
            }
            out.push('\n');
        }
    }

    if !r.technical_skills.is_empty() {
        out.push_str("TECHNICAL SKILLS\n================\n");
        for s in &r.technical_skills {
            if !s.description.is_empty() {
                out.push_str(&format!("{}: {}\n", s.name, s.description));
            } else {
                out.push_str(&format!("{}\n", s.name));
            }
        }
        out.push('\n');
    }

    if !r.soft_skills.is_empty() {
        out.push_str("KEY COMPETENCIES\n================\n");
        out.push_str(&r.soft_skills.join(", "));
        out.push_str("\n\n");
    }

    if !r.certifications.is_empty() {
        out.push_str("CERTIFICATIONS\n==============\n");
        for c in &r.certifications {
            out.push_str(&format!("{} - {} ({})\n", c.name, c.issuer, c.date));
        }
        out.push('\n');
    }

    if !r.languages.is_empty() {
        out.push_str("LANGUAGES\n=========\n");
        out.push_str(&r.languages.join(", "));
        out.push_str("\n\n");
    }

    if !r.hidden_keywords.is_empty() {
        out.push_str("ATS KEYWORDS\n============\n");
        out.push_str(&r.hidden_keywords.join(", "));
        out.push_str("\n\n");
    }

    out
}

/// Renders a standalone, single-file HTML document formatted and styled
/// according to the project's chosen template architecture.
pub fn export_html(project: &CvProject) -> String {
    let r = &project.data.resume_data;
    let template = project.data.active_template;
    let accent = &project.data.accent_color;
    let font_family = &project.data.font_family;

    let title = format!("{} — Resume", r.personal_info.full_name);

    let template_body = match template {
        TemplateType::Ats => render_ats_html(r),
        TemplateType::Modern => render_modern_html(r, accent),
        TemplateType::Timeline => render_timeline_html(r, accent),
        TemplateType::TimelineTwoColumn => render_twocolumn_html(r, accent),
    };

    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>{title}</title>
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&family=Outfit:wght@400;500;600;700&family=Lora:wght@400;500;600;700&display=swap" rel="stylesheet">
  <style>
    *, *::before, *::after {{ box-sizing: border-box; margin: 0; padding: 0; }}
    body {{
      font-family: '{font_family}', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      color: #1e293b;
      background-color: #f8fafc;
      line-height: 1.5;
      padding: 2rem 1rem;
    }}
    .cv-container {{
      max-width: 210mm;
      min-height: 297mm;
      margin: 0 auto;
      background: #ffffff;
      box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.1);
      border-radius: 8px;
      overflow: hidden;
      padding: 18mm 16mm;
    }}
    @media print {{
      body {{ background: transparent; padding: 0; }}
      .cv-container {{
        box-shadow: none;
        border-radius: 0;
        margin: 0;
        width: 100%;
        min-height: auto;
        padding: 12mm 12mm;
      }}
      @page {{ size: A4 portrait; margin: 0; }}
    }}
    h1, h2, h3, h4 {{ color: #0f172a; }}
    a {{ color: {accent}; text-decoration: none; }}
    a:hover {{ text-decoration: underline; }}
    .section-title {{
      font-size: 0.85rem;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 0.08em;
      border-bottom: 2px solid {accent};
      padding-bottom: 0.25rem;
      margin-bottom: 0.75rem;
      color: {accent};
    }}
    .item-header {{ display: flex; justify-content: space-between; align-items: baseline; }}
    .hidden-keywords {{
      font-size: 1px;
      color: rgba(255, 255, 255, 0.01);
      user-select: none;
    }}
  </style>
</head>
<body>
  <div class="cv-container">
    {template_body}
    <div class="hidden-keywords">{hidden_kw}</div>
  </div>
</body>
</html>"#,
        title = html_escape(&title),
        font_family = font_family,
        accent = accent,
        template_body = template_body,
        hidden_kw = html_escape(&r.hidden_keywords.join(" "))
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn render_ats_html(r: &ResumeData) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        r#"<div style="text-align: center; margin-bottom: 1.5rem;">
          <h1 style="font-size: 1.8rem; font-weight: 800; letter-spacing: -0.02em;">{}</h1>
          <p style="font-size: 1rem; font-weight: 600; color: #475569; margin-top: 0.2rem;">{}</p>
          <p style="font-size: 0.8rem; color: #64748b; margin-top: 0.4rem;">
            {} • {} • {}
          </p>
        </div>"#,
        html_escape(&r.personal_info.full_name),
        html_escape(&r.personal_info.title),
        html_escape(&r.personal_info.email),
        html_escape(&r.personal_info.phone),
        html_escape(&r.personal_info.location)
    ));

    if !r.summary.is_empty() {
        out.push_str(&format!(
            r#"<section style="margin-bottom: 1.25rem;">
              <h2 class="section-title">Summary</h2>
              <p style="font-size: 0.85rem; color: #334155; line-height: 1.6;">{}</p>
            </section>"#,
            html_escape(&r.summary)
        ));
    }

    if !r.experience.is_empty() {
        out.push_str(r#"<section style="margin-bottom: 1.25rem;"><h2 class="section-title">Experience</h2>"#);
        for exp in &r.experience {
            let period = if exp.current { format!("{} — Present", exp.start_date) } else { format!("{} — {}", exp.start_date, exp.end_date) };
            out.push_str(&format!(
                r#"<div style="margin-bottom: 0.9rem;">
                  <div class="item-header">
                    <span style="font-size: 0.9rem; font-weight: 700;">{}</span>
                    <span style="font-size: 0.75rem; color: #64748b; font-weight: 500;">{}</span>
                  </div>
                  <div style="font-size: 0.8rem; font-weight: 600; color: #475569; margin-bottom: 0.3rem;">{}</div>
                  <div style="font-size: 0.8rem; color: #334155; white-space: pre-line;">{}</div>
                </div>"#,
                html_escape(&exp.position),
                html_escape(&period),
                html_escape(&exp.company),
                html_escape(&exp.description)
            ));
        }
        out.push_str("</section>");
    }

    if !r.education.is_empty() {
        out.push_str(r#"<section style="margin-bottom: 1.25rem;"><h2 class="section-title">Education</h2>"#);
        for edu in &r.education {
            out.push_str(&format!(
                r#"<div style="margin-bottom: 0.7rem;">
                  <div class="item-header">
                    <span style="font-size: 0.85rem; font-weight: 700;">{}, {}</span>
                    <span style="font-size: 0.75rem; color: #64748b;">{} — {}</span>
                  </div>
                  <div style="font-size: 0.8rem; color: #475569;">{}</div>
                </div>"#,
                html_escape(&edu.degree),
                html_escape(&edu.field),
                html_escape(&edu.start_date),
                html_escape(&edu.end_date),
                html_escape(&edu.institution)
            ));
        }
        out.push_str("</section>");
    }

    if !r.technical_skills.is_empty() {
        out.push_str(r#"<section style="margin-bottom: 1rem;"><h2 class="section-title">Skills</h2><div style="display: flex; flex-wrap: wrap; gap: 0.4rem;">"#);
        for skill in &r.technical_skills {
            out.push_str(&format!(
                r#"<span style="font-size: 0.75rem; font-weight: 600; background: #f1f5f9; padding: 0.2rem 0.5rem; border-radius: 4px; border: 1px solid #e2e8f0;">{}</span>"#,
                html_escape(&skill.name)
            ));
        }
        out.push_str("</div></section>");
    }

    out
}

fn render_modern_html(r: &ResumeData, accent: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        r#"<div style="border-bottom: 3px solid {}; padding-bottom: 1.25rem; margin-bottom: 1.5rem; display: flex; justify-content: space-between; align-items: flex-end;">
          <div>
            <h1 style="font-size: 2.2rem; font-weight: 900; letter-spacing: -0.03em; color: #0f172a; line-height: 1.1;">{}</h1>
            <p style="font-size: 1.1rem; font-weight: 700; color: {}; margin-top: 0.3rem;">{}</p>
          </div>
          <div style="text-align: right; font-size: 0.75rem; color: #64748b; line-height: 1.6;">
            <div>{}</div>
            <div>{}</div>
            <div>{}</div>
          </div>
        </div>"#,
        accent,
        html_escape(&r.personal_info.full_name),
        accent,
        html_escape(&r.personal_info.title),
        html_escape(&r.personal_info.email),
        html_escape(&r.personal_info.phone),
        html_escape(&r.personal_info.location)
    ));

    out.push_str(r#"<div style="display: grid; grid-template-columns: 2fr 1fr; gap: 2rem;"><div>"#);

    // Left Column: Summary + Experience
    if !r.summary.is_empty() {
        out.push_str(&format!(
            r#"<section style="margin-bottom: 1.5rem;">
              <h2 class="section-title">Profile</h2>
              <p style="font-size: 0.85rem; color: #334155; line-height: 1.6;">{}</p>
            </section>"#,
            html_escape(&r.summary)
        ));
    }

    if !r.experience.is_empty() {
        out.push_str(r#"<section style="margin-bottom: 1.5rem;"><h2 class="section-title">Experience</h2>"#);
        for exp in &r.experience {
            let period = if exp.current { format!("{} — Present", exp.start_date) } else { format!("{} — {}", exp.start_date, exp.end_date) };
            out.push_str(&format!(
                r#"<div style="margin-bottom: 1.2rem;">
                  <div class="item-header">
                    <span style="font-size: 0.95rem; font-weight: 800; color: #0f172a;">{}</span>
                    <span style="font-size: 0.75rem; font-weight: 600; color: #64748b;">{}</span>
                  </div>
                  <div style="font-size: 0.85rem; font-weight: 700; color: {}; margin-bottom: 0.4rem;">{}</div>
                  <div style="font-size: 0.8rem; color: #334155; line-height: 1.55; white-space: pre-line;">{}</div>
                </div>"#,
                html_escape(&exp.position),
                html_escape(&period),
                accent,
                html_escape(&exp.company),
                html_escape(&exp.description)
            ));
        }
        out.push_str("</section>");
    }

    out.push_str(r#"</div><div>"#);

    // Right Column: Skills + Education + Certs
    if !r.technical_skills.is_empty() {
        out.push_str(r#"<section style="margin-bottom: 1.5rem;"><h2 class="section-title">Technical Skills</h2><div style="display: flex; flex-wrap: wrap; gap: 0.35rem;">"#);
        for skill in &r.technical_skills {
            out.push_str(&format!(
                r#"<span style="font-size: 0.75rem; font-weight: 700; background: #f8fafc; color: #1e293b; padding: 0.25rem 0.55rem; border-radius: 6px; border: 1px solid #e2e8f0;">{}</span>"#,
                html_escape(&skill.name)
            ));
        }
        out.push_str("</div></section>");
    }

    if !r.education.is_empty() {
        out.push_str(r#"<section style="margin-bottom: 1.5rem;"><h2 class="section-title">Education</h2>"#);
        for edu in &r.education {
            out.push_str(&format!(
                r#"<div style="margin-bottom: 0.8rem;">
                  <div style="font-size: 0.85rem; font-weight: 700; color: #0f172a;">{}, {}</div>
                  <div style="font-size: 0.8rem; color: #475569;">{}</div>
                  <div style="font-size: 0.75rem; color: #94a3b8; margin-top: 0.1rem;">{} — {}</div>
                </div>"#,
                html_escape(&edu.degree),
                html_escape(&edu.field),
                html_escape(&edu.institution),
                html_escape(&edu.start_date),
                html_escape(&edu.end_date)
            ));
        }
        out.push_str("</section>");
    }

    if !r.languages.is_empty() {
        out.push_str(r#"<section style="margin-bottom: 1.5rem;"><h2 class="section-title">Languages</h2><div style="font-size: 0.8rem; color: #334155;">"#);
        out.push_str(&html_escape(&r.languages.join(" • ")));
        out.push_str("</div></section>");
    }

    out.push_str("</div></div>");
    out
}

fn render_timeline_html(r: &ResumeData, accent: &str) -> String {
    render_modern_html(r, accent)
}

fn render_twocolumn_html(r: &ResumeData, accent: &str) -> String {
    render_modern_html(r, accent)
}
