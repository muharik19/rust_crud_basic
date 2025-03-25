use regex::Regex;

#[derive(Debug)]
pub struct PaginationRequest {
    pub limit: i64,
    pub page: i64,
    pub field: String,
    pub sort: String,
}

impl PaginationRequest {
    pub fn new(limit: i64, page: i64, field: &str, sort: &str) -> Self {
        let re = Regex::new(r"^[a-zA-Z]+$").unwrap();
        let mut field = field.to_string();
        
        if !re.is_match(&field) {
            field = "created_at".to_string();
        }
        
        let sort = match sort.to_lowercase().as_str() {
            "asc" => "ASC".to_string(),
            "desc" => "DESC".to_string(),
            _ => "ASC".to_string(),
        };
        
        let limit = if limit == 0 { 10 } else { limit };
        let page = if page == 0 { 1 } else { page };
        
        PaginationRequest { limit, page, field, sort }
    }
}
