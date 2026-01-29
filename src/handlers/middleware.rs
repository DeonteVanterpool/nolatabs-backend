use crate::AppState;
use crate::logic::auth::login_user;
use axum::extract::Request;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Result};
use firebase_auth::FirebaseUser;
use tracing::Level;

pub async fn with_authenticated(
    State(state): State<AppState>,
    user: FirebaseUser,
    mut req: Request,
    next: Next,
    // Json(payload): Json<SignupPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    let (email, email_verified) = match (user.email, user.email_verified) {
        (Some(email), verified) => (email, verified.unwrap_or(false)),
        (None, _) => {
            tracing::warn!("No email found in FirebaseUser");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    if !crate::logic::user::verify_email(state.environment, &email, email_verified) {
        return Err(StatusCode::FORBIDDEN);
    }

    let uid = login_user(&state.user_repository, &email)
        .await
        .map_err(|e| {
            tracing::error!(email = %email, error = %e, "Database error finding user by email");
            StatusCode::from(e)
        })?
        .ok_or_else(|| {
            tracing::warn!(email = %email, "User not found in database");
            StatusCode::NOT_FOUND
        })?;

    req.extensions_mut().insert(uid);
    Ok(next.run(req).await)
}

pub async fn with_logging(req: Request, next: Next) -> Result<impl IntoResponse, StatusCode> {
    let log_id = uuid::Uuid::new_v4();

    // Extract useful headers
    let headers = req.headers();
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    // Create span
    let span = tracing::info_span!(
        "request",
        request_id = %log_id,
        method = %req.method(),
        uri = %req.uri(),
        user_agent = %user_agent,
    );

    let _enter = span.enter();

    tracing::info!(
        method = %req.method(),
        uri = %req.uri(),
        user_agent = %user_agent,
        "Incoming request"
    );

    let start = std::time::Instant::now();
    // Process request
    let response = next.run(req).await;
    let duration = start.elapsed();

    // Log completion
    let status = response.status();
    tracing::event!(
        Level::INFO,
        request_id = %log_id,
        status = status.as_u16(),
        latency_ms = duration.as_micros(),
        "Request completed"
    );

    // Add request_id to response headers (optional)
    let mut response = response;
    response
        .headers_mut()
        .insert("x-request-id", log_id.to_string().parse().unwrap());

    Ok(response)
}
