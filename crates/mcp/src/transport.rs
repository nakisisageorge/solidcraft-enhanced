//! HTTP, SSE, and WebSocket transports for the SolidCraft MCP server.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    body::Body,
    extract::{State, WebSocketUpgrade, Query, Path},
    http::{header, HeaderMap, HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response, Sse, EventStream},
    routing::{get, post, any},
    Json, Router,
};
use futures::{stream, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::sync::{broadcast, mpsc, Mutex, RwLock};
use tokio_stream::wrappers::BroadcastStream;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tower_http::compression::CompressionLayer;
use uuid::Uuid;

use crate::server::{Server, PROTOCOL_VERSION, SUPPORTED_VERSIONS};
use crate::backend::Backend;
use crate::options::Options;

/// Shared state for the HTTP MCP server
#[derive(Clone)]
pub struct HttpMcpState {
    pub server: Arc<Mutex<Server>>,
    pub sessions: Arc<RwLock<HashMap<String, SessionState>>>,
    pub event_tx: broadcast::Sender<Value>,
    pub options: Options,
}

/// Session state for HTTP/SSE connections
#[derive(Clone)]
pub struct SessionState {
    pub id: String,
    pub initialized: bool,
    pub scope: Option<String>,
    pub last_activity: std::time::Instant,
}

/// Request/response types for HTTP MCP
#[derive(Deserialize)]
pub struct InitializeRequest {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: Option<String>,
    pub capabilities: Option<Value>,
    #[serde(rename = "clientInfo")]
    pub client_info: Option<Value>,
}

#[derive(Deserialize)]
pub struct ToolCallRequest {
    pub name: String,
    pub arguments: Option<Value>,
}

#[derive(Serialize)]
pub struct InitializeResponse {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    pub capabilities: Value,
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
    pub instructions: String,
}

#[derive(Serialize)]
pub struct ServerInfo {
    pub name: String,
    pub title: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct ToolsListResponse {
    pub tools: Vec<Value>,
}

#[derive(Serialize)]
pub struct ToolCallResponse {
    pub content: Vec<Value>,
    #[serde(rename = "isError")]
    pub is_error: Option<bool>,
}

#[derive(Serialize)]
pub struct ResourcesListResponse {
    pub resources: Vec<Value>,
}

#[derive(Serialize)]
pub struct ResourceReadResponse {
    pub contents: Vec<ResourceContent>,
}

#[derive(Serialize)]
pub struct ResourceContent {
    pub uri: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub text: String,
}

/// Create the HTTP MCP router
pub fn create_router(state: HttpMcpState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any)
        .allow_credentials(false);

    Router::new()
        .route("/mcp", post(handle_mcp_request))
        .route("/mcp/initialize", post(handle_initialize))
        .route("/mcp/tools/list", get(handle_tools_list))
        .route("/mcp/tools/call", post(handle_tool_call))
        .route("/mcp/resources/list", get(handle_resources_list))
        .route("/mcp/resources/read", post(handle_resource_read))
        .route("/mcp/sse", get(handle_sse))
        .route("/mcp/ws", any(handle_websocket))
        .route("/health", get(health_check))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .with_state(state)
}

/// Handle generic MCP JSON-RPC request
async fn handle_mcp_request(
    State(state): State<HttpMcpState>,
    headers: HeaderMap,
    Json(request): Json<Value>,
) -> impl IntoResponse {
    let session_id = extract_session_id(&headers);
    let mut server = state.server.lock().await;

    // Process the request through the existing server logic
    let reply = server.handle(request);

    // Update session activity
    if let Some(sid) = session_id {
        update_session_activity(&state, &sid).await;
    }

    match reply {
        Some(response) => (StatusCode::OK, Json(response)).into_response(),
        None => (StatusCode::NO_CONTENT, Json(json!({}))).into_response(),
    }
}

/// Handle initialize request
async fn handle_initialize(
    State(state): State<HttpMcpState>,
    headers: HeaderMap,
    Json(request): Json<InitializeRequest>,
) -> impl IntoResponse {
    let session_id = extract_session_id(&headers).unwrap_or_else(|| Uuid::new_v4().to_string());
    let mut server = state.server.lock().await;

    let protocol_version = request
        .protocol_version
        .as_deref()
        .and_then(|v| SUPPORTED_VERSIONS.iter().find(|&&sv| sv == v).copied())
        .unwrap_or(PROTOCOL_VERSION);

    let response = InitializeResponse {
        protocol_version: protocol_version.to_string(),
        capabilities: json!({
            "tools": {"listChanged": false},
            "resources": {"listChanged": false}
        }),
        server_info: ServerInfo {
            name: "solidcraft".to_string(),
            title: "SolidCraft".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
        instructions: format!(
            "SolidCraft is an independent parametric 3D CAD app. Backend: {}.",
            server.backend.describe()
        ),
    };

    // Create/update session
    let mut sessions = state.sessions.write().await;
    sessions.insert(
        session_id.clone(),
        SessionState {
            id: session_id.clone(),
            initialized: true,
            scope: None,
            last_activity: std::time::Instant::now(),
        },
    );

    let mut response_builder = Json(response).into_response();
    response_builder.headers_mut().insert(
        "Mcp-Session-Id",
        HeaderValue::from_str(&session_id).unwrap(),
    );

    response_builder
}

/// Handle tools/list request
async fn handle_tools_list(
    State(state): State<HttpMcpState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let session_id = extract_session_id(&headers);
    let server = state.server.lock().await;

    if let Some(sid) = session_id {
        validate_session(&state, &sid).await;
    }

    let options = server.options();
    let tools = crate::tools::tool_definitions_with(options);

    (StatusCode::OK, Json(ToolsListResponse { tools })).into_response()
}

/// Handle tools/call request
async fn handle_tool_call(
    State(state): State<HttpMcpState>,
    headers: HeaderMap,
    Json(request): Json<ToolCallRequest>,
) -> impl IntoResponse {
    let session_id = extract_session_id(&headers);
    let mut server = state.server.lock().await;

    if let Some(sid) = session_id {
        validate_session(&state, &sid).await;
    }

    let options = server.options();
    let args = request.arguments.unwrap_or(json!({}));

    // Validate arguments
    if let Err(e) = crate::tools::check_arguments_with(&request.name, &args, options) {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": e}))).into_response();
    }

    // Execute tool
    let open = server.backend.design_path();
    server.scope = server.scope.take().map(|s| s.with_design(open.as_deref()));
    let scope = server.scope.clone();
    let backend = server.backend.as_mut();

    let mut result = crate::server::vfs::scoped(scope, || {
        crate::tools::call_tool_with(backend, &request.name, &args, options)
    });

    // Update scope with design files
    if result["isError"] != true {
        for file in crate::server::design_files(&request.name, &args)
            .iter()
            .filter(|f| std::path::Path::new(f).is_absolute())
        {
            server.scope = server.scope.take().map(|s| s.with_design_file(file));
        }
    }

    if let Some(note) = backend.notice() {
        if let Some(content) = result.get_mut("content").and_then(Value::as_array_mut) {
            content.push(json!({"type": "text", "text": note}));
        }
    }

    let response = ToolCallResponse {
        content: result["content"].as_array().cloned().unwrap_or_default(),
        is_error: result["isError"].as_bool(),
    };

    (StatusCode::OK, Json(response)).into_response()
}

/// Handle resources/list request
async fn handle_resources_list(
    State(state): State<HttpMcpState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let session_id = extract_session_id(&headers);
    if let Some(sid) = session_id {
        validate_session(&state, &sid).await;
    }

    let resources = vec![
        json!({
            "uri": "solidcraft://design",
            "name": "Active design",
            "description": "document.inspect with measurements",
            "mimeType": "application/json"
        }),
        json!({
            "uri": "solidcraft://commands",
            "name": "Command catalog",
            "description": "every command with its parameter docs",
            "mimeType": "application/json"
        }),
    ];

    (StatusCode::OK, Json(ResourcesListResponse { resources })).into_response()
}

/// Handle resources/read request
async fn handle_resource_read(
    State(state): State<HttpMcpState>,
    headers: HeaderMap,
    Json(request): Json<Value>,
) -> impl IntoResponse {
    let session_id = extract_session_id(&headers);
    let server = state.server.lock().await;

    if let Some(sid) = session_id {
        validate_session(&state, &sid).await;
    }

    let uri = request.get("uri").and_then(Value::as_str).unwrap_or("");
    let result = match uri {
        "solidcraft://design" | "solvecraft://design" => server.backend.call("document.inspect", json!({"measure": true})),
        "solidcraft://commands" | "solvecraft://commands" => server.backend.call("engine.commands", json!({})),
        _ => return (StatusCode::NOT_FOUND, Json(json!({"error": format!("unknown resource `{}`", uri)}))).into_response(),
    };

    match result {
        Ok(v) => {
            let response = ResourceReadResponse {
                contents: vec![ResourceContent {
                    uri: uri.to_string(),
                    mime_type: "application/json".to_string(),
                    text: v.to_string(),
                }],
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

/// Handle SSE connection
async fn handle_sse(
    State(state): State<HttpMcpState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let session_id = extract_session_id(&headers).unwrap_or_else(|| Uuid::new_v4().to_string());

    // Create session if needed
    {
        let mut sessions = state.sessions.write().await;
        sessions.insert(
            session_id.clone(),
            SessionState {
                id: session_id.clone(),
                initialized: false,
                scope: None,
                last_activity: std::time::Instant::now(),
            },
        );
    }

    let rx = state.event_tx.subscribe();
    let stream = BroadcastStream::new(rx)
        .filter_map(|msg| async move {
            msg.ok().map(|v| {
                Event::default()
                    .event("message")
                    .data(v.to_string())
            })
        });

    Sse::new(stream)
        .keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(30)))
        .into_response()
}

/// Handle WebSocket connection
async fn handle_websocket(
    State(state): State<HttpMcpState>,
    ws: WebSocketUpgrade,
    headers: HeaderMap,
) -> impl IntoResponse {
    let session_id = extract_session_id(&headers).unwrap_or_else(|| Uuid::new_v4().to_string());

    ws.on_upgrade(move |socket| handle_ws_connection(socket, state, session_id))
}

async fn handle_ws_connection(
    mut socket: axum::extract::ws::WebSocket,
    state: HttpMcpState,
    session_id: String,
) {
    // Create session
    {
        let mut sessions = state.sessions.write().await;
        sessions.insert(
            session_id.clone(),
            SessionState {
                id: session_id.clone(),
                initialized: false,
                scope: None,
                last_activity: std::time::Instant::now(),
            },
        );
    }

    let mut rx = state.event_tx.subscribe();

    loop {
        tokio::select! {
            // Handle incoming WebSocket messages
            msg = socket.recv() => {
                match msg {
                    Some(Ok(axum::extract::ws::Message::Text(text))) => {
                        let server = state.server.lock().await;
                        let reply = server.handle_line(&text);
                        if let Some(reply) = reply {
                            let _ = socket.send(axum::extract::ws::Message::Text(reply)).await;
                        }
                    }
                    Some(Ok(axum::extract::ws::Message::Binary(_))) => {
                        // Ignore binary messages
                    }
                    Some(Ok(axum::extract::ws::Message::Ping(data))) => {
                        let _ = socket.send(axum::extract::ws::Message::Pong(data)).await;
                    }
                    Some(Ok(axum::extract::ws::Message::Close(_))) => {
                        break;
                    }
                    Some(Err(_)) => {
                        break;
                    }
                    None => break,
                }
            }
            // Broadcast events to WebSocket
            event = rx.recv() => {
                match event {
                    Ok(v) => {
                        let _ = socket.send(axum::extract::ws::Message::Text(v.to_string())).await;
                    }
                    Err(_) => break,
                }
            }
        }
    }

    // Clean up session
    state.sessions.write().await.remove(&session_id);
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "healthy", "service": "solidcraft-mcp"})))
}

/// Extract session ID from headers
fn extract_session_id(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Mcp-Session-Id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

/// Validate and update session activity
async fn validate_session(state: &HttpMcpState, session_id: &str) -> bool {
    let mut sessions = state.sessions.write().await;
    if let Some(session) = sessions.get_mut(session_id) {
        session.last_activity = std::time::Instant::now();
        true
    } else {
        false
    }
}

/// Update session activity
async fn update_session_activity(state: &HttpMcpState, session_id: &str) {
    let mut sessions = state.sessions.write().await;
    if let Some(session) = sessions.get_mut(session_id) {
        session.last_activity = std::time::Instant::now();
    }
}

/// Start the HTTP MCP server
pub async fn serve_http(
    backend: Box<dyn Backend>,
    addr: std::net::SocketAddr,
    options: Options,
    path_scope: Option<crate::server::vfs::PathScope>,
) -> std::io::Result<()> {
    let server = Server::new(backend)
        .with_options(options.clone())
        .with_path_scope(path_scope);

    let (event_tx, _) = broadcast::channel(1000);

    let state = HttpMcpState {
        server: Arc::new(Mutex::new(server)),
        sessions: Arc::new(RwLock::new(HashMap::new())),
        event_tx,
        options,
    };

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("SolidCraft MCP HTTP server listening on {}", addr);

    axum::serve(listener, app).await
}

/// Start the HTTP MCP server with stdio fallback
pub async fn serve_all(
    backend: Box<dyn Backend>,
    http_addr: Option<std::net::SocketAddr>,
    options: Options,
    path_scope: Option<crate::server::vfs::PathScope>,
) -> std::io::Result<()> {
    let server = Server::new(backend)
        .with_options(options.clone())
        .with_path_scope(path_scope);

    let (event_tx, _) = broadcast::channel(1000);

    let state = HttpMcpState {
        server: Arc::new(Mutex::new(server)),
        sessions: Arc::new(RwLock::new(HashMap::new())),
        event_tx: event_tx.clone(),
        options: options.clone(),
    };

    // Spawn HTTP server if address provided
    if let Some(addr) = http_addr {
        let http_state = state.clone();
        let http_app = create_router(http_state);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!("SolidCraft MCP HTTP server listening on {}", addr);
        tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, http_app).await {
                tracing::error!("HTTP server error: {}", e);
            }
        });
    }

    // Serve stdio on main thread (for backward compatibility)
    let mut server = state.server.lock().await;
    server.serve(&mut std::io::stdin().lock(), &mut std::io::stdout().lock())?;

    Ok(())
}