use crate::Client;
use crate::assignment::{AssignmentDate, AssignmentState};
use crate::{Course, error::Result};
use reqwest::Method;
use scraper::Selector;

pub struct Exam {
    pub title: String,
    pub state: AssignmentState,
    pub start_date: Option<AssignmentDate>,
    pub due_date: Option<AssignmentDate>,
}

impl Client {
    pub async fn get_exams(&self, course: &Course) -> Result<Vec<Exam>> {
        let url = format!("{}_query", course.id);
        let html = self.get_html(Method::GET, url).await?;

        let selector = Selector::parse("table.stdlist tr:not(.title)").unwrap();
        let exam_element = html.select(&selector);

        let exams = exam_element
            .map(|report_element| {
                let selector = Selector::parse("td").unwrap();
                let mut rows = report_element.select(&selector);

                let title = {
                    let row = rows.next().unwrap();
                    let selector = Selector::parse("a").unwrap();
                    let report_title_element = row.select(&selector).next().unwrap();
                    report_title_element.inner_html()
                };

                let state = {
                    let row = rows.next().unwrap();
                    let selector = Selector::parse("strong").unwrap();

                    if row.select(&selector).next().is_some() {
                        AssignmentState::Done
                    } else {
                        AssignmentState::Todo
                    }
                };

                let start_date = {
                    let row = rows.next().unwrap();
                    let date = row.inner_html();
                    if date.is_empty() {
                        None
                    } else {
                        Some(AssignmentDate::new(&date))
                    }
                };

                let due_date = {
                    let row = rows.next().unwrap();
                    let date = row.inner_html();
                    if date.is_empty() {
                        None
                    } else {
                        Some(AssignmentDate::new(&date))
                    }
                };

                Exam {
                    title,
                    state,
                    start_date,
                    due_date,
                }
            })
            .collect::<Vec<Exam>>();

        Ok(exams)
    }
}
