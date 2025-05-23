use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub database_url: String,
    pub port: String,
    pub secret_key_jwt: String,
    pub jwt_exp: String,
}

// Using Lazy to initialize configuration once.
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let secret_key_jwt = env::var("SECRET_KEY_JWT").expect("SECRET_KEY_JWT must be set");
    let jwt_exp = env::var("JWT_EXP").expect("JWT_EXP must be set");
    // println!("DATABASE_URL: {}", database_url);
    // println!("DATABASE_URL: {}", std::env::var("DATABASE_URL").unwrap());
    Config { database_url, port, secret_key_jwt, jwt_exp }
});
