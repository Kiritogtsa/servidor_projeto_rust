use crate::NewTrait;

use super::head::Head;
use serde::de::DeserializeOwned;
pub struct Request<T> {
    pub head: Head,
    pub body: T,
}
type Error = String;
impl<T> Request<T>
where
    T: DeserializeOwned,
{
    // Cria um Request a partir de head + body raw (String)
    pub fn from_parts(head: Head, raw_body: &str) -> Result<Self, Error> {
        // Aqui você poderia verificar Content-Type no head, etc
        // Para simplificar, vamos assumir que sempre é JSON:
        match head.content_type() {
            Some(ct) if ct.contains("application/json") => {
                let body = serde_json::from_str(raw_body)
                    .map_err(|e| format!("Erro ao desserializar JSON: {}", e))?;
                Ok(Request { head, body })
            }
            Some(ct) if ct.contains("text/plain") => {
                // se T = String, por exemplo
                // Ou implementar outra lógica
                Err("Deserialização para text/plain não implementada".into())
            }
            Some(ct) => Err(format!("Content-Type não suportado: {}", ct)),
            None => Err("Content-Type ausente".into()),
        }
    }
}
