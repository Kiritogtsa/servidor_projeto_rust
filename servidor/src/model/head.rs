use std::collections::HashMap;

use std::convert::TryFrom;

#[derive(Debug)]
pub struct Head {
    method: String,
    path: String,
    args: Option<String>,
    version: String,
    pub headers: HashMap<String, String>,
    cookies: Option<HashMap<String, String>>,
    host: Option<String>,
    user_agent: Option<String>,
}

pub(crate) trait NewTrait {
    fn content_type(&self) -> Option<&str>;
}

impl NewTrait for Head {
    fn content_type(&self) -> Option<&str> {
        self.headers.get("content-type").map(|s| s.as_str())
    }
}
impl TryFrom<&[String]> for Head {
    type Error = String;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        if lines.is_empty() {
            return Err("Empty request".into());
        }

        let mut parts = lines[0].split_whitespace();
        let method = parts.next().ok_or("Missing method")?.to_string();
        let url = parts.next().ok_or("Missing path")?.to_string();
        let version = parts.next().ok_or("Missing HTTP version")?.to_string();

        let (path, args) = if let Some(idx) = url.find('?') {
            (url[..idx].to_string(), Some(url[idx + 1..].to_string()))
        } else {
            (url, None)
        };

        let mut headers = HashMap::new();
        let mut cookies_map = HashMap::new();
        let mut host = None;
        let mut user_agent = None;

        for line in &lines[1..] {
            if let Some((key, value)) = line.split_once(": ") {
                let key_lower = key.to_ascii_lowercase();
                let value = value.trim().to_string();
                headers.insert(key_lower.clone(), value.clone());

                match key_lower.as_str() {
                    "host" => host = Some(value),
                    "user-agent" => user_agent = Some(value),
                    "cookie" => {
                        for pair in value.split(';') {
                            if let Some((k, v)) = pair.trim().split_once('=') {
                                cookies_map.insert(k.trim().to_string(), v.trim().to_string());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let cookies = if cookies_map.is_empty() {
            None
        } else {
            Some(cookies_map)
        };

        Ok(Head {
            method,
            path,
            args,
            version,
            headers,
            cookies,
            host,
            user_agent,
        })
    }
}
