use axum::Router;
use diesel::migration::MigrationConnection;
use dotenvy::dotenv;
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

#[macro_use]
extern crate diesel;

use crate::{config::database_connection, handlers::create_router, repositories::seed};
mod config;
mod extractors;
mod handlers;
mod middlewares;
mod repositories;
mod schema;
mod utils;

// struct AppState {
//     pub env_config: config::EnvConfig,
// }
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn run_migrations() {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    let mut connection = database_connection();
    println!("Migrations run successfully");
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    run_migrations();
    seed();
    // build our application with a route

    let env_config = config::EnvConfig::new();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "example_tracing_aka_logging=debug,tower_http=debug",
        )
    }

    // let shared_state = Arc::new(AppState {
    //     env_config: env_config.clone(),
    // });

    tracing_subscriber::fmt::init();

    let router = create_router();
    let app = Router::new()
        .merge(router)
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], env_config.port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
