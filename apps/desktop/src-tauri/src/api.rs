use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use crate::content_images;
use crate::db::{fields_body, get_content_item_by_id, list_media_for_content, list_publish_tasks, update_publish_task_status, DbState};
use crate::rich_text;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tauri::{AppHandle, Manager};
use tower_http::cors::{AllowOrigin, CorsLayer};
use uuid::Uuid;

const API_PORT_START: u16 = 17345;
const API_PORT_END: u16 = 17365;

#[derive(Clone)]
pub struct ApiContext {
    app: AppHandle,
    token: Arc<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RootResponse {
    ok: bool,
    service: &'static str,
    version: &'static str,
    hint: &'static str,
    endpoints: &'static [&'static str],
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    ok: bool,
    version: &'static str,
    ui_locale: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ApiTaskSummary {
    id: String,
    status: String,
    channel_name: String,
    channel_color: String,
    content_title: String,
    content_language: String,
    publish_url: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ApiMediaRef {
    id: String,
    path: String,
    file_name: String,
    kind: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ApiTaskDetail {
    id: String,
    status: String,
    publish_url: String,
    channel: ApiChannelRef,
    content: ApiContentRef,
    media: Vec<ApiMediaRef>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ApiChannelRef {
    id: String,
    name: String,
    color: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ApiContentRef {
    id: String,
    title: String,
    language: String,
    body: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PrepareResponse {
    task_id: String,
    body: String,
    body_html: String,
    body_plain: String,
    media_paths: Vec<String>,
    image_count: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CopyImageResponse {
    ok: bool,
    file_name: String,
    image_count: usize,
    mode: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StageImagesResponse {
    ok: bool,
    folder_path: String,
    copied_count: usize,
    mode: &'static str,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PublishBody {
    url: Option<String>,
    published_at: Option<String>,
}

#[derive(Serialize)]
struct ApiErrorBody {
    error: ApiErrorDetail,
}

#[derive(Serialize)]
struct ApiErrorDetail {
    code: &'static str,
    message: String,
}

struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ApiErrorBody {
                error: ApiErrorDetail {
                    code: self.code,
                    message: self.message,
                },
            }),
        )
            .into_response()
    }
}

fn auth(headers: &HeaderMap, token: &str) -> Result<(), ApiError> {
    let header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    let expected = format!("Bearer {token}");
    if header != expected {
        return Err(ApiError {
            status: StatusCode::UNAUTHORIZED,
            code: "unauthorized",
            message: "Invalid or missing Authorization header".into(),
        });
    }
    Ok(())
}

fn map_task_summary(row: &(
    String, String, String, String, String, String, String, String, String, String, String, String,
    String, String,
)) -> ApiTaskSummary {
    ApiTaskSummary {
        id: row.0.clone(),
        status: row.1.clone(),
        channel_name: row.8.clone(),
        channel_color: row.9.clone(),
        content_title: row.11.clone(),
        content_language: row.12.clone(),
        publish_url: row.2.clone(),
    }
}

fn load_task_detail(state: &DbState, task_id: &str) -> Result<ApiTaskDetail, ApiError> {
    crate::db::with_conn(state, |conn| {
        let rows = list_publish_tasks(conn, None)?;
        let row = rows
            .into_iter()
            .find(|row| row.0 == task_id)
            .ok_or_else(|| "Task not found".to_string())?;

        let media = list_media_for_content(conn, &row.10)?
            .into_iter()
            .map(|(id, path, file_name, kind, _)| ApiMediaRef {
                id,
                path,
                file_name,
                kind,
            })
            .collect();

        Ok(ApiTaskDetail {
            id: row.0.clone(),
            status: row.1.clone(),
            publish_url: row.2.clone(),
            channel: ApiChannelRef {
                id: row.7.clone(),
                name: row.8.clone(),
                color: row.9.clone(),
            },
            content: ApiContentRef {
                id: row.10.clone(),
                title: row.11.clone(),
                language: row.12.clone(),
                body: fields_body(&row.13),
            },
            media,
        })
    })
    .map_err(|message| {
        if message == "Task not found" {
            ApiError {
                status: StatusCode::NOT_FOUND,
                code: "not_found",
                message,
            }
        } else {
            ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                code: "internal",
                message,
            }
        }
    })
}

async fn root() -> Json<RootResponse> {
    Json(RootResponse {
        ok: true,
        service: "PublishKit Local API",
        version: env!("CARGO_PKG_VERSION"),
        hint: "Use Authorization: Bearer <pairing_token> on /health and other endpoints.",
        endpoints: &[
            "/health",
            "/settings/ui-locale",
            "/tasks/today",
            "/tasks/:id",
            "/tasks/:id/prepare",
            "/tasks/:id/publish",
            "/tasks/:id/copy-image",
            "/tasks/:id/stage-images",
        ],
    })
}

async fn health(State(ctx): State<ApiContext>, headers: HeaderMap) -> Result<Json<HealthResponse>, ApiError> {
    auth(&headers, &ctx.token)?;
    let settings = crate::load_settings(&ctx.app).map_err(|message| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        code: "internal",
        message,
    })?;
    Ok(Json(HealthResponse {
        ok: true,
        version: env!("CARGO_PKG_VERSION"),
        ui_locale: settings.ui_locale,
    }))
}

async fn ui_locale(State(ctx): State<ApiContext>, headers: HeaderMap) -> Result<Json<serde_json::Value>, ApiError> {
    auth(&headers, &ctx.token)?;
    let settings = crate::load_settings(&ctx.app).map_err(|message| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        code: "internal",
        message,
    })?;
    Ok(Json(serde_json::json!({ "uiLocale": settings.ui_locale })))
}

async fn tasks_today(State(ctx): State<ApiContext>, headers: HeaderMap) -> Result<Json<Vec<ApiTaskSummary>>, ApiError> {
    auth(&headers, &ctx.token)?;
    let state = ctx.app.state::<DbState>();
    let tasks = crate::db::with_conn(&state, |conn| {
        Ok(list_publish_tasks(conn, Some("ready"))?
            .iter()
            .map(map_task_summary)
            .collect::<Vec<_>>())
    })
    .map_err(|message| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        code: "internal",
        message,
    })?;
    Ok(Json(tasks))
}

async fn task_detail(
    State(ctx): State<ApiContext>,
    headers: HeaderMap,
    Path(task_id): Path<String>,
) -> Result<Json<ApiTaskDetail>, ApiError> {
    auth(&headers, &ctx.token)?;
    let state = ctx.app.state::<DbState>();
    load_task_detail(&state, &task_id).map(Json)
}

async fn task_prepare(
    State(ctx): State<ApiContext>,
    headers: HeaderMap,
    Path(task_id): Path<String>,
) -> Result<Json<PrepareResponse>, ApiError> {
    auth(&headers, &ctx.token)?;
    let state = ctx.app.state::<DbState>();
    let detail = load_task_detail(&state, &task_id)?;
    let media_paths: Vec<String> = detail.media.iter().map(|item| item.path.clone()).collect();
    let image_count = detail
        .media
        .iter()
        .filter(|item| item.kind == "image")
        .count();
    let body = detail.content.body.clone();
    Ok(Json(PrepareResponse {
        task_id: detail.id,
        body_html: rich_text::markdown_to_html(&body),
        body_plain: rich_text::markdown_to_plain(&body),
        body,
        media_paths,
        image_count,
    }))
}

async fn task_copy_image(
    State(ctx): State<ApiContext>,
    headers: HeaderMap,
    Path(task_id): Path<String>,
) -> Result<Json<CopyImageResponse>, ApiError> {
    auth(&headers, &ctx.token)?;
    let state = ctx.app.state::<DbState>();
    let detail = load_task_detail(&state, &task_id)?;
    let images = crate::db::with_conn(&state, |conn| {
        content_images::list_content_image_paths(conn, &detail.content.id)
    })
    .map_err(|message| ApiError {
        status: StatusCode::BAD_REQUEST,
        code: "bad_request",
        message,
    })?;
    let (file_name, image_count) = content_images::copy_first_image(&images.paths).map_err(|message| ApiError {
        status: StatusCode::BAD_REQUEST,
        code: "bad_request",
        message,
    })?;
    Ok(Json(CopyImageResponse {
        ok: true,
        file_name,
        image_count,
        mode: "clipboard",
    }))
}

async fn task_stage_images(
    State(ctx): State<ApiContext>,
    headers: HeaderMap,
    Path(task_id): Path<String>,
) -> Result<Json<StageImagesResponse>, ApiError> {
    auth(&headers, &ctx.token)?;
    let state = ctx.app.state::<DbState>();
    let detail = load_task_detail(&state, &task_id)?;
    let (title, _, _) = crate::db::with_conn(&state, |conn| get_content_item_by_id(conn, &detail.content.id))
        .map_err(|message| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "internal",
            message,
        })?;
    let images = crate::db::with_conn(&state, |conn| {
        content_images::list_content_image_paths(conn, &detail.content.id)
    })
    .map_err(|message| ApiError {
        status: StatusCode::BAD_REQUEST,
        code: "bad_request",
        message,
    })?;
    let (dir, copied_count) = content_images::stage_images(&ctx.app, &title, &images.paths).map_err(|message| {
        ApiError {
            status: StatusCode::BAD_REQUEST,
            code: "bad_request",
            message,
        }
    })?;
    Ok(Json(StageImagesResponse {
        ok: true,
        folder_path: dir.to_string_lossy().into_owned(),
        copied_count,
        mode: "folder",
    }))
}

async fn task_publish(
    State(ctx): State<ApiContext>,
    headers: HeaderMap,
    Path(task_id): Path<String>,
    Json(body): Json<PublishBody>,
) -> Result<Json<ApiTaskDetail>, ApiError> {
    auth(&headers, &ctx.token)?;
    let state = ctx.app.state::<DbState>();
    crate::db::with_conn(&state, |conn| {
        update_publish_task_status(
            conn,
            &task_id,
            "published",
            body.url.as_deref(),
            None,
        )
    })
    .map_err(|message| ApiError {
        status: StatusCode::BAD_REQUEST,
        code: "bad_request",
        message,
    })?;
    load_task_detail(&state, &task_id).map(Json)
}

fn cors_layer() -> CorsLayer {
    CorsLayer::new().allow_origin(AllowOrigin::predicate(
        |origin: &axum::http::HeaderValue, _| {
            origin
                .to_str()
                .map(|value| {
                    value.starts_with("chrome-extension://")
                        || value.starts_with("http://127.0.0.1")
                        || value.starts_with("http://localhost")
                })
                .unwrap_or(false)
        },
    ))
    .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::OPTIONS])
    .allow_headers([axum::http::header::AUTHORIZATION, axum::http::header::CONTENT_TYPE])
}

pub fn pick_api_port() -> Result<u16, String> {
    for port in API_PORT_START..=API_PORT_END {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        if std::net::TcpListener::bind(addr).is_ok() {
            return Ok(port);
        }
    }
    Err("无法分配本地 API 端口".into())
}

pub fn generate_pairing_token() -> String {
    Uuid::new_v4().to_string().replace('-', "")
}

pub fn start_server(app: AppHandle, port: u16, token: String) {
    let ctx = ApiContext {
        app,
        token: Arc::new(token),
    };
    let router = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/settings/ui-locale", get(ui_locale))
        .route("/tasks/today", get(tasks_today))
        .route("/tasks/:id", get(task_detail))
        .route("/tasks/:id/prepare", post(task_prepare))
        .route("/tasks/:id/publish", post(task_publish))
        .route("/tasks/:id/copy-image", post(task_copy_image))
        .route("/tasks/:id/stage-images", post(task_stage_images))
        .layer(cors_layer())
        .with_state(ctx);

    tauri::async_runtime::spawn(async move {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                if axum::serve(listener, router).await.is_err() {
                    eprintln!("PublishKit API server stopped unexpectedly");
                }
            }
            Err(error) => eprintln!("PublishKit API bind failed: {error}"),
        }
    });
}
