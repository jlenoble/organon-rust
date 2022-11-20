mod tests;

use chrono::{ DateTime, NaiveDateTime, Utc };

use uuid::Uuid;

use crate::{ Mask, Priority, Recur, Result, Status, TaskError };

#[derive(Debug, PartialEq)]
pub struct Task {
    depends: Vec<Uuid>,
    description: String,
    due: Option<DateTime<Utc>>,
    entry: DateTime<Utc>,
    imask: f64,
    mask: Vec<Mask>,
    modified: Option<DateTime<Utc>>,
    parent: Option<Uuid>,
    priority: Priority,
    project: String,
    recur: Recur,
    scheduled: Option<DateTime<Utc>>,
    status: Status,
    uuid: Uuid,
    wait: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new() -> Self {
        Self {
            depends: vec![],
            description: String::new(),
            due: None,
            entry: Utc::now(),
            imask: f64::NAN,
            mask: vec![],
            modified: None,
            parent: None,
            priority: Priority::NotSet,
            project: String::new(),
            recur: Recur::NotSet,
            scheduled: None,
            status: Status::Pending,
            uuid: Uuid::new_v4(),
            wait: None,
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

impl Task {
    pub fn get_due(&self) -> Option<DateTime<Utc>> {
        self.due
    }

    pub fn set_due(&mut self, value: &str) -> Result<()> {
        self.due = Some(Self::parse_datetime(value)?);
        Ok(())
    }
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

impl Task {
    pub fn get_imask(&self) -> f64 {
        self.imask
    }

    pub fn set_imask(&mut self, value: &str) -> Result<()> {
        self.imask = value
            .parse()
            .or_else(|err| Err(TaskError::FailedToParseIMask(value.to_owned(), err)))?;
        Ok(())
    }
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

impl Task {
    pub fn get_modified(&self) -> Option<DateTime<Utc>> {
        self.modified
    }

    pub fn set_modified(&mut self, value: &str) -> Result<()> {
        self.modified = Some(Self::parse_datetime(value)?);
        Ok(())
    }
}

impl Task {
    pub fn get_parent(&self) -> Option<Uuid> {
        self.parent
    }

    pub fn set_parent(&mut self, value: &str) -> Result<()> {
        self.parent = Some(
            Uuid::parse_str(value).or_else(|err| Err(TaskError::BadUuid(value.to_owned(), err)))?
        );

        Ok(())
    }
}

impl Task {
    pub fn get_priority(&self) -> Priority {
        self.priority
    }

    pub fn set_priority(&mut self, value: &str) -> Result<()> {
        let pri: Priority = value.to_owned().into();

        if pri == Priority::Unknown {
            return Err(TaskError::FailedToParsePriority(value.to_owned()));
        }

        self.priority = pri;
        Ok(())
    }
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

impl Task {
    pub fn get_scheduled(&self) -> Option<DateTime<Utc>> {
        self.scheduled
    }

    pub fn set_scheduled(&mut self, value: &str) -> Result<()> {
        self.scheduled = Some(Self::parse_datetime(value)?);
        Ok(())
    }
}

impl Task {
    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn set_status(&mut self, value: &str) -> Result<()> {
        let stat: Status = value.to_owned().into();

        if stat == Status::Unknown {
            return Err(TaskError::FailedToParseStatus(value.to_owned()));
        }

        self.status = stat;

        Ok(())
    }
}

impl Task {
    pub fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn set_uuid(&mut self, value: &str) -> Result<()> {
        self.uuid = Uuid::parse_str(value).or_else(|err|
            Err(TaskError::BadUuid(value.to_owned(), err))
        )?;

        Ok(())
    }
}

impl Task {
    pub fn get_wait(&self) -> Option<DateTime<Utc>> {
        self.wait
    }

    pub fn set_wait(&mut self, value: &str) -> Result<()> {
        self.wait = Some(Self::parse_datetime(value)?);
        Ok(())
    }
}