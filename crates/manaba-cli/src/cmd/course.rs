use crate::{color::AppColorize as _, error::Result};
use dialoguer::{theme::ColorfulTheme, Select};
use manaba_sdk::{
    Client, Course,
    assignment::{AssignmentImportanceLevel, AssignmentReceptibleState, AssignmentSubmitState},
};

async fn count_assignments_by_urgency(client: &Client, course: &Course) -> Result<(usize, usize, usize)> {
    let mut high_count = 0;
    let mut medium_count = 0;
    let mut low_count = 0;

    // Count reports
    let reports = client.get_reports(course).await?;
    for report in reports {
        if matches!(
            report,
            manaba_sdk::Report {
                submit_state: AssignmentSubmitState::Todo,
                receptiable_state: AssignmentReceptibleState::Open,
                ..
            }
        ) {
            if let Some(due_date) = &report.due_date {
                match due_date.importance_level {
                    AssignmentImportanceLevel::High => high_count += 1,
                    AssignmentImportanceLevel::Medium => medium_count += 1,
                    AssignmentImportanceLevel::Low => low_count += 1,
                    AssignmentImportanceLevel::None => {},
                }
            }
        }
    }

    // Count exams
    let exams = client.get_exams(course).await?;
    for exam in exams {
        if matches!(
            exam,
            manaba_sdk::Exam {
                submit_state: AssignmentSubmitState::Todo,
                receptiable_state: AssignmentReceptibleState::Open,
                ..
            }
        ) {
            if let Some(due_date) = &exam.due_date {
                match due_date.importance_level {
                    AssignmentImportanceLevel::High => high_count += 1,
                    AssignmentImportanceLevel::Medium => medium_count += 1,
                    AssignmentImportanceLevel::Low => low_count += 1,
                    AssignmentImportanceLevel::None => {},
                }
            }
        }
    }

    Ok((high_count, medium_count, low_count))
}

fn format_assignment_indicator(high: usize, medium: usize, low: usize) -> String {
    let high_str = if high > 0 { high.to_string().red() } else { "0".gray() };
    let medium_str = if medium > 0 { medium.to_string().yellow() } else { "0".gray() };
    let low_str = if low > 0 { low.to_string().aqua() } else { "0".gray() };
    
    format!("({}/{}/{}) ", high_str, medium_str, low_str)
}

pub async fn course(client: &Client) -> Result<()> {
    let courses = client.get_courses().await?;
    
    if courses.is_empty() {
        println!("No courses found.");
        return Ok(());
    }

    println!("{}", "Loading assignment information...".gray());
    
    // Collect assignment counts for each course
    let mut course_display_items = Vec::new();
    for course in &courses {
        let (high, medium, low) = count_assignments_by_urgency(client, course).await?;
        let indicator = format_assignment_indicator(high, medium, low);
        let display_title = format!("{}{}", indicator, course.title);
        course_display_items.push(display_title);
    }
    
    let total_courses = courses.len();
    let max_display = 10;
    
    let prompt = "Select a course to open";
    
    // Display course count information
    if total_courses > max_display {
        println!("{}", format!("Found {} courses (showing {} at a time)", total_courses, max_display).to_string().aqua());
        println!("{}", "Use ↑↓ to navigate, PgUp/PgDn for pages, Enter to select".to_string().gray());
        println!("{}", "Assignment counts: (urgent/1week/normal)".to_string().gray());
        println!();
    } else {
        println!("{}", format!("Found {} courses", total_courses).to_string().aqua());
        println!("{}", "Use ↑↓ to navigate, Enter to select".to_string().gray());
        println!("{}", "Assignment counts: (urgent/1week/normal)".to_string().gray());
        println!();
    }
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .max_length(max_display)
        .items(&course_display_items)
        .interact()?;

    let selected_course = &courses[selection];
    let course_url = selected_course.url(&client.base_url);
    
    println!("Opening course: {}", selected_course.title.green());
    opener::open(&course_url)?;
    
    Ok(())
}