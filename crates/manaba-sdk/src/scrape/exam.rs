use crate::Client;
use crate::assignment::{AssignmentDate, AssignmentReceptibleState, AssignmentSubmitState};
use crate::{Course, error::Result};
use reqwest::Method;
use scraper::Selector;

pub struct Exam {
    pub title: String,
    pub submit_state: AssignmentSubmitState,
    pub receptiable_state: AssignmentReceptibleState,
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

                let (receptiable_state, submit_state) = {
                    let row = rows.next().unwrap();

                    let receptiable_state = match row.text().next().unwrap().trim() {
                        "受付中" => AssignmentReceptibleState::Open,
                        "受付終了" => AssignmentReceptibleState::Closed,
                        _ => AssignmentReceptibleState::NotStarted,
                    };

                    let selector = Selector::parse("span").unwrap();
                    let submit_state = row.select(&selector).next().map_or(
                        AssignmentSubmitState::Done,
                        |submit_state| {
                            if submit_state.inner_html().trim() == "未提出" {
                                AssignmentSubmitState::Todo
                            } else {
                                AssignmentSubmitState::Done
                            }
                        },
                    );

                    (receptiable_state, submit_state)
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
                    receptiable_state,
                    submit_state,
                    start_date,
                    due_date,
                }
            })
            .collect::<Vec<Exam>>();

        Ok(exams)
    }
}
