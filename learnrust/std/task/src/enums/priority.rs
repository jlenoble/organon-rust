#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Priority {
    Unknown,
    NotSet,
    Low,
    Medium,
    High,
}

impl Into<String> for Priority {
    fn into(self) -> String {
        match self {
            Priority::Unknown => "?".to_owned(),
            Priority::NotSet => "".to_owned(),
            Priority::Low => "L".to_owned(),
            Priority::Medium => "M".to_owned(),
            Priority::High => "H".to_owned(),
        }
    }
}

impl Into<Priority> for String {
    fn into(self) -> Priority {
        match self.as_str() {
            "" => Priority::NotSet,
            "L" => Priority::Low,
            "M" => Priority::Medium,
            "H" => Priority::High,
            _ => Priority::Unknown,
        }
    }
}