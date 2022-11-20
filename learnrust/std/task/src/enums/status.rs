#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Status {
    Unknown,
    Pending,
    Deleted,
    Waiting,
    Recurring,
}

impl Into<String> for Status {
    fn into(self) -> String {
        match self {
            Status::Unknown => "?".to_owned(),
            Status::Pending => "pending".to_owned(),
            Status::Deleted => "deleted".to_owned(),
            Status::Waiting => "waiting".to_owned(),
            Status::Recurring => "recurring".to_owned(),
        }
    }
}

impl Into<Status> for String {
    fn into(self) -> Status {
        match self.as_str() {
            "pending" => Status::Pending,
            "deleted" => Status::Deleted,
            "waiting" => Status::Waiting,
            "recurring" => Status::Recurring,
            _ => Status::Unknown,
        }
    }
}