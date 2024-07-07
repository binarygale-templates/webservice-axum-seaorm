use sea_orm::DatabaseConnection;

use crate::settings::Settings;

#[derive(Clone, Debug)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub settings: Settings,
}
