use crate::{
    auth::models::auth::NewAuth,
    schema::{auths, users},
};
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Queryable, PartialEq, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Insertable, Deserialize, Validate)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[validate(length(min = 4))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub role: String,
}

pub enum Role {
    Admin(String),
    User(String),
}

#[derive(Debug, Validate, Deserialize)]
pub struct Register {
    #[validate(length(min = 4))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, AsChangeset)]
#[diesel(table_name = users)]
pub struct Update {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

impl User {
    /// Get all users
    pub fn all(connection: &mut SqliteConnection) -> Result<Vec<User>, Error> {
        use crate::schema::users::dsl::*;

        let results = users.load::<User>(connection)?;

        Ok(results)
    }

    /// Find a user by id
    pub fn find(connection: &mut SqliteConnection, id_param: i32) -> Result<Self, Error> {
        use crate::schema::users::dsl::*;

        let user = users.find(id_param).first::<User>(connection)?;

        Ok(user)
    }

    /// Find a user by email
    pub fn find_by_email(
        connection: &mut SqliteConnection,
        email_param: String,
    ) -> Result<Self, Error> {
        use crate::schema::users::dsl::*;

        users
            .filter(email.eq(email_param))
            .first::<User>(connection)
    }

    /// Create a new user
    pub fn create(connection: &mut SqliteConnection, param: Register) -> Result<Self, Error> {
        let already_exists = User::find_by_email(connection, param.email.clone()).is_ok();
        if already_exists {
            return Err(Error::RollbackTransaction);
        }

        let new_user = NewUser {
            name: param.name,
            email: param.email,
            role: String::from("user"),
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(connection)?;

        let user = users::table
            .order(users::id.desc())
            .first::<User>(connection)?;

        let new_auth = NewAuth::new(user.id, param.password);

        let res = diesel::insert_into(auths::table)
            .values(&new_auth)
            .execute(connection)
            .map_err(|_| {
                diesel::delete(users::table.filter(users::id.eq(user.id))).execute(connection)
            });

        if res.is_err() {
            return Err(Error::RollbackTransaction);
        }

        Ok(user)
    }

    /// Update a user
    pub fn update(
        connection: &mut SqliteConnection,
        id_param: i32,
        param: Update,
    ) -> Result<Self, Error> {
        use crate::schema::users::dsl::*;

        diesel::update(users.filter(id.eq(id_param)))
            .set::<Update>(param)
            .execute(connection)?;

        users.find(id_param).first(connection)
    }

    /// Delete a user
    pub fn delete(connection: &mut SqliteConnection, id_param: i32) -> Result<usize, Error> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.find(id_param)).execute(connection)
    }
}
