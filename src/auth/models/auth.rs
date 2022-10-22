use crate::schema::auths;
use argon2::{hash_encoded, verify_encoded, Config};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::env::var;

#[derive(Debug, Queryable)]
pub struct Auth {
    pub user_id: i32,
    pub hash: String,
    pub error: i32,
}

#[derive(Insertable)]
#[diesel(table_name = auths)]
pub struct NewAuth {
    pub user_id: i32,
    pub hash: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

impl Auth {
    /// Check if the authentification is possible
    pub fn is_blocked(&self) -> bool {
        self.error > 5
    }

    /// Check if the password is valid
    pub fn is_valid(&self, password: String) -> bool {
        verify_encoded(&self.hash, password.as_bytes()).unwrap()
    }

    /// Find an authentification by user id
    pub fn find_by_user_id(
        connection: &mut SqliteConnection,
        user_id_param: i32,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::auths::dsl::*;

        auths
            .filter(user_id.eq(user_id_param))
            .first::<Auth>(connection)
    }
}

impl NewAuth {
    /// Create a new authentification
    pub fn new(user_id: i32, password: String) -> Self {
        let config = Config::default();
        let salt = var("SALT").expect("SALT not found");
        let hash = hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
        Self { user_id, hash }
    }
}
