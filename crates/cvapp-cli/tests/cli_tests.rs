use cvapp_cli::{export, mcp, ops, storage};
use serde_json::json;
use std::fs;

#[test]
fn test_storage_and_ops_lifecycle() {
    let temp_dir = std::env::temp_dir().join(format!("cvapp_test_{}", uuid::Uuid::new_v4()));
    let _ = fs::create_dir_all(&temp_dir);
    let db_path = temp_dir.join("projects.json");

    // 1. Initial load seeds example project
    let (resolved_path, mut store) = storage::load_store(Some(&db_path)).unwrap();
    assert_eq!(resolved_path, db_path);
    assert_eq!(store.projects.len(), 1);
    let initial_id = store.projects[0].id.clone();
    assert_eq!(store.active_project_id, Some(initial_id.clone()));

    // 2. Create second project
    let new_id = ops::create_project(&mut store, "Tailored Google Resume", false);
    assert_eq!(store.projects.len(), 2);
    assert_eq!(store.active_project_id, Some(new_id.clone()));

    // 3. Update profile
    ops::update_personal_info(
        &mut store,
        Some(&new_id),
        ops::UpdateProfileArgs {
            full_name: Some("Jane Doe".to_string()),
            title: Some("Staff Distributed Systems Engineer".to_string()),
            email: Some("jane.doe@example.com".to_string()),
            ..Default::default()
        },
    ).unwrap();

    let p = storage::find_project(&store, Some(&new_id)).unwrap();
    assert_eq!(p.data.resume_data.personal_info.full_name, "Jane Doe");
    assert_eq!(p.data.resume_data.personal_info.title, "Staff Distributed Systems Engineer");

    // 4. Inject ATS keywords
    let added = ops::inject_ats_keywords(
        &mut store,
        Some(&new_id),
        vec!["Kubernetes".to_string(), "Rust".to_string(), "kubernetes".to_string()],
    ).unwrap();
    assert_eq!(added, 2);

    let p = storage::find_project(&store, Some(&new_id)).unwrap();
    assert_eq!(p.data.resume_data.hidden_keywords.len(), 2);

    // 5. Test ATS job fit analysis
    let analysis = ops::analyze_job_fit(p, "Looking for Staff Distributed Systems Engineer with Kubernetes experience");
    assert!(analysis.match_score > 60.0);
    assert!(analysis.matched_keywords.contains(&"Kubernetes".to_string()));

    // 6. Test Exporters
    let md = export::export_markdown(p);
    assert!(md.contains("# Jane Doe"));
    assert!(md.contains("Staff Distributed Systems Engineer"));

    let txt = export::export_text(p);
    assert!(txt.contains("JANE DOE"));

    let html = export::export_html(p);
    assert!(html.contains("Jane Doe"));
    assert!(html.contains("Kubernetes Rust"));

    // 7. Save and reload
    storage::save_store(&db_path, &store).unwrap();
    let (_, reloaded) = storage::load_store(Some(&db_path)).unwrap();
    assert_eq!(reloaded.projects.len(), 2);

    // 8. Test MCP tools directly
    let ctx = mcp::handlers::McpContext::new(Some(db_path.clone()));
    let list_json = mcp::handlers::handle_tool_call(&ctx, "cvapp_list_projects", &json!({})).unwrap();
    assert!(list_json.contains("Jane Doe"));

    let get_json = mcp::handlers::handle_tool_call(&ctx, "cvapp_get_project", &json!({ "project_id": new_id })).unwrap();
    assert!(get_json.contains("Staff Distributed Systems Engineer"));

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
}
