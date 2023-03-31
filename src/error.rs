use axum::response::IntoResponse;
use axum::response::Response;
use garde::Errors;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WithValidationRejection<T> {
    ExtractionError(T),
    ValidationError(#[from] Errors),
}

impl<T: IntoResponse> IntoResponse for WithValidationRejection<T> {
    fn into_response(self) -> Response {
        match self {
            WithValidationRejection::ExtractionError(t) => t.into_response(),
            WithValidationRejection::ValidationError(e) => format!("{e}").into_response(),
        }
    }
}
