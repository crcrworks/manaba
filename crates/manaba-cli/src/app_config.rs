use std::collections::HashMap;

use serde::{Deserialize, Serialize};

const DEFAULT_BASE_URL: &str = "https://ct.ritsumei.ac.jp/ct/";
const DEFAULT_COOKIE_DOMAIN: &str = "ct.ritsumei.ac.jp";

#[derive(Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub base_url: String,
    pub cookie_domain: String,
    pub timetable: HashMap<String, String>,
    pub color: HashMap<String, String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_owned(),
            cookie_domain: DEFAULT_COOKIE_DOMAIN.to_owned(),
            timetable: Default::default(),
            color: Default::default(),
        }
    }
}
