use std::vec;

use axum::{
    Extension, Json,
    extract::{Multipart, Path, State},
    http::HeaderMap,
    response::IntoResponse,
};
use tokio::fs;
use tracing::{info, warn};

use crate::{AppError, AppState, ChatFile, User};
pub(crate) async fn send_message_handler() -> impl IntoResponse {
    // Handler logic for sending a message
}

pub(crate) async fn list_message_handler() -> impl IntoResponse {
    // Handler logic for listing messages
}

pub(crate) async fn file_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path((ws_id, path)): Path<(i64, String)>,
) -> Result<impl IntoResponse, AppError> {
    if user.ws_id != ws_id {
        return Err(AppError::NotFound(
            "File doesn't exist or you don't have permission to access it".to_string(),
        ));
    }
    let base_dir = &state.config.server.base_dir.join(ws_id.to_string());
    let path = base_dir.join(path);
    if !path.exists() {
        return Err(AppError::NotFound("File doesn't exist".to_string()));
    }

    let mime = mime_guess::from_path(&path).first_or_octet_stream();
    // TODO: streaming
    let body = fs::read(path).await?;
    // let file = File::open(&path).await?;
    // let stream = ReaderStream::new(file);
    // let body = StreamBody::new(stream);
    let mut headers = HeaderMap::new();
    headers.insert("content-type", mime.to_string().parse().unwrap());

    Ok((headers, body))
}

pub(crate) async fn upload_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let ws_id = user.ws_id as u64;
    let base_dir = &state.config.server.base_dir;
    let mut files = vec![];

    // 添加绝对路径日志
    info!("=== Upload Debug Info ===");
    info!("Config base_dir: {:?}", state.config.server.base_dir);
    info!(
        "Base dir absolute path: {:?}",
        base_dir.canonicalize().unwrap_or_else(|_| base_dir.clone())
    );
    info!("Current working directory: {:?}", std::env::current_dir());

    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().map(|name| name.to_string());
        let (Some(filename), Ok(data)) = (filename, field.bytes().await) else {
            warn!("failed to read multipart field");
            continue;
        };

        let file = ChatFile::new(ws_id, &filename, &data);
        let path = file.path(base_dir);

        // 写入前记录绝对路径
        let absolute_path = path
            .canonicalize()
            .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default().join(&path));
        info!("Absolute file path will be: {:?}", absolute_path);

        if path.exists() {
            info!("File already exists at: {:?}", absolute_path);
        } else {
            if let Some(parent) = path.parent() {
                if let Err(e) = fs::create_dir_all(parent).await {
                    warn!("Failed to create directory {:?}: {}", parent, e);
                    continue;
                }
            }

            if let Err(e) = fs::write(&path, &data).await {
                warn!("Failed to write file {:?}: {}", path, e);
                continue;
            }

            // 写入后再次确认文件位置
            let final_path = path.canonicalize().unwrap_or_else(|_| path.clone());
            info!("✅ File successfully written to: {:?}", final_path);
        }
        files.push(file.url());
    }

    Ok(Json(files))
}
