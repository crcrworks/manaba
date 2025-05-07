use crate::error::Result;
use colored_text::Colorize;
use manaba_sdk::{
    Client,
    assignment::{AssignmentDate, AssignmentImportanceLevel, AssignmentState},
};

pub async fn report(client: &Client, should_show_all: bool, should_show_warn: bool) -> Result<()> {
    let courses = client.get_courses().await?;

    for course in courses {
        let reports = client.get_reports(&course).await?;

        let reports = reports
            .into_iter()
            .filter(|report| {
                if should_show_warn {
                    return matches!(
                        report.due_date,
                        Some(AssignmentDate {
                            importance_level: AssignmentImportanceLevel::High,
                            ..
                        })
                    );
                }

                if should_show_all {
                    true
                } else {
                    report.state == AssignmentState::Todo
                }
            })
            .collect::<Vec<_>>();

        if reports.is_empty() {
            continue;
        }

        println!("{}", course.title.blue().bold());

        for report in reports {
            print!("  - ");

            match report.state {
                AssignmentState::Todo => {
                    if should_show_all {
                        print!("{}", "[TODO]".red());
                    }
                }
                AssignmentState::Done => {
                    print!("{}", "[DONE]".green());
                }
            }

            println!(" {}", report.title);

            if let Some(start_date) = &report.start_date {
                print!("    開始: ");
                print!("{}", start_date.date);
                println!();
            }

            if let Some(due_date) = &report.due_date {
                print!("    締切: ");
                print_report_date(due_date)?;
                println!();
            }
        }

        println!();
    }

    Ok(())
}

fn print_report_date(report_date: &AssignmentDate) -> Result<()> {
    let date_string = report_date.date.format("%Y-%m-%d %H:%M").to_string();

    match report_date.importance_level {
        AssignmentImportanceLevel::High => {
            print!("{}", date_string.red());
        }
        AssignmentImportanceLevel::Medium => {
            print!("{}", date_string.yellow());
        }
        AssignmentImportanceLevel::Low => {
            print!("{}", date_string.cyan());
        }
        AssignmentImportanceLevel::None => {
            print!("{}", date_string);
        }
    }
    Ok(())
}
