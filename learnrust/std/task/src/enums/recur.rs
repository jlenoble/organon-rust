#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Recur {
    Unknown,
    NotSet,
    Daily,
    Weekly,
    Monthly,
    Quarterly,
}

impl Into<String> for Recur {
    fn into(self) -> String {
        match self {
            Recur::Unknown => "?".to_owned(),
            Recur::NotSet => String::new(),
            Recur::Daily => "daily".to_owned(),
            Recur::Weekly => "weekly".to_owned(),
            Recur::Monthly => "monthly".to_owned(),
            Recur::Quarterly => "quarterly".to_owned(),
        }
    }
}

impl Into<Recur> for String {
    fn into(self) -> Recur {
        match self.as_str() {
            "" => Recur::NotSet,
            "daily" => Recur::Daily,
            "weekly" => Recur::Weekly,
            "monthly" => Recur::Monthly,
            "quarterly" => Recur::Quarterly,
            _ => Recur::Unknown,
        }
    }
}