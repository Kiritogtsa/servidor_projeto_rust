use super::{body::Body, head::Head};
use serde::de::DeserializeOwned;
pub struct Request<T> {
    pub head: Head,
    pub body: T,
}

impl<T> Request<T>
where
    T: DeserializeOwned,
{
    // Cria um Request a partir de head + body raw (String)
    pub fn from_parts(head: Head, raw_body: &str) -> Result<Self, serde_json::Error> {
        // Aqui você poderia verificar Content-Type no head, etc
        // Para simplificar, vamos assumir que sempre é JSON:
        let body: T = serde_json::from_str(raw_body)?;
        Ok(Request { head, body })
    }
}
