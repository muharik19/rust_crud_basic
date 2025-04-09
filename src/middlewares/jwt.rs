use crate::internal::domain::entities::auth::login::Claims;
use crate::config::settings::CONFIG;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, errors::ErrorKind};
use std::time::Duration;

pub async fn jwt_decode(token: &str) -> Result<Claims, String> {
    match decode::<Claims>(token, &DecodingKey::from_secret(CONFIG.secret_key_jwt.clone().as_ref()), &Validation::new(Algorithm::HS256)) {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => match *err.kind() {
            ErrorKind::ExpiredSignature => Err("Token has expired.".to_string()),
            ErrorKind::InvalidToken => Err("Token is invalid.".to_string()),
            _ => Err(format!("Token verification failed: {}", err)),
        },
    }
}

// pub async fn is_authorized(token: &str) -> Result<bool, String> {
//     let claims = jwt_decode(token, secret).await?;
//     // Optional: check if claims are acceptable
//     if claims.sub.is_empty() {
//         return Ok(false);
//     }
//     Ok(true)
// }

pub fn parse_jwt_exp(exp: &str) -> Option<Duration> {
    let (num_part, unit_part) = exp.trim().split_at(exp.len() - 1);
    let number: u64 = num_part.parse().ok()?;
    match unit_part {
        "s" => Some(Duration::from_secs(number)),          // seconds
        "m" => Some(Duration::from_secs(number * 60)),     // minutes
        "h" => Some(Duration::from_secs(number * 3600)),   // hours
        "d" => Some(Duration::from_secs(number * 86400)),  // days
        _ => None, // invalid format
    }
}
