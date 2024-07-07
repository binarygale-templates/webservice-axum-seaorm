use anyhow::Context;
use clap::Parser;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tokio::net::TcpListener;
use tracing::info;

use webservice_axum_seaorm::{
    db_migrations::DbMigrator, routers::build_main_router, settings::LogFormat, AppState, Settings,
};

/// Sets up a relevant shutdown signals. This will exit on either SIGINT
/// (aka Ctrl+C) or SIGTERM.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to create Ctrl+C handler");
    };

    let sigterm = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to create SIGTERM handler")
            .recv()
            .await;
    };

    tokio::select! {
        () = ctrl_c => {},
        () = sigterm => {},
    }

    info!("shutdown signal received")
}

/// Creates a [DatabaseConnection] if possible. The pool has its max_connections
/// value set to the number of CPUs available.
pub async fn get_db_pool(
    connect_options: ConnectOptions,
) -> Result<DatabaseConnection, sea_orm::error::DbErr> {
    let mut options = connect_options.clone();
    options
        .max_connections(
            num_cpus::get()
                .try_into()
                .expect("number of CPU cores should fit into an u32"),
        )
        .sqlx_logging_level(log::LevelFilter::Debug);

    Database::connect(options).await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::parse();
    let settings_clone = settings.clone();

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(settings_clone.log_level.tracing_level())
        .with_target(false);
    match settings_clone.log_format {
        LogFormat::Text => subscriber.with_ansi(false).init(),
        LogFormat::TextColor => subscriber.with_ansi(true).init(),
        LogFormat::Json => subscriber.json().with_span_list(false).init(),
    }

    let database = get_db_pool(settings_clone.database_url).await?;
    DbMigrator::up(&database, None).await?;

    let router = build_main_router(AppState { database, settings });

    let listener = TcpListener::bind(settings_clone.listen)
        .await
        .context(format!("could not listen to `{}`", settings_clone.listen))?;

    info!("starting server on `{}`", settings_clone.listen);
    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("failed to start server")?;

    Ok(())
}
