use crate::AppState;
use axum::{
    extract::{FromRequestParts, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use tracing::warn;

pub async fn verify_token(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let (mut parts, body) = req.into_parts();
    let req =
        match TypedHeader::<Authorization<Bearer>>::from_request_parts(&mut parts, &state).await {
            Ok(TypedHeader(Authorization(bearer))) => {
                // Token is valid, continue with the request
                let token = bearer.token();

                match state.dk.verify(token) {
                    Ok(user) => {
                        let mut req = Request::from_parts(parts, body);
                        req.extensions_mut().insert(user);
                        req
                    }
                    Err(e) => {
                        let msg = format!("verify token failed: {}", e);
                        warn!(msg);
                        return (StatusCode::FORBIDDEN, msg).into_response();
                    }
                }
            }
            Err(e) => {
                // Token is invalid, return an error response
                let msg = format!("parse Authorization header failed: {}", e);
                warn!(msg);
                return (StatusCode::UNAUTHORIZED, msg).into_response();
            }
        };

    next.run(req).await
}
