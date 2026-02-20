use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use tiny_http::{Header, Response, Server, StatusCode};

#[derive(Parser, Debug)]
#[command(name = "munin-ui")]
struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    #[arg(long, default_value_t = 8080)]
    port: u16,
    #[arg(long, default_value = "/opt/muninos/ui")]
    ui_dir: String,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let addr = format!("{}:{}", args.host, args.port);
    let ui_root = PathBuf::from(args.ui_dir);

    let server = Server::http(&addr)?;
    tracing::info!("munin-ui serving {:?} at http://{}", ui_root, addr);

    for req in server.incoming_requests() {
        let url = req.url().trim_start_matches('/');
        let rel = if url.is_empty() { "index.html" } else { url };
        let target = safe_join(&ui_root, rel).unwrap_or_else(|| ui_root.join("index.html"));

        let (status, data, content_type) = if target.exists() && target.is_file() {
            let bytes = fs::read(&target).unwrap_or_default();
            (StatusCode(200), bytes, mime_for(&target))
        } else {
            let fallback = ui_root.join("index.html");
            if fallback.exists() {
                let bytes = fs::read(fallback).unwrap_or_default();
                (StatusCode(200), bytes, "text/html; charset=utf-8")
            } else {
                (
                    StatusCode(404),
                    b"Munin UI assets not found".to_vec(),
                    "text/plain; charset=utf-8",
                )
            }
        };

        let mut resp = Response::from_data(data).with_status_code(status);
        if let Ok(h) = Header::from_bytes("Content-Type", content_type) {
            resp = resp.with_header(h);
        }
        if let Err(e) = req.respond(resp) {
            tracing::warn!("response failed: {}", e);
        }
    }

    Ok(())
}

fn safe_join(base: &Path, rel: &str) -> Option<PathBuf> {
    let candidate = base.join(rel);
    let canon_base = base.canonicalize().ok()?;
    let canon_candidate = candidate.canonicalize().ok()?;
    if canon_candidate.starts_with(canon_base) {
        Some(canon_candidate)
    } else {
        None
    }
}

fn mime_for(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or_default() {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        _ => "application/octet-stream",
    }
}
