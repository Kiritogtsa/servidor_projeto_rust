use super::types::handlefunc::myfunchandle;
pub enum Args {
    String(String),
    Json(serde_json::Value),
}
pub struct Route {
    path: String,
    handle: myfunchandle,
}
