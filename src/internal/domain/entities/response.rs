use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(rename = "responseCode")]
    pub response_code: String,
    #[serde(rename = "responseDesc")]
    pub response_desc: String,
    #[serde(rename = "responseData", skip_serializing_if = "Option::is_none")]
    pub response_data: Option<T>,
}