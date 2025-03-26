use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub database_url: String,
    pub port: String,
}

// Using Lazy to initialize configuration once.
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = env::var("PORT").expect("PORT must be set");
    // println!("DATABASE_URL: {}", database_url);
    // println!("DATABASE_URL: {}", std::env::var("DATABASE_URL").unwrap());
    Config { database_url, port }
});
