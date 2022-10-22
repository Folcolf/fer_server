use super::models::{
    auth::{Auth, AuthBody, AuthPayload},
    claims::Claims,
    keys::KEYS,
};
use crate::{
    route,
    user::models::user::{Register, User},
    utils::{db::establish_connection, error::ApiError},
};
use axum::{routing::post, Json, Router};
use jsonwebtoken::{encode, Header};

/// Log user with email and password and return a JWT token
async fn login(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, ApiError> {
    // Check if the user sent the credentials
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(ApiError::NotValid);
    }

    // Check if the user exists
    let connection = &mut establish_connection();

    let user = User::find_by_email(connection, payload.email).map_err(|_| ApiError::NotFound)?;
    let auth = Auth::find_by_user_id(connection, user.id).map_err(|_| ApiError::NotFound)?;

    if !auth.is_valid(payload.password) {
        return Err(ApiError::NotValid);
    }

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims::new(
        user.id.to_string(),
        user.role,
        "fer".to_string(),
        expiration,
    );

    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| ApiError::InternalServerError)?;

    // Send the authorized token
    Ok(Json(AuthBody {
        access_token: token,
    }))
}

/// Register a new user
async fn register(Json(payload): Json<Register>) -> Result<Json<String>, ApiError> {
    // Check if the user sent the credentials
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(ApiError::MissingCredentials);
    }

    let connection = &mut establish_connection();

    let user = User::create(connection, payload).map_err(|_| ApiError::NotValid)?;

    Ok(Json(user.id.to_string()))
}

/// Create the auth routes
pub fn controller(router: &Router) -> Router {
    router
        .clone()
        .route(route("/login".to_string()).as_str(), post(login))
        .route(route("/register".to_string()).as_str(), post(register))
}
