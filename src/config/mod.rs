use std::env;

#[derive(Debug, Clone)]
pub struct AppEnv {
    pub port: String,
    pub database: String,
    pub secret: String,
}

pub fn app_configuration() -> AppEnv {
    AppEnv {
        port: env::var("PORT").expect("Expected port to be set"),
        database: env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set"),
        secret: env::var("JWT_SECRET").expect("Expected JWT_SECRET to be set"),
    }
}