use crate::{
    cmd::{INDENT, colorize, colorize_bg, date_as_str},
    error::Result,
};
use colored_text::Colorize;
use manaba_sdk::{
    Client, Exam,
    assignment::{
        AssignmentDate, AssignmentImportanceLevel, AssignmentReceptibleState, AssignmentSubmitState,
    },
};

pub async fn exam(client: &Client, should_show_all: bool, should_show_warn: bool) -> Result<()> {
    let courses = client.get_courses().await?;

    for course in courses {
        let exams = client.get_exams(&course).await?;

        let exams = exams
            .into_iter()
            .filter(|report| {
                if should_show_all {
                    return true;
                }

                if should_show_warn {
                    return matches!(
                        report,
                        Exam {
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
                    Exam {
                        submit_state: AssignmentSubmitState::Todo,
                        receptiable_state: AssignmentReceptibleState::Open,
                        ..
                    }
                )
            })
            .collect::<Vec<_>>();

        if exams.is_empty() {
            continue;
        }

        println!("{}", course.title.black().bold().on_blue());

        for exam in exams {
            let header_str = {
                let status_str = if should_show_all {
                    match exam {
                        Exam {
                            receptiable_state: AssignmentReceptibleState::NotStarted,
                            ..
                        } => " WAITING ".black().on_white(),

                        Exam {
                            submit_state: AssignmentSubmitState::Todo,
                            receptiable_state: AssignmentReceptibleState::Open,
                            ..
                        } => " TODO ".black().on_red(),

                        Exam {
                            submit_state: AssignmentSubmitState::Done,
                            receptiable_state:
                                AssignmentReceptibleState::Closed | AssignmentReceptibleState::Open,
                            ..
                        } => " DONE ".cyan().on_white(),

                        _ => " CLOSED ".black().on_white(),
                    }
                } else if let Some(due_date) = &exam.due_date {
                    colorize_bg(" ", &exam.receptiable_state, &due_date.importance_level)
                } else {
                    String::new()
                };

                let title_str = format!(" {} ", exam.title);

                format!("{INDENT}{}{}", status_str, title_str.on_black())
            };

            let start_date_str = exam.start_date.as_ref().map_or(String::new(), date_as_str);

            let due_date_str = exam.due_date.as_ref().map_or(String::new(), date_as_str);

            let content = format!(
                "{INDENT}{INDENT}開始: {start_date_str}\n{INDENT}{INDENT}締切: {due_date_str}"
            );

            if let Some(due_date) = &exam.due_date {
                println!(
                    "{}\n{}",
                    header_str,
                    colorize(content, &exam.receptiable_state, &due_date.importance_level)
                );
            } else {
                println!("{}\n{}", header_str, content);
            }
        }

        println!();
    }

    Ok(())
}
