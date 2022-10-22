use super::models::contact::{Contact, NewUpdateContact};
use crate::{
    auth::models::claims::Claims,
    route,
    utils::{db::establish_connection, error::ApiError},
};
use axum::{extract::Path, Json, Router};
use serde_json::{json, Value};

/// Get all contacts
pub async fn get_all(claims: Claims, Path(id): Path<i32>) -> Result<Json<Value>, ApiError> {
    if !claims.is_admin() && !claims.is_user(id) {
        return Err(ApiError::InvalidToken);
    }

    let connection = &mut establish_connection();

    let contacts = Contact::all(connection, id).map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(json!({ "contacts": contacts })))
}

/// Get a contact by id
pub async fn find(claims: Claims, Path(id): Path<i32>) -> Result<Json<Contact>, ApiError> {
    if !claims.is_admin() && !claims.is_user(id) {
        return Err(ApiError::InvalidToken);
    }

    let connection = &mut establish_connection();

    let contact = Contact::find(connection, id).map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(contact))
}

/// Create a new contact
pub async fn create(
    claims: Claims,
    Path(id): Path<i32>,
    Json(new_contact): Json<NewUpdateContact>,
) -> Result<Json<Contact>, ApiError> {
    if !claims.is_admin() && !claims.is_user(id) {
        return Err(ApiError::InvalidToken);
    }

    let connection = &mut establish_connection();

    let contact =
        Contact::create(connection, new_contact).map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(contact))
}

/// Update a contact
pub async fn update(
    claims: Claims,
    Path(id): Path<i32>,
    Json(contact): Json<NewUpdateContact>,
) -> Result<Json<Contact>, ApiError> {
    if !claims.is_admin() && !claims.is_user(id) {
        return Err(ApiError::InvalidToken);
    }

    let connection = &mut establish_connection();

    let contact =
        Contact::update(connection, id, contact).map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(contact))
}

/// Delete a contact
pub async fn delete(claims: Claims, Path(id): Path<i32>) -> Result<Json<Value>, ApiError> {
    if !claims.is_admin() && !claims.is_user(id) {
        return Err(ApiError::InvalidToken);
    }

    let connection = &mut establish_connection();

    Contact::delete(connection, id).map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(json!({ "message": "Contact deleted" })))
}

/// Create a router for the contact routes
pub fn controller(router: &Router) -> Router {
    router
        .clone()
        .route(
            route("/contacts/:id/all".to_string()).as_str(),
            axum::routing::get(get_all),
        )
        .route(
            route("/contacts/:id".to_string()).as_str(),
            axum::routing::get(find),
        )
        .route(
            route("/contacts/:id".to_string()).as_str(),
            axum::routing::post(create),
        )
        .route(
            route("/contacts/:id".to_string()).as_str(),
            axum::routing::put(update),
        )
        .route(
            route("/contacts/:id".to_string()).as_str(),
            axum::routing::delete(delete),
        )
}
