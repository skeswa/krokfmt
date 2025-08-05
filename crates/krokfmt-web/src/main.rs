use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, services::ServeDir, trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    // Future state like database connections, configs, etc.
}

#[derive(Debug, Deserialize)]
struct FormatRequest {
    code: String,
    #[serde(default)]
    #[allow(dead_code)]
    stdin: bool,
}

#[derive(Debug, Serialize)]
struct FormatResponse {
    formatted: String,
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "krokfmt_web=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState {};

    let app = Router::new()
        .route("/", get(index))
        .route("/docs", get(docs))
        .route("/playground", get(playground))
        .route("/api/format", post(format_code))
        .route("/health", get(health))
        .nest_service("/static", ServeDir::new("crates/krokfmt-web/static"))
        .nest_service("/wasm", ServeDir::new("crates/krokfmt-playground/pkg"))
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("krokfmt-web listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn index() -> Html<String> {
    let html = std::fs::read_to_string("crates/krokfmt-web/templates/index.html")
        .unwrap_or_else(|_| "Error loading page".to_string())
        .replace("{% extends \"base.html\" %}", "")
        .replace("{% block title %}", "")
        .replace("{% endblock %}", "")
        .replace("{% block content %}", "")
        .replace("{% if page == \"home\" %}active{% endif %}", "active");

    Html(wrap_in_base(html, "home"))
}

async fn docs() -> Html<String> {
    let html = std::fs::read_to_string("crates/krokfmt-web/templates/docs.html")
        .unwrap_or_else(|_| "Error loading page".to_string())
        .replace("{% extends \"base.html\" %}", "")
        .replace("{% block title %}", "")
        .replace("{% endblock %}", "")
        .replace("{% block content %}", "");

    Html(wrap_in_base(html, "docs"))
}

async fn playground() -> Html<String> {
    let html = std::fs::read_to_string("crates/krokfmt-web/templates/playground.html")
        .unwrap_or_else(|_| "Error loading page".to_string())
        .replace("{% extends \"base.html\" %}", "")
        .replace("{% block title %}", "")
        .replace("{% endblock %}", "")
        .replace("{% block content %}", "")
        .replace("{% block scripts %}", "")
        .replace("<script src=\"/static/js/playground.js\"></script>", "");

    let mut full_html = wrap_in_base(html, "playground");
    // Add the script tag before closing body
    full_html = full_html.replace(
        "</body>",
        "<script src=\"/static/js/playground.js\"></script>\n</body>",
    );

    Html(full_html)
}

fn wrap_in_base(content: String, page: &str) -> String {
    let base = std::fs::read_to_string("crates/krokfmt-web/templates/base.html")
        .unwrap_or_else(|_| "Error loading base template".to_string());

    base.replace(
        "{% block title %}krokfmt - Opinionated TypeScript Formatter{% endblock %}",
        "krokfmt",
    )
    .replace("{% block head %}{% endblock %}", "")
    .replace("{% block content %}{% endblock %}", &content)
    .replace("{% block scripts %}{% endblock %}", "")
    .replace(
        "{% if page == \"home\" %}active{% endif %}",
        if page == "home" { "active" } else { "" },
    )
    .replace(
        "{% if page == \"docs\" %}active{% endif %}",
        if page == "docs" { "active" } else { "" },
    )
    .replace(
        "{% if page == \"playground\" %}active{% endif %}",
        if page == "playground" { "active" } else { "" },
    )
}

async fn format_code(State(_state): State<AppState>, Json(req): Json<FormatRequest>) -> Response {
    // Use krokfmt to format the TypeScript code
    match krokfmt::format_typescript(&req.code, "input.ts") {
        Ok(formatted) => {
            let response = FormatResponse {
                formatted,
                success: true,
                error: None,
            };
            Json(response).into_response()
        }
        Err(err) => {
            let response = FormatResponse {
                formatted: req.code.clone(), // Return original on error
                success: false,
                error: Some(format!("Formatting error: {err}")),
            };
            Json(response).into_response()
        }
    }
}

async fn health() -> StatusCode {
    StatusCode::OK
}
