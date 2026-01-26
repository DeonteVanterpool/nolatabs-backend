use tracing::Level;
use crate::AppState;
use crate::repository::user::UserRepositoryTrait;
use axum::extract::Request;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Result};
use firebase_auth::FirebaseUser;

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
        },
    };
    if !verify_email(
        state.environment,
        &email,
        email_verified
    ) {
        return Err(StatusCode::FORBIDDEN);
    }

    let uid = state
        .user_repository
        .find_by_email(&email)
        .await
        .map_err(|e| {
            tracing::error!(email = %email, error = %e, "Database error finding user by email");
            StatusCode::INTERNAL_SERVER_ERROR
        })
        .map(|option| 
            {
                if option.is_none() {
                    tracing::warn!(email = %email, "User not found in database");
                }
                option.ok_or(StatusCode::NOT_FOUND)
            }
                ).map(|v| v);
    req.extensions_mut().insert(uid??);
    Ok(next.run(req).await)
}

fn verify_email(env: crate::state::Environment, email: &str, email_verified: bool) -> bool {
    return email_verified
        || (email.ends_with("@test.account") && env != crate::state::Environment::Production);
}

#[cfg(test)]
mod tests {
    use super::verify_email;
    #[test]
    fn verify_email_tests() {
        assert!(!verify_email(
            crate::state::Environment::Production,
            "fake@outlook.com",
            false
        ));
        assert!(!verify_email(
            crate::state::Environment::Production,
            "fake@test.account",
            false
        ));
        assert!(verify_email(
            crate::state::Environment::Production,
            "fake@outlook.com",
            true
        ));
        assert!(verify_email(
            crate::state::Environment::Production,
            "fake@test.account",
            true
        ));
        assert!(verify_email(
            crate::state::Environment::Testing,
            "fake@test.account",
            true
        ));
        assert!(!verify_email(
            crate::state::Environment::Testing,
            "fake@outlook.com",
            false
        ));
        assert!(verify_email(
            crate::state::Environment::Staging,
            "fake@test.account",
            true
        ));
    }
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
    response.headers_mut().insert(
        "x-request-id",
        log_id.to_string().parse().unwrap()
    );
    
    Ok(response)
}
