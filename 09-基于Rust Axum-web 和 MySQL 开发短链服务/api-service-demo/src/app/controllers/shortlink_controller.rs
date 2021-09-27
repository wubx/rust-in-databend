use axum::Json;
use crate::dto;
use axum::extract::Extension;
use sqlx::{Pool, MySql};
use axum::response::IntoResponse;
use axum::http::StatusCode;
use crate::app::models::shortlink;

pub async fn create_shortlink(
    Json(req): Json<dto::CreateShortLinkReq>,
    Extension(pool): Extension<Pool<MySql>>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    match shortlink::create_shortlink(&pool, &req.url).await {
        Ok(_) => {
            (StatusCode::OK, Json(dto::CreateUserResp {
                ok: true
            }))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(dto::CreateUserResp {
                ok: false
            }))
        }
    }
}

pub async fn delete_shortlink(
    Json(req): Json<dto::DeleteShortLinkReq>,
    Extension(pool): Extension<Pool<MySql>>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    match shortlink::delete_shortlink(&pool, req.id).await {
        Ok(_) => {
            (StatusCode::OK, Json(dto::DeleteUserResp {
                ok: true
            }))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(dto::DeleteUserResp {
                ok: false
            }))
        }
    }
}