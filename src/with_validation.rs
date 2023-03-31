use std::marker::PhantomData;

use super::Unwrapable;
use super::WithValidationRejection;

use axum::async_trait;
use axum::extract::FromRef;
use axum::extract::FromRequest;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use garde::Unvalidated;
use garde::Valid;
use garde::Validate;

#[derive(Debug)]
pub struct WithValidation<T, E>(pub Valid<T>, pub PhantomData<E>);

impl<T, E> From<Valid<T>> for WithValidation<T, E> {
    fn from(value: Valid<T>) -> Self {
        WithValidation(value, Default::default())
    }
}

#[async_trait]
impl<S, T, E, CONTEXT> FromRequestParts<S> for WithValidation<T, E>
where
    S: Send + Sync,
    E: FromRequestParts<S> + Unwrapable<T>,
    T: Validate<Context = CONTEXT>,
    CONTEXT: FromRef<S>,
{
    type Rejection = WithValidationRejection<E::Rejection>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let extracted = E::from_request_parts(parts, state)
            .await
            .map_err(WithValidationRejection::ExtractionError)?;
        let t: T = extracted.extract();
        let state = FromRef::from_ref(state);
        let valid = Unvalidated::from(t)
            .validate(&state)
            .map_err(WithValidationRejection::ValidationError)?;
        Ok(WithValidation(valid, Default::default()))
    }
}

#[async_trait]
impl<S, B, T, E, CONTEXT> FromRequest<S, B> for WithValidation<T, E>
where
    B: Send + 'static,
    S: Send + Sync,
    E: FromRequest<S, B> + Unwrapable<T>,
    T: Validate<Context = CONTEXT>,
    CONTEXT: FromRef<S>,
{
    type Rejection = WithValidationRejection<E::Rejection>;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let extracted = E::from_request(req, state)
            .await
            .map_err(WithValidationRejection::ExtractionError)?;

        let t: T = extracted.extract();
        let state = FromRef::from_ref(state);
        let valid = Unvalidated::from(t)
            .validate(&state)
            .map_err(WithValidationRejection::ValidationError)?;
        Ok(WithValidation(valid, Default::default()))
    }
}
