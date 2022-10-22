use axum::{
    async_trait,
    extract::{Form, FromRequest, RequestParts},
    BoxError,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::ApiError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ApiError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req)
            .await
            .map_err(|_| ApiError::NotValid)?;
        value.validate().map_err(|_| ApiError::NotValid)?;
        Ok(ValidatedForm(value))
    }
}
