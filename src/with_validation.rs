use std::ops::Deref;

use super::WithValidationRejection;

use axum::async_trait;
use axum::extract::FromRef;
use axum::extract::FromRequest;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use garde::Validate;

#[derive(Debug)]
pub struct WithValidation<T>(pub T);

#[async_trait]
impl<S, T, E, CONTEXT> FromRequestParts<S> for WithValidation<E>
where
    S: Send + Sync,
    E: FromRequestParts<S> + Deref<Target = T>,
    T: Validate<Context = CONTEXT>,
    CONTEXT: FromRef<S>,
{
    type Rejection = WithValidationRejection<E::Rejection>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let extracted = E::from_request_parts(parts, state)
            .await
            .map_err(WithValidationRejection::ExtractionError)?;
        let state = FromRef::from_ref(state);
        extracted
            .validate(&state)
            .map_err(WithValidationRejection::ValidationError)?;
        Ok(WithValidation(extracted))
    }
}

#[async_trait]
impl<S, B, T, E, CONTEXT> FromRequest<S, B> for WithValidation<E>
where
    B: Send + 'static,
    S: Send + Sync,
    E: FromRequest<S, B> + Deref<Target = T>,
    T: Validate<Context = CONTEXT>,
    CONTEXT: FromRef<S>,
{
    type Rejection = WithValidationRejection<E::Rejection>;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let extracted = E::from_request(req, state)
            .await
            .map_err(WithValidationRejection::ExtractionError)?;

        let state = FromRef::from_ref(state);
        extracted
            .validate(&state)
            .map_err(WithValidationRejection::ValidationError)?;
        Ok(WithValidation(extracted))
    }
}
