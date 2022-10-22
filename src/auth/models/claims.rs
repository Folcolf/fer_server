use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    TypedHeader,
};
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::error::ApiError;

use super::keys::KEYS;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    role: String,
    company: String,
    exp: i64,
}

impl Claims {
    /// Create a new claims
    pub fn new(sub: String, role: String, company: String, exp: i64) -> Self {
        Self {
            sub,
            role,
            company,
            exp,
        }
    }

    /// Check if the user is an admin
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }

    /// Check if the id is the same as the user id
    pub fn is_user(&self, id: i32) -> bool {
        self.sub.parse::<i32>().unwrap() == id
    }
}

// verify token and extract data from it (a kind of middleware), whenever you try to extract claims in the handle it will first run this code
#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = ApiError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| ApiError::InvalidToken)?;

        let data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| ApiError::InvalidToken)?;

        Ok(data.claims)
    }
}
