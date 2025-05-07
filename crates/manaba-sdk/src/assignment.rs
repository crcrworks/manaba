use chrono::NaiveDateTime;

#[derive(Debug, PartialEq, Eq)]
pub enum AssignmentState {
    Todo,
    Done,
}

#[derive(Debug)]
pub struct AssignmentDate {
    pub date: NaiveDateTime,
    pub importance_level: AssignmentImportanceLevel,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AssignmentImportanceLevel {
    None,
    Low,
    Medium,
    High,
}

impl AssignmentDate {
    pub fn new(date: &str) -> Self {
        const DATE_FORMAT: &str = "%Y-%m-%d %H:%M";
        let date = NaiveDateTime::parse_from_str(date, DATE_FORMAT).unwrap();
        let importance_level = AssignmentImportanceLevel::from(date);

        AssignmentDate {
            date,
            importance_level,
        }
    }
}

impl From<NaiveDateTime> for AssignmentImportanceLevel {
    fn from(date: NaiveDateTime) -> Self {
        let today: chrono::NaiveDate = chrono::Local::now().date_naive();
        let due_date: chrono::NaiveDate = date.date();

        if due_date < today {
            AssignmentImportanceLevel::None
        } else {
            let tomorrow = today + chrono::Duration::days(1);
            let one_week_later = today + chrono::Duration::days(7);

            if due_date <= tomorrow {
                AssignmentImportanceLevel::High
            } else if due_date <= one_week_later {
                AssignmentImportanceLevel::Medium
            } else {
                AssignmentImportanceLevel::Low
            }
        }
    }
}
