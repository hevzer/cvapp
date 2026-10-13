pub mod handlers;

use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use self::handlers::{
    get_prompts_definitions, get_resources_definitions, get_tools_definitions, handle_prompt_get,
    handle_resource_read, handle_tool_call, McpContext,
};

/// Runs the Model Context Protocol (MCP) server over standard input/output.
pub fn run_stdio_server(explicit_store_path: Option<PathBuf>) -> io::Result<()> {
    eprintln!("[cvapp-mcp] Server started (JSON-RPC 2.0 stdio transport). Ready for agent commands.");

    let ctx = McpContext::new(explicit_store_path);
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let reader = stdin.lock();

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(e) => {
                eprintln!("[cvapp-mcp] Error reading from stdin: {}", e);
                break;
            }
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let parsed: Value = match serde_json::from_str(trimmed) {
            Ok(val) => val,
            Err(e) => {
                eprintln!("[cvapp-mcp] Parse error: {}", e);
                let err_resp = json!({
                    "jsonrpc": "2.0",
                    "id": Value::Null,
                    "error": {
                        "code": -32700,
                        "message": format!("Parse error: {}", e)
                    }
                });
                writeln!(stdout, "{}", err_resp)?;
                stdout.flush()?;
                continue;
            }
        };

        let id = parsed.get("id").cloned();
        let method = parsed.get("method").and_then(|m| m.as_str()).unwrap_or_default();
        let params = parsed.get("params").cloned().unwrap_or(Value::Null);

        // If no ID is present, this is a notification. Notifications do not receive responses.
        if id.is_none() {
            match method {
                "notifications/initialized" => {
                    eprintln!("[cvapp-mcp] Agent client connection established.");
                }
                "notifications/cancelled" => {
                    eprintln!("[cvapp-mcp] Received cancellation notification.");
                }
                _ => {
                    eprintln!("[cvapp-mcp] Unhandled notification: {}", method);
                }
            }
            continue;
        }

        let req_id = id.unwrap();

        let response = match method {
            "initialize" => {
                json!({
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": {
                            "tools": { "listChanged": false },
                            "resources": { "subscribe": false, "listChanged": false },
                            "prompts": { "listChanged": false }
                        },
                        "serverInfo": {
                            "name": "cvapp-mcp",
                            "version": "0.2.0"
                        }
                    }
                })
            }

            "ping" => {
                json!({
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": {}
                })
            }

            "tools/list" => {
                let tools = get_tools_definitions();
                json!({
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": {
                        "tools": tools
                    }
                })
            }

            "tools/call" => {
                let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or_default();
                let tool_args = params.get("arguments").cloned().unwrap_or(json!({}));

                match handle_tool_call(&ctx, tool_name, &tool_args) {
                    Ok(output_text) => {
                        json!({
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "result": {
                                "content": [
                                    {
                                        "type": "text",
                                        "text": output_text
                                    }
                                ],
                                "isError": false
                            }
                        })
                    }
                    Err(err_msg) => {
                        json!({
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "result": {
                                "content": [
                                    {
                                        "type": "text",
                                        "text": format!("Error: {}", err_msg)
                                    }
                                ],
                                "isError": true
                            }
                        })
                    }
                }
            }

            "resources/list" => {
                let res = get_resources_definitions(&ctx);
                json!({
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": res
                })
            }

            "resources/read" => {
                let uri = params.get("uri").and_then(|u| u.as_str()).unwrap_or_default();
                match handle_resource_read(&ctx, uri) {
                    Ok(res_val) => {
                        json!({
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "result": res_val
                        })
                    }
                    Err(err_msg) => {
                        json!({
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "error": {
                                "code": -32602,
                                "message": err_msg
                            }
                        })
                    }
                }
            }

            "prompts/list" => {
                let res = get_prompts_definitions();
                json!({
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": res
                })
            }

            "prompts/get" => {
                let prompt_name = params.get("name").and_then(|n| n.as_str()).unwrap_or_default();
                let prompt_args = params.get("arguments").cloned().unwrap_or(json!({}));
                match handle_prompt_get(&ctx, prompt_name, &prompt_args) {
                    Ok(res_val) => {
                        json!({
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "result": res_val
                        })
                    }
                    Err(err_msg) => {
                        json!({
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "error": {
                                "code": -32602,
                                "message": err_msg
                            }
                        })
                    }
                }
            }

            _ => {
                json!({
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "error": {
                        "code": -32601,
                        "message": format!("Method '{}' not found", method)
                    }
                })
            }
        };

        writeln!(stdout, "{}", response)?;
        stdout.flush()?;
    }

    eprintln!("[cvapp-mcp] Server stopped.");
    Ok(())
}
