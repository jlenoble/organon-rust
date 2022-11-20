use chrono::{ DateTime, NaiveDateTime, Utc };

use uuid::Uuid;

use crate::{ Mask, Recur, Result, TaskError };

#[derive(Debug, PartialEq)]
pub struct Task {
    depends: Vec<Uuid>,
    description: String,
    due: Option<DateTime<Utc>>,
    entry: DateTime<Utc>,
    mask: Vec<Mask>,
    modified: Option<DateTime<Utc>>,
    project: String,
    recur: Recur,
}

impl Task {
    pub fn new() -> Self {
        Self {
            depends: vec![],
            description: String::new(),
            due: None,
            entry: Utc::now(),
            mask: vec![],
            modified: None,
            project: String::new(),
            recur: Recur::NotSet,
        }
    }

    fn parse_datetime(value: &str) -> Result<DateTime<Utc>> {
        if let Ok(timestamp) = value.parse::<i64>() {
            if let Some(naive) = NaiveDateTime::from_timestamp_opt(timestamp, 0) {
                return Ok(DateTime::from_utc(naive, Utc));
            }
        }

        Err(TaskError::FailedToParseDateTime(value.to_owned()))
    }
}

impl Task {
    pub fn get_depends(&self) -> &Vec<Uuid> {
        &self.depends
    }

    pub fn set_depends(&mut self, value: &str) -> Result<()> {
        self.depends.clear();

        for dep in value.split(',') {
            let uuid = Uuid::parse_str(dep).or_else(|err|
                Err(TaskError::BadUuid(dep.to_owned(), err))
            )?;

            self.depends.push(uuid);
        }

        Ok(())
    }
}

#[test]
fn can_set_depends_property() {
    use uuid::uuid;

    let mut task = Task::new();

    assert!(task.set_depends("dummy string").is_err());

    assert!(task.set_depends("\"67e55044-10b1-426f-9247-bb680e5fe0c8\"").is_err());

    assert!(
        task
            .set_depends(
                "\"67e55044-10b1-426f-9247-bb680e5fe0c8,91ebfab9-5d73-408a-bfc4-5c0652e55cee\""
            )
            .is_err()
    );

    assert!(task.set_depends("67e55044-10b1-426f-9247-bb680e5fe0c8").is_ok());
    assert_eq!(*task.get_depends(), vec![uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8")]);

    assert!(
        task
            .set_depends(
                "67e55044-10b1-426f-9247-bb680e5fe0c8,91ebfab9-5d73-408a-bfc4-5c0652e55cee"
            )
            .is_ok()
    );
    assert_eq!(
        *task.get_depends(),
        vec![
            uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
            uuid!("91ebfab9-5d73-408a-bfc4-5c0652e55cee")
        ]
    );
}

impl Task {
    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn set_description(&mut self, value: &str) -> Result<()> {
        self.description.clear();
        self.description.push_str(value);

        Ok(())
    }
}

#[test]
fn can_set_description_property() {
    let mut task = Task::new();

    assert!(task.set_description("unquoted string").is_ok());
    assert_eq!(*task.get_description(), "unquoted string");

    assert!(task.set_description("\"quoted string\"").is_ok());
    assert_eq!(*task.get_description(), "\"quoted string\"");
}

impl Task {
    pub fn get_due(&self) -> Option<DateTime<Utc>> {
        self.due
    }

    pub fn set_due(&mut self, value: &str) -> Result<()> {
        self.due = Some(Self::parse_datetime(value)?);
        Ok(())
    }
}

#[test]
fn can_set_due_property() {
    use chrono::{ FixedOffset, TimeZone };

    let mut task = Task::new();

    assert!(task.set_due("bad date").is_err());

    assert!(task.set_due("1669028400").is_ok());
    assert_eq!(
        task.get_due().unwrap(),
        FixedOffset::east_opt(3600).unwrap().with_ymd_and_hms(2022, 11, 21, 12, 0, 0).unwrap()
    )
}

impl Task {
    pub fn get_entry(&self) -> DateTime<Utc> {
        self.entry
    }

    pub fn set_entry(&mut self, value: &str) -> Result<()> {
        self.entry = Self::parse_datetime(value)?;
        Ok(())
    }
}

#[test]
fn can_set_entry_property() {
    use chrono::{ FixedOffset, TimeZone };

    let mut task = Task::new();

    assert!(task.set_entry("bad date").is_err());

    assert!(task.set_entry("1669028400").is_ok());
    assert_eq!(
        task.get_entry(),
        FixedOffset::east_opt(3600).unwrap().with_ymd_and_hms(2022, 11, 21, 12, 0, 0).unwrap()
    )
}

impl Task {
    pub fn get_mask(&self) -> &Vec<Mask> {
        &self.mask
    }

    pub fn set_mask(&mut self, value: &str) -> Result<()> {
        let mut mask: Vec<Mask> = vec![];

        for ch in value.as_bytes() {
            let msk: Mask = (*ch).into();

            if msk == Mask::Unknown {
                return Err(TaskError::FailedToParseMask(value.to_owned()));
            }

            mask.push(msk);
        }

        self.mask = mask;
        return Ok(());
    }
}

#[test]
fn can_set_mask_property() {
    let mut task = Task::new();

    assert!(task.set_mask("bad mask").is_err());

    assert!(task.set_mask("").is_ok());
    assert_eq!(*task.get_mask(), vec![]);

    assert!(task.set_mask("-").is_ok());
    assert_eq!(*task.get_mask(), vec![Mask::Pending]);

    assert!(task.set_mask("+").is_ok());
    assert_eq!(*task.get_mask(), vec![Mask::Completed]);

    assert!(task.set_mask("X").is_ok());
    assert_eq!(*task.get_mask(), vec![Mask::Deleted]);

    assert!(task.set_mask("W").is_ok());
    assert_eq!(*task.get_mask(), vec![Mask::Waiting]);

    assert!(task.set_mask("+-").is_ok());
    assert_eq!(*task.get_mask(), vec![Mask::Completed, Mask::Pending]);

    assert!(task.set_mask("+--").is_ok());
    assert_eq!(*task.get_mask(), vec![Mask::Completed, Mask::Pending, Mask::Pending]);

    assert!(task.set_mask("++X+W-").is_ok());
    assert_eq!(
        *task.get_mask(),
        vec![
            Mask::Completed,
            Mask::Completed,
            Mask::Deleted,
            Mask::Completed,
            Mask::Waiting,
            Mask::Pending
        ]
    );

    assert!(task.set_mask("+XXX++WXWW--XW").is_ok());
    assert_eq!(
        *task.get_mask(),
        vec![
            Mask::Completed,
            Mask::Deleted,
            Mask::Deleted,
            Mask::Deleted,
            Mask::Completed,
            Mask::Completed,
            Mask::Waiting,
            Mask::Deleted,
            Mask::Waiting,
            Mask::Waiting,
            Mask::Pending,
            Mask::Pending,
            Mask::Deleted,
            Mask::Waiting
        ]
    );

    assert!(task.set_mask("+XXX++WXWW--XW!").is_err());
    assert!(task.set_mask("+XXX++W!WW--XW").is_err());
    assert!(task.set_mask("!+XXX++WXWW--XW").is_err());
}

impl Task {
    pub fn get_modified(&self) -> Option<DateTime<Utc>> {
        self.modified
    }

    pub fn set_modified(&mut self, value: &str) -> Result<()> {
        self.modified = Some(Self::parse_datetime(value)?);
        Ok(())
    }
}

#[test]
fn can_set_modified_property() {
    use chrono::{ FixedOffset, TimeZone };

    let mut task = Task::new();

    assert!(task.set_modified("bad date").is_err());

    assert!(task.set_modified("1669028400").is_ok());
    assert_eq!(
        task.get_modified().unwrap(),
        FixedOffset::east_opt(3600).unwrap().with_ymd_and_hms(2022, 11, 21, 12, 0, 0).unwrap()
    )
}

impl Task {
    pub fn get_project(&self) -> &String {
        &self.project
    }

    pub fn set_project(&mut self, value: &str) -> Result<()> {
        self.project.clear();
        self.project.push_str(value);
        Ok(())
    }
}

#[test]
fn can_set_project_property() {
    let mut task = Task::new();

    assert!(task.set_project("unquoted string").is_ok());
    assert_eq!(*task.get_project(), "unquoted string");

    assert!(task.set_project("\"quoted string\"").is_ok());
    assert_eq!(*task.get_project(), "\"quoted string\"");
}

impl Task {
    pub fn get_recur(&self) -> Recur {
        self.recur
    }

    pub fn set_recur(&mut self, value: &str) -> Result<()> {
        let rec: Recur = value.to_owned().into();

        if rec == Recur::Unknown {
            return Err(TaskError::FailedToParseRecur(value.to_owned()));
        }

        self.recur = rec;

        Ok(())
    }
}

#[test]
fn can_set_recur_property() {
    let mut task = Task::new();

    assert!(task.set_recur("").is_ok());
    assert_eq!(task.get_recur(), Recur::NotSet);

    assert!(task.set_recur("daily").is_ok());
    assert_eq!(task.get_recur(), Recur::Daily);

    assert!(task.set_recur("weekly").is_ok());
    assert_eq!(task.get_recur(), Recur::Weekly);

    assert!(task.set_recur("monthly").is_ok());
    assert_eq!(task.get_recur(), Recur::Monthly);

    assert!(task.set_recur("foo").is_err());
}