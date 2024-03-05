use axum::{
    http,
    routing::get,
    middleware::{self, Next},
    extract::{Request, Extension},
    extract::rejection::JsonRejection,
    Json,
    response::{IntoResponse, Response},
    body::Body,
    http::StatusCode,
};

use axum_extra::extract::WithRejection;
use thiserror::Error;
use serde::{Deserialize, Serialize};

use crate::tools::jwt;

async fn auth_jwt(req: Request, next: Next) -> Response {
    match handle_auth_jwt(req, next).await {
        Ok(response) => response,
        Err(status_code) => {
            let body = format!("Error: {}", status_code);
            Response::builder()
                .status(status_code)
                .body(body.into())
                .unwrap()
        }
    }
}

async fn handle_auth_jwt(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    if let Some(uid) = jwt::dn_token(auth_header).await  {
        req.extensions_mut().insert(Some(CurrentUser{id:uid}));
        Ok(next.run(req).await)
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }
}
