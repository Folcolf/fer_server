use super::models::user::{Update, User};
use crate::{
    auth::models::claims::Claims,
    route,
    utils::{db::establish_connection, error::ApiError},
};
use axum::{extract::Path, Json, Router};
use serde_json::{json, Value};

/// Get all users
async fn get_all(claims: Claims) -> Result<Json<Value>, ApiError> {
    if !claims.is_admin() {
        return Err(ApiError::AdminRequired);
    }

    let connection = &mut establish_connection();

    let users = User::all(connection).map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(json!({ "users": users })))
}

/// Get a user by id
async fn get_one(claims: Claims, Path(id): Path<i32>) -> Result<Json<User>, ApiError> {
    if !claims.is_admin() && !claims.is_user(id) {
        return Err(ApiError::InvalidToken);
    }

    let connection = &mut establish_connection();

    let user = User::find(connection, id).map_err(|_| ApiError::NotFound)?;

    Ok(Json(user))
}

/// Update a user
async fn update_one(
    claims: Claims,
    Path(id): Path<i32>,
    Json(payload): Json<Update>,
) -> Result<Json<User>, ApiError> {
    if !claims.is_admin() && !claims.is_user(id) {
        return Err(ApiError::InvalidToken);
    }

    let connection = &mut establish_connection();

    let user = User::update(connection, id, payload).map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(user))
}

/// Delete a user
async fn delete_one(claims: Claims, Path(id): Path<i32>) -> Result<Json<Value>, ApiError> {
    if !claims.is_admin() && !claims.is_user(id) {
        return Err(ApiError::InvalidToken);
    }

    let connection = &mut establish_connection();

    User::delete(connection, id).map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(json!({ "message": "User deleted" })))
}

/// User routes
pub fn controller(router: &Router) -> Router {
    router
        .clone()
        .route(
            route("/user".to_string()).as_str(),
            axum::routing::get(get_all),
        )
        .route(
            route("/user/:id".to_string()).as_str(),
            axum::routing::get(get_one),
        )
        .route(
            route("/user/:id".to_string()).as_str(),
            axum::routing::put(update_one),
        )
        .route(
            route("/user/:id".to_string()).as_str(),
            axum::routing::delete(delete_one),
        )
}
