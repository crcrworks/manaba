use crate::error::{ManabaError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Cookie(pub String);

#[allow(unused)]
impl Cookie {
    pub fn new(cookie: &str) -> Self {
        Cookie(cookie.to_owned())
    }

    pub fn load(domain: &str) -> Result<Self> {
        let domains = vec![domain.to_owned()];
        let domains = Some(domains);

        let mut browsers = [
            rookie::chrome,
            rookie::chromium,
            rookie::zen,
            rookie::brave,
            rookie::arc,
        ];

        for browser in browsers {
            let mut cookies = match browser(domains.clone()) {
                Ok(v) => v,
                Err(_) => continue,
            };

            cookies
                .iter_mut()
                .for_each(|s| s.name = s.name.trim().to_string());

            if cookies.is_empty() {
                continue;
            }

            if !cookies.iter().any(|v| v.name == "sessionid") {
                continue;
            }

            let cookie = cookies
                .iter()
                .map(|cookie| format!("{}={}", cookie.name, cookie.value))
                .collect::<Vec<_>>()
                .join(";");

            return Ok(Cookie(cookie));
        }

        Err(ManabaError::LoadCookie("Cookie not found".to_owned()))
    }
}
