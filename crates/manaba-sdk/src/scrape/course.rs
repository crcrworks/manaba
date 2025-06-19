use crate::Client;
use crate::error::Result;
use scraper::Selector;
use std::collections::HashMap;

pub struct Course {
    pub title: String,
    pub id: String,
}

impl Course {
    pub fn url(&self, base_url: &str) -> String {
        format!("{}/{}", base_url, self.id)
    }
}

impl Client {
    pub async fn get_courses(&self) -> Result<Vec<Course>> {
        let html = self.get_html(reqwest::Method::GET, "home_course").await?;

        let selector = Selector::parse("a[href^='course_']").unwrap();
        let course_elements = html.select(&selector);

        let mut all_courses = HashMap::new();
        for course_element in course_elements {
            if let Some(href) = course_element.attr("href") {
                all_courses
                    .entry(href.to_string())
                    .or_insert(course_element.inner_html());
            }
        }

        let courses = all_courses
            .into_iter()
            .filter(|(_, v)| v.chars().take(5).all(|c| c.is_numeric()))
            .map(|(id, title)| Course { title, id })
            .collect::<Vec<Course>>();

        Ok(courses)
    }
}
