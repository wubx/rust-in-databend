use serde::{Deserialize, Serialize};
use crate::app::models;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateShortLinkReq {
    pub url: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserResp {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteShortLinkReq {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteUserResp {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetShortLinkReq {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUsersResp {
    pub users: models::shortlink::ShortLink,
}
