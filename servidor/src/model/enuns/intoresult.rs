use serde_json::Value;

use super::httpcodes::HTTPSTATUS;
pub enum Intoresult {
    String(String),
    Json(Value),
    Bool(bool),
    HTTPCode(HTTPSTATUS),
}
