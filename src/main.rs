use axum::routing::{get, post};
use axum::{Json, Router, http::StatusCode, response::IntoResponse};
use hyper::server::conn::http1;
use hyper_util::{rt::tokio::TokioIo, service::TowerToHyperService};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ping", get(ping))
        .route("/echo", post(echo))
        .fallback_service(ServeDir::new("public").append_index_html_on_directories(true));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("✅ Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let app = app.clone();
        let service = TowerToHyperService::new(app);

        tokio::spawn(async move {
            let io = TokioIo::new(stream);
            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                eprintln!("❌ Server error: {}", err);
            }
        });
    }
}

// ✅ /ping -> 200 OK + JSON
async fn ping() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "pong": true })))
}

#[derive(Deserialize)]
struct Input {
    message: String,
}

// ✅ /echo -> 200 OK + JSON (หรือ 400 Bad Request ถ้า message ว่าง)
async fn echo(Json(payload): Json<Input>) -> impl IntoResponse {
    if payload.message.trim().is_empty() {
        // Bad Request
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "message must not be empty" })),
        );
    }

    // OK
    (
        StatusCode::OK,
        Json(serde_json::json!({ "received": payload.message })),
    )
}
