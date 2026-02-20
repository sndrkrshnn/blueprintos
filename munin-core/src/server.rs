use crate::agent::AgentRuntime;
use crate::protocol::{CoreEvent, SpeechTurn, ToolCall, ToolResult};
use crate::tools::ToolRouter;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::io::Read;
use std::sync::{Arc, Mutex};
use tiny_http::{Header, Method, Response, Server, StatusCode};

#[derive(Clone)]
pub struct ApiState {
    runtime: Arc<AgentRuntime>,
    pending: Arc<Mutex<HashMap<String, ToolCall>>>,
}

impl ApiState {
    pub fn new(runtime: AgentRuntime) -> Self {
        Self {
            runtime: Arc::new(runtime),
            pending: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[derive(Debug, Deserialize)]
struct TranscriptIn {
    transcript: String,
    session_id: Option<String>,
    locale: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ConfirmIn {
    id: String,
    approve: bool,
}

#[derive(Debug, Serialize)]
struct PendingItem {
    id: String,
    tool: String,
    args: serde_json::Value,
}

pub fn serve(addr: &str, state: ApiState) -> Result<()> {
    let server = Server::http(addr)?;
    tracing::info!("munin-core api listening on http://{}", addr);

    for mut req in server.incoming_requests() {
        let path = req.url().to_string();
        let method = req.method().clone();

        let resp = match (method, path.as_str()) {
            (Method::Post, "/v1/transcript") => {
                let mut body = String::new();
                let _ = req.as_reader().read_to_string(&mut body);
                handle_transcript(&state, &body)
            }
            (Method::Get, "/v1/pending") => handle_pending(&state),
            (Method::Post, "/v1/confirm") => {
                let mut body = String::new();
                let _ = req.as_reader().read_to_string(&mut body);
                handle_confirm(&state, &body)
            }
            (Method::Get, "/health") => ok(json!({"ok": true})),
            _ => json_response(StatusCode(404), json!({"error": "not_found"})),
        };

        let _ = req.respond(resp);
    }

    Ok(())
}

fn handle_transcript(state: &ApiState, body: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    let input: TranscriptIn = match serde_json::from_str(body) {
        Ok(v) => v,
        Err(e) => return json_response(StatusCode(400), json!({"error": e.to_string()})),
    };

    let events = match tokio::runtime::Handle::try_current() {
        Ok(h) => h.block_on(state.runtime.handle_text(&input.transcript, false)),
        Err(_) => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(state.runtime.handle_text(&input.transcript, false))
        }
    };

    match events {
        Ok(events) => {
            for ev in &events {
                if let CoreEvent::ToolCall(call) = ev {
                    if call.requires_confirmation {
                        state.pending.lock().unwrap().insert(call.id.clone(), call.clone());
                    }
                }
            }
            ok(json!({
                "session": input.session_id.unwrap_or_else(|| "default".into()),
                "events": events,
                "pending_count": state.pending.lock().unwrap().len()
            }))
        }
        Err(e) => json_response(StatusCode(500), json!({"error": e.to_string()})),
    }
}

fn handle_pending(state: &ApiState) -> Response<std::io::Cursor<Vec<u8>>> {
    let list: Vec<PendingItem> = state
        .pending
        .lock()
        .unwrap()
        .values()
        .map(|c| PendingItem {
            id: c.id.clone(),
            tool: c.tool.clone(),
            args: c.args.clone(),
        })
        .collect();
    ok(json!({"pending": list}))
}

fn handle_confirm(state: &ApiState, body: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    let input: ConfirmIn = match serde_json::from_str(body) {
        Ok(v) => v,
        Err(e) => return json_response(StatusCode(400), json!({"error": e.to_string()})),
    };

    let call = match state.pending.lock().unwrap().remove(&input.id) {
        Some(c) => c,
        None => return json_response(StatusCode(404), json!({"error": "pending_id_not_found"})),
    };

    if !input.approve {
        return ok(json!({"id": call.id, "ok": false, "message": "denied"}));
    }

    let result = match tokio::runtime::Handle::try_current() {
        Ok(h) => h.block_on(ToolRouter::execute(&call.tool, &call.args)),
        Err(_) => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(ToolRouter::execute(&call.tool, &call.args))
        }
    };

    match result {
        Ok(output) => ok(json!({"id": call.id, "ok": true, "result": ToolResult { id: call.id, ok: true, output }})),
        Err(e) => json_response(StatusCode(500), json!({"id": call.id, "ok": false, "error": e.to_string()})),
    }
}

fn ok(v: serde_json::Value) -> Response<std::io::Cursor<Vec<u8>>> {
    json_response(StatusCode(200), v)
}

fn json_response(code: StatusCode, v: serde_json::Value) -> Response<std::io::Cursor<Vec<u8>>> {
    let mut resp = Response::from_string(v.to_string()).with_status_code(code);
    if let Ok(h) = Header::from_bytes("Content-Type", "application/json") {
        resp = resp.with_header(h);
    }
    resp
}
