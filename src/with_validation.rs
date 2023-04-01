use std::fmt::Debug;

use super::IntoInner;
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

pub struct WithValidation<E: IntoInner>(pub Valid<<E as IntoInner>::Inner>);

#[async_trait]
impl<S, E, Ctx> FromRequestParts<S> for WithValidation<E>
where
    S: Send + Sync,
    E: FromRequestParts<S> + IntoInner,
    <E as IntoInner>::Inner: Validate<Context = Ctx>,
    Ctx: FromRef<S>,
{
    type Rejection = WithValidationRejection<E::Rejection>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let value = E::from_request_parts(parts, state)
            .await
            .map_err(WithValidationRejection::ExtractionError)?;
        let ctx = FromRef::from_ref(state);
        let value = value.into_inner();
        let value = Unvalidated::new(value)
            .validate(&ctx)
            .map_err(WithValidationRejection::ValidationError)?;
        Ok(WithValidation(value))
    }
}

#[async_trait]
impl<S, B, E, Ctx> FromRequest<S, B> for WithValidation<E>
where
    B: Send + 'static,
    S: Send + Sync,
    E: FromRequest<S, B> + IntoInner,
    <E as IntoInner>::Inner: Validate<Context = Ctx>,
    Ctx: FromRef<S>,
{
    type Rejection = WithValidationRejection<E::Rejection>;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let value = E::from_request(req, state)
            .await
            .map_err(WithValidationRejection::ExtractionError)?;
        let ctx = FromRef::from_ref(state);
        let value = value.into_inner();
        let value = Unvalidated::new(value)
            .validate(&ctx)
            .map_err(WithValidationRejection::ValidationError)?;
        Ok(WithValidation(value))
    }
}

impl<E: IntoInner> Debug for WithValidation<E>
where
    <E as IntoInner>::Inner: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::ops::Deref;
        Debug::fmt(&self.0.deref(), f)
    }
}
