use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum ResponseError {
    #[error("internal server error")]
    DatabaseError(#[from] sea_orm::DbErr),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        error!("response error: {}", self);

        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(self.to_string()))
            .expect("Response Builder with known setup should not fail")
            .into_response()
    }
}
