use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EmailConfig {
    pub smtp_service: String,
    pub username: String,
    pub password: String,
}