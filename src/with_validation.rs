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

pub struct WithValidation<Extractor>(pub Valid<Extractor::Inner>)
where
    Extractor: IntoInner;

#[async_trait]
impl<State, Extractor, Context> FromRequestParts<State> for WithValidation<Extractor>
where
    State: Send + Sync,
    Extractor: FromRequestParts<State> + IntoInner,
    Extractor::Inner: Validate<Context = Context>,
    Context: FromRef<State>,
{
    type Rejection = WithValidationRejection<Extractor::Rejection>;

    async fn from_request_parts(parts: &mut Parts, state: &State) -> Result<Self, Self::Rejection> {
        let value = Extractor::from_request_parts(parts, state)
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
impl<State, Body, Extractor, Context> FromRequest<State, Body> for WithValidation<Extractor>
where
    Body: Send + 'static,
    State: Send + Sync,
    Extractor: FromRequest<State, Body> + IntoInner,
    Extractor::Inner: Validate<Context = Context>,
    Context: FromRef<State>,
{
    type Rejection = WithValidationRejection<Extractor::Rejection>;

    async fn from_request(req: Request<Body>, state: &State) -> Result<Self, Self::Rejection> {
        let value = Extractor::from_request(req, state)
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

impl<Extractor> Debug for WithValidation<Extractor>
where
    Extractor: IntoInner + Debug,
    Extractor::Inner: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WithValidation").field(&self.0).finish()
    }
}
