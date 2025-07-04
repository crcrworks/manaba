use chrono::NaiveDateTime;

#[derive(Debug, PartialEq, Eq, Default)]
pub enum AssignmentSubmitState {
    #[default]
    Todo,
    Done,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum AssignmentReceptibleState {
    #[default]
    NotStarted,
    Open,
    Closed,
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
            let three_days_later = today + chrono::Duration::days(3);
            let one_week_later = today + chrono::Duration::days(7);

            if due_date <= three_days_later {
                AssignmentImportanceLevel::High
            } else if due_date <= one_week_later {
                AssignmentImportanceLevel::Medium
            } else {
                AssignmentImportanceLevel::Low
            }
        }
    }
}
