use crate::{
    Cookie,
    error::{ManabaError, Result},
};
use reqwest::{IntoUrl, Method, RequestBuilder, header::HeaderMap};
use scraper::{Html, Selector};
use std::fmt::Display;

pub struct Client {
    pub base_url: String,
    pub(crate) reqwest_client: reqwest::Client,
    pub(crate) headers: HeaderMap,
}

impl Client {
    pub async fn new(base_url: &str, cookie: &Cookie) -> Result<Self> {
        let reqwest_client = reqwest::Client::builder().build().unwrap();

        let headers = {
            let mut headers = HeaderMap::new();
            headers.insert("Cookie", cookie.0.parse().unwrap());
            headers
        };

        let client = Client {
            base_url: base_url.to_owned(),
            reqwest_client,
            headers,
        };

        if client.is_valid_cookie().await? {
            Ok(client)
        } else {
            Err(ManabaError::InvalidCookie)
        }
    }

    pub(crate) fn request<U>(&self, method: Method, url: U) -> RequestBuilder
    where
        U: IntoUrl + Display,
    {
        self.reqwest_client
            .request(method, format!("{}/{}", self.base_url, url))
            .headers(self.headers.clone())
    }

    pub(crate) async fn get_html<U>(&self, method: reqwest::Method, url: U) -> Result<Html>
    where
        U: IntoUrl + Display,
    {
        let request = self.request(method, url);

        let response = request
            .send()
            .await
            .map_err(ManabaError::SendRequestError)?;

        let body = response
            .text()
            .await
            .map_err(ManabaError::HtmlBodyParseError)?;

        let document = Html::parse_document(&body);
        Ok(document)
    }

    pub async fn is_valid_cookie(&self) -> Result<bool> {
        let request = self.request(reqwest::Method::GET, "");

        let response = request
            .send()
            .await
            .map_err(ManabaError::SendRequestError)?;

        let body = response
            .text()
            .await
            .map_err(ManabaError::HtmlBodyParseError)?;

        let document = Html::parse_document(&body);

        let selector = Selector::parse("div#orgheader").unwrap();
        let elements = document.select(&selector);

        Ok(elements.count() != 0)
    }
}
