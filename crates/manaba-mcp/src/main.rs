use anyhow::Result;
use manaba_sdk::{Client, Cookie};
use rmcp::{
    ServerHandler,
    model::{
        CallToolRequestMethod, CallToolRequestParam, CallToolResult, Content, Implementation,
        InitializeResult, ListResourcesResult, ListToolsResult, PaginatedRequestParam, ServerCapabilities, Tool,
        ToolsCapability,
    },
    service::{RequestContext, RoleServer, serve_server},
    transport::io::stdio,
};
use serde_json::json;
use std::{borrow::Cow, sync::Arc};
use tokio::sync::OnceCell;

static CLIENT: OnceCell<Client> = OnceCell::const_new();

async fn get_client() -> Result<&'static Client> {
    CLIENT
        .get_or_try_init(|| async {
            let base_url = "https://ct.ritsumei.ac.jp";
            let cookie = Cookie::load(base_url)?;
            Client::new(base_url, &cookie)
                .await
                .map_err(anyhow::Error::from)
        })
        .await
}

#[derive(Clone, Debug)]
struct ManabaServer;

impl ServerHandler for ManabaServer {
    async fn list_tools(
        &self,
        _request: PaginatedRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, rmcp::Error> {
        let tools = vec![
            Tool {
                name: Cow::Borrowed("list_courses"),
                description: Cow::Borrowed("List all available courses"),
                input_schema: Arc::new(
                    json!({
                        "type": "object",
                        "properties": {},
                        "required": []
                    })
                    .as_object()
                    .unwrap()
                    .clone(),
                ),
            },
            Tool {
                name: Cow::Borrowed("list_reports"),
                description: Cow::Borrowed("List reports for a specific course"),
                input_schema: Arc::new(
                    json!({
                        "type": "object",
                        "properties": {
                            "course_id": {
                                "type": "string",
                                "description": "The course ID to get reports for"
                            }
                        },
                        "required": ["course_id"]
                    })
                    .as_object()
                    .unwrap()
                    .clone(),
                ),
            },
            Tool {
                name: Cow::Borrowed("list_exams"),
                description: Cow::Borrowed("List exams for a specific course"),
                input_schema: Arc::new(
                    json!({
                        "type": "object",
                        "properties": {
                            "course_id": {
                                "type": "string",
                                "description": "The course ID to get exams for"
                            }
                        },
                        "required": ["course_id"]
                    })
                    .as_object()
                    .unwrap()
                    .clone(),
                ),
            },
            Tool {
                name: Cow::Borrowed("list_all_assignments"),
                description: Cow::Borrowed(
                    "List all assignments (reports and exams) across all courses",
                ),
                input_schema: Arc::new(
                    json!({
                        "type": "object",
                        "properties": {},
                        "required": []
                    })
                    .as_object()
                    .unwrap()
                    .clone(),
                ),
            },
        ];

        Ok(ListToolsResult {
            tools,
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, rmcp::Error> {
        let client = get_client().await.map_err(|e| {
            rmcp::Error::internal_error(format!("Failed to get client: {}", e), None)
        })?;

        let result = match request.name.as_ref() {
            "list_courses" => {
                let courses = client.get_courses().await.map_err(|e| {
                    rmcp::Error::internal_error(format!("Failed to get courses: {}", e), None)
                })?;
                let result = json!({
                    "courses": courses.iter().map(|course| {
                        json!({
                            "id": course.id,
                            "title": course.title
                        })
                    }).collect::<Vec<_>>()
                });
                serde_json::to_string_pretty(&result).map_err(|e| {
                    rmcp::Error::internal_error(format!("Serialization error: {}", e), None)
                })?
            }
            "list_reports" => {
                let course_id = request
                    .arguments
                    .as_ref()
                    .and_then(|obj| obj.get("course_id"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| rmcp::Error::invalid_params("course_id is required", None))?;

                let courses = client.get_courses().await.map_err(|e| {
                    rmcp::Error::internal_error(format!("Failed to get courses: {}", e), None)
                })?;
                let course = courses
                    .iter()
                    .find(|c| c.id == course_id)
                    .ok_or_else(|| rmcp::Error::invalid_params("Course not found", None))?;

                let reports = client.get_reports(course).await.map_err(|e| {
                    rmcp::Error::internal_error(format!("Failed to get reports: {}", e), None)
                })?;
                let result = json!({
                    "course": {
                        "id": course.id,
                        "title": course.title
                    },
                    "reports": reports.iter().map(|report| {
                        json!({
                            "title": report.title,
                            "submit_state": format!("{:?}", report.submit_state),
                            "receptible_state": format!("{:?}", report.receptiable_state),
                            "start_date": report.start_date.as_ref().map(|d| d.date.format("%Y-%m-%d %H:%M").to_string()),
                            "due_date": report.due_date.as_ref().map(|d| d.date.format("%Y-%m-%d %H:%M").to_string()),
                            "importance_level": report.due_date.as_ref().map(|d| format!("{:?}", d.importance_level))
                        })
                    }).collect::<Vec<_>>()
                });
                serde_json::to_string_pretty(&result).map_err(|e| {
                    rmcp::Error::internal_error(format!("Serialization error: {}", e), None)
                })?
            }
            "list_exams" => {
                let course_id = request
                    .arguments
                    .as_ref()
                    .and_then(|obj| obj.get("course_id"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| rmcp::Error::invalid_params("course_id is required", None))?;

                let courses = client.get_courses().await.map_err(|e| {
                    rmcp::Error::internal_error(format!("Failed to get courses: {}", e), None)
                })?;
                let course = courses
                    .iter()
                    .find(|c| c.id == course_id)
                    .ok_or_else(|| rmcp::Error::invalid_params("Course not found", None))?;

                let exams = client.get_exams(course).await.map_err(|e| {
                    rmcp::Error::internal_error(format!("Failed to get exams: {}", e), None)
                })?;
                let result = json!({
                    "course": {
                        "id": course.id,
                        "title": course.title
                    },
                    "exams": exams.iter().map(|exam| {
                        json!({
                            "title": exam.title,
                            "submit_state": format!("{:?}", exam.submit_state),
                            "receptible_state": format!("{:?}", exam.receptiable_state),
                            "start_date": exam.start_date.as_ref().map(|d| d.date.format("%Y-%m-%d %H:%M").to_string()),
                            "due_date": exam.due_date.as_ref().map(|d| d.date.format("%Y-%m-%d %H:%M").to_string()),
                            "importance_level": exam.due_date.as_ref().map(|d| format!("{:?}", d.importance_level))
                        })
                    }).collect::<Vec<_>>()
                });
                serde_json::to_string_pretty(&result).map_err(|e| {
                    rmcp::Error::internal_error(format!("Serialization error: {}", e), None)
                })?
            }
            "list_all_assignments" => {
                let courses = client.get_courses().await.map_err(|e| {
                    rmcp::Error::internal_error(format!("Failed to get courses: {}", e), None)
                })?;
                let mut all_assignments = Vec::new();

                for course in &courses {
                    let reports = client.get_reports(course).await.map_err(|e| {
                        rmcp::Error::internal_error(format!("Failed to get reports: {}", e), None)
                    })?;
                    for report in reports {
                        all_assignments.push(json!({
                                "type": "report",
                                "course_id": course.id,
                                "course_title": course.title,
                                "title": report.title,
                                "submit_state": format!("{:?}", report.submit_state),
                                "receptible_state": format!("{:?}", report.receptiable_state),
                                "start_date": report.start_date.as_ref().map(|d| d.date.format("%Y-%m-%d %H:%M").to_string()),
                                "due_date": report.due_date.as_ref().map(|d| d.date.format("%Y-%m-%d %H:%M").to_string()),
                                "importance_level": report.due_date.as_ref().map(|d| format!("{:?}", d.importance_level))
                            }));
                    }

                    let exams = client.get_exams(course).await.map_err(|e| {
                        rmcp::Error::internal_error(format!("Failed to get exams: {}", e), None)
                    })?;
                    for exam in exams {
                        all_assignments.push(json!({
                                "type": "exam",
                                "course_id": course.id,
                                "course_title": course.title,
                                "title": exam.title,
                                "submit_state": format!("{:?}", exam.submit_state),
                                "receptible_state": format!("{:?}", exam.receptiable_state),
                                "start_date": exam.start_date.as_ref().map(|d| d.date.format("%Y-%m-%d %H:%M").to_string()),
                                "due_date": exam.due_date.as_ref().map(|d| d.date.format("%Y-%m-%d %H:%M").to_string()),
                                "importance_level": exam.due_date.as_ref().map(|d| format!("{:?}", d.importance_level))
                            }));
                    }
                }

                let result = json!({
                    "assignments": all_assignments
                });
                serde_json::to_string_pretty(&result).map_err(|e| {
                    rmcp::Error::internal_error(format!("Serialization error: {}", e), None)
                })?
            }
            _ => {
                return Err(rmcp::Error::method_not_found::<CallToolRequestMethod>());
            }
        };

        Ok(CallToolResult {
            content: vec![Content::text(result)],
            is_error: None,
        })
    }

    async fn list_resources(
        &self,
        _request: PaginatedRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, rmcp::Error> {
        // Return empty resources list since we don't provide any resources
        Ok(ListResourcesResult {
            resources: vec![],
            next_cursor: None,
        })
    }

    fn get_info(&self) -> InitializeResult {
        InitializeResult {
            protocol_version: Default::default(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability::default()),
                ..Default::default()
            },
            server_info: Implementation {
                name: "manaba-mcp".to_string(),
                version: "0.1.0".to_string(),
            },
            instructions: None,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("Starting manaba MCP server...");

    let server = ManabaServer;
    let transport = stdio();

    eprintln!("Server initialized, starting to serve...");

    // This should run indefinitely, handling MCP requests
    let result = serve_server(server, transport).await;

    eprintln!("Server finished with result: {:?}", result);

    result?;
    Ok(())
}
