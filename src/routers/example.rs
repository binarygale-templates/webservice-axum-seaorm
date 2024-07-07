use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use sea_orm::prelude::*;

use crate::{entities::prelude::*, errors::ResponseError, AppState};

/// This is an example router to use the Database entity.
pub fn build() -> Router<AppState> {
    Router::new().route("/example", get(example_handler))
}

/// Returns the first Example entity.
#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn example_handler(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, ResponseError> {
    let res = Example::find().one(&app_state.database).await?;
    Ok(Json(res))
}
