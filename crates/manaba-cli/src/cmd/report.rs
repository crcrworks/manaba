use crate::color::AppColorize as _;
use crate::{
    cmd::{INDENT, colorize, colorize_bg, date_as_str},
    error::Result,
};
use manaba_sdk::{
    Client, Report,
    assignment::{
        AssignmentDate, AssignmentImportanceLevel, AssignmentReceptibleState, AssignmentSubmitState,
    },
};

pub async fn report(client: &Client, should_show_all: bool, should_show_warn: bool) -> Result<()> {
    let courses = client.get_courses().await?;

    for course in courses {
        let reports = client.get_reports(&course).await?;

        let reports = reports
            .into_iter()
            .filter(|report| {
                if should_show_all {
                    return true;
                }

                if should_show_warn {
                    return matches!(
                        report,
                        Report {
                            submit_state: AssignmentSubmitState::Todo,
                            receptiable_state: AssignmentReceptibleState::Open,
                            due_date: Some(AssignmentDate {
                                importance_level: AssignmentImportanceLevel::High
                                    | AssignmentImportanceLevel::Medium,
                                ..
                            }),
                            ..
                        }
                    );
                }

                matches!(
                    report,
                    Report {
                        submit_state: AssignmentSubmitState::Todo,
                        receptiable_state: AssignmentReceptibleState::Open,
                        ..
                    }
                )
            })
            .collect::<Vec<_>>();

        if reports.is_empty() {
            continue;
        }

        println!("{}", course.title.black().on_blue().with_bold());

        for report in reports {
            let header_str = {
                let status_str = if should_show_all {
                    match report {
                        Report {
                            receptiable_state: AssignmentReceptibleState::NotStarted,
                            ..
                        } => " WAITING ".black().on_gray(),

                        Report {
                            submit_state: AssignmentSubmitState::Todo,
                            receptiable_state: AssignmentReceptibleState::Open,
                            ..
                        } => " TODO ".black().on_red(),

                        Report {
                            submit_state: AssignmentSubmitState::Done,
                            receptiable_state:
                                AssignmentReceptibleState::Closed | AssignmentReceptibleState::Open,
                            ..
                        } => " DONE ".aqua().on_gray(),

                        _ => " CLOSED ".black().on_gray(),
                    }
                } else if let Some(due_date) = &report.due_date {
                    colorize_bg(" ", &report.receptiable_state, &due_date.importance_level)
                } else {
                    String::new()
                };

                let title_str = format!(" {} ", report.title);

                format!("{INDENT}{}{}", status_str, title_str.on_black())
            };

            let start_date_str = report
                .start_date
                .as_ref()
                .map_or(String::new(), date_as_str);

            let due_date_str = report.due_date.as_ref().map_or(String::new(), date_as_str);

            let content = format!(
                "{INDENT}{INDENT}開始: {}\n{INDENT}{INDENT}締切: {}",
                start_date_str, due_date_str
            );

            if let Some(due_date) = &report.due_date {
                println!(
                    "{}\n{}",
                    header_str,
                    colorize(
                        content,
                        &report.receptiable_state,
                        &due_date.importance_level
                    )
                );
            } else {
                println!("{}\n{}", header_str, content);
            }
        }

        println!();
    }

    Ok(())
}
