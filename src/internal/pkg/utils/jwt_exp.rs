use std::time::Duration;

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