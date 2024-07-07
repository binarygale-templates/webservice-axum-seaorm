mod app_meta;
mod example;
mod fallback;

use axum::Router;

use crate::AppState;

/// Builds the main router.
/// This should collect all the routes from all over the app, and return a
/// full router for use in the app.
pub fn build_main_router(state: AppState) -> Router {
    Router::new()
        .merge(fallback::build())
        .merge(app_meta::build())
        .merge(example::build())
        .with_state(state)
}
