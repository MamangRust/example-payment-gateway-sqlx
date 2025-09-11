use axum::{
    Extension, Json,
    body::Body,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use shared::{abstract_trait::jwt::DynJwtService, errors::ErrorResponse};

pub async fn auth(
    cookie_jar: CookieJar,
    Extension(jwt): Extension<DynJwtService>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| auth_value.strip_prefix("Bearer ").map(str::to_owned))
        });

    let token = match token {
        Some(token) => token,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    status: "fail".to_string(),
                    message: "You are not logged in, please provide token".to_string(),
                }),
            ));
        }
    };

    let user_id = match jwt.verify_token(&token, "access") {
        Ok(id) => id as i32,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    status: "fail".to_string(),
                    message: "Invalid token".to_string(),
                }),
            ));
        }
    };

    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}
