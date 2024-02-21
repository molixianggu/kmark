use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: u16,
    pub message: String,
    pub data: T,
}

impl<T> Response<T> {
    pub fn success(data: T) -> Json<Self> {
        Json(Self {
            code: 200,
            message: "success".to_string(),
            data,
        })
    }
}
