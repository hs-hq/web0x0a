use diesel::prelude::*;
use std::env;

#[derive(Clone)]
pub struct EnvConfig {
    pub database_url: String,
    pub port: u16,
    pub host: String,
}

impl EnvConfig {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let port = env::var("PORT").expect("PORT must be set");
        let host = env::var("HOST").expect("HOST must be set");

        Self {
            database_url,
            port: port.parse().unwrap(),
            host,
        }
    }
}

pub fn database_connection() -> PgConnection {
    let env_config = EnvConfig::new();
    let database_url = env_config.database_url.clone();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
