use super::types::{handlefunc::myfunchandle, methods::Methods};
pub enum Args {
    String(String),
    Json(serde_json::Value),
}
#[derive(Clone)]
pub struct Route {
    pub path: String,
    handle: myfunchandle,
    pub method: Methods,
}
