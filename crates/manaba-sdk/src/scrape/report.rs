use crate::assignment::{AssignmentDate, AssignmentReceptibleState, AssignmentSubmitState};
use crate::error::Result;
use crate::{Client, Course};
use scraper::Selector;

#[derive(Debug)]
pub struct Report {
    pub title: String,
    pub submit_state: AssignmentSubmitState,
    pub receptiable_state: AssignmentReceptibleState,
    pub start_date: Option<AssignmentDate>,
    pub due_date: Option<AssignmentDate>,
}

impl Client {
    pub async fn get_reports(&self, course: &Course) -> Result<Vec<Report>> {
        let url = format!("{}_report", course.id);
        let html = self.get_html(reqwest::Method::GET, url).await?;

        let selector = Selector::parse("table.stdlist tr:not(.title)")?;
        let reports_element = html.select(&selector);

        let reports = reports_element
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

                    let selector = Selector::parse("div").unwrap();
                    let receptiable_state = row.select(&selector).next().map_or(
                        AssignmentReceptibleState::NotStarted,
                        |state| match state.inner_html().as_str() {
                            "受付中" => AssignmentReceptibleState::Open,
                            "受付終了" => AssignmentReceptibleState::Closed,
                            _ => AssignmentReceptibleState::NotStarted,
                        },
                    );

                    let selector = Selector::parse("span").unwrap();
                    let submit_state = row.select(&selector).next().map_or(
                        AssignmentSubmitState::Done,
                        |submit_state| {
                            if submit_state.inner_html() == "未提出" {
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

                Report {
                    title,
                    receptiable_state,
                    submit_state,
                    start_date,
                    due_date,
                }
            })
            .collect::<Vec<Report>>();

        Ok(reports)
    }
}
