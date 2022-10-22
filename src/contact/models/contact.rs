use crate::schema::contacts;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Queryable, PartialEq)]
pub struct Contact {
    id: i32,
    user_id: i32,
    lastname: String,
    firstname: String,
    email: String,
    phone: String,
}

#[derive(Deserialize, Validate, Insertable, AsChangeset)]
#[diesel(table_name = contacts)]
pub struct NewUpdateContact {
    #[validate(length(min = 4))]
    lastname: Option<String>,
    #[validate(length(min = 4))]
    firstname: Option<String>,
    #[validate(email)]
    email: Option<String>,
    phone: Option<String>,
}

impl Contact {
    /// Find all contacts for a user
    pub fn all(
        connection: &mut SqliteConnection,
        user_id_param: i32,
    ) -> Result<Vec<Contact>, Error> {
        use crate::schema::contacts::dsl::*;

        let results = contacts
            .filter(user_id.eq(user_id_param))
            .load::<Contact>(connection)?;

        Ok(results)
    }

    /// Find a contact by id
    pub fn find(connection: &mut SqliteConnection, id_param: i32) -> Result<Self, Error> {
        use crate::schema::contacts::dsl::*;

        let contact = contacts.find(id_param).first::<Contact>(connection)?;

        Ok(contact)
    }

    /// Create a new contact
    pub fn create(
        connection: &mut SqliteConnection,
        new_contact: NewUpdateContact,
    ) -> Result<Self, Error> {
        use crate::schema::contacts::dsl::*;

        diesel::insert_into(contacts)
            .values(&new_contact)
            .execute(connection)?;

        let contact = contacts.order(id.desc()).first::<Contact>(connection)?;

        Ok(contact)
    }

    /// Update a contact
    pub fn update(
        connection: &mut SqliteConnection,
        id_param: i32,
        new_contact: NewUpdateContact,
    ) -> Result<Self, Error> {
        use crate::schema::contacts::dsl::*;

        diesel::update(contacts.find(id_param))
            .set::<NewUpdateContact>(new_contact)
            .execute(connection)?;

        let contact = contacts.find(id_param).first::<Contact>(connection)?;

        Ok(contact)
    }

    /// Delete a contact
    pub fn delete(connection: &mut SqliteConnection, id_param: i32) -> Result<usize, Error> {
        use crate::schema::contacts::dsl::*;

        let result = diesel::delete(contacts.find(id_param)).execute(connection)?;

        Ok(result)
    }
}
