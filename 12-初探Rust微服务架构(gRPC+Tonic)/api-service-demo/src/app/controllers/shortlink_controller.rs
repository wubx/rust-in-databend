use axum::{Json, extract};
use crate::app::models::dto;
use axum::extract::Extension;
use axum::response::IntoResponse;
use axum::http::{StatusCode, HeaderMap, Request};
use crate::app::models::shortlink;
use sqlx::{Pool, MySql};
use axum::http::header::LOCATION;
use axum::body::Body;
use redis::{Client, AsyncCommands, RedisResult};
use redis::aio::Connection;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use std::ops::Deref;

pub async fn create_shortlink(
    Json(req): Json<dto::CreateShortLinkReq>,
    Extension(pool): Extension<Pool<MySql>>
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
            (StatusCode::OK, Json(dto::DeleteShortLinkResp {
                ok: true
            }))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(dto::DeleteShortLinkResp {
                ok: false
            }))
        }
    }
}

pub async fn get_shortlink(
    extract::Path(id): extract::Path<i32>,
    req: Request<Body>
) -> impl IntoResponse {
    let mut url = String::from("/api/not_found");
    //let pool = req.extensions().get::<Pool<MySql>>().unwrap();
    let mut con = req.extensions().get::<Arc<Mutex<Connection>>>().unwrap().lock().await;
    let mut redis_key = String::from("url_");
    redis_key.push_str(&*id.to_string());
    let res: RedisResult<String> = con.get(redis_key).await;
    match res {
        Ok(v) => {
            url = v;
        }
        Err(err) => {
            println!("err = {:#?}", err);
        }
    }
    // match shortlink::get_shortlink(pool, id).await {
    //     Ok(record) => {
    //         url = Box::leak(record.url.into_boxed_str());
    //     }
    //     Err(err) => {
    //         println!("err = {:#?}", err);
    //     }
    // }
    let mut headers = HeaderMap::new();
    headers.insert(LOCATION, url.parse().unwrap());
    (StatusCode::FOUND, headers, ())
}

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::OK, "404 Not Found")
}