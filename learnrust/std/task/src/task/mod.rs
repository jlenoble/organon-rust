mod tests;

use std::collections::HashMap;

use chrono::{ DateTime, NaiveDateTime, Utc };

use uuid::Uuid;

use crate::{ Entry, Mask, Priority, Recur, Result, Status, TaskError };

#[derive(Debug, PartialEq)]
pub struct Task {
    annotations: HashMap<DateTime<Utc>, String>,
    depends: Vec<Uuid>,
    description: String,
    due: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    entry: DateTime<Utc>,
    imask: f64,
    mask: Vec<Mask>,
    modified: Option<DateTime<Utc>>,
    parent: Option<Uuid>,
    priority: Priority,
    project: String,
    recur: Recur,
    scheduled: Option<DateTime<Utc>>,
    start: Option<DateTime<Utc>>,
    status: Status,
    subtask_of: Option<Uuid>,
    subtasks: Vec<Uuid>,
    tags: Vec<String>,
    until: Option<DateTime<Utc>>,
    uuid: Uuid,
    wait: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new() -> Self {
        Self {
            annotations: HashMap::new(),
            depends: vec![],
            description: String::new(),
            due: None,
            end: None,
            entry: Utc::now(),
            imask: f64::NAN,
            mask: vec![],
            modified: None,
            parent: None,
            priority: Priority::NotSet,
            project: String::new(),
            recur: Recur::NotSet,
            scheduled: None,
            start: None,
            status: Status::Pending,
            subtask_of: None,
            subtasks: vec![],
            tags: vec![],
            until: None,
            uuid: Uuid::new_v4(),
            wait: None,
        }
    }

    pub fn build(props: &Vec<(String, String)>) -> Result<Task> {
        let mut task = Task::new();

        for (key, value) in props {
            match key.as_str() {
                "depends" => {
                    task.set_depends(value)?;
                }
                "description" => {
                    task.set_description(value)?;
                }
                "due" => {
                    task.set_due(value)?;
                }
                "end" => {
                    task.set_end(value)?;
                }
                "entry" => {
                    task.set_entry(value)?;
                }
                "imask" => {
                    task.set_imask(value)?;
                }
                "mask" => {
                    task.set_mask(value)?;
                }
                "modified" => {
                    task.set_modified(value)?;
                }
                "parent" => {
                    task.set_parent(value)?;
                }
                "priority" => {
                    task.set_priority(value)?;
                }
                "project" => {
                    task.set_project(value)?;
                }
                "recur" => {
                    task.set_recur(value)?;
                }
                "scheduled" => {
                    task.set_scheduled(value)?;
                }
                "start" => {
                    task.set_start(value)?;
                }
                "status" => {
                    task.set_status(value)?;
                }
                "subtask_of" => {
                    task.set_subtask_of(value)?;
                }
                "subtasks" => {
                    task.set_subtasks(value)?;
                }
                "tags" => {
                    task.set_tags(value)?;
                }
                "until" => {
                    task.set_until(value)?;
                }
                "uuid" => {
                    task.set_uuid(value)?;
                }
                "wait" => {
                    task.set_wait(value)?;
                }
                key => {
                    if let Ok((key, dt)) = Entry::split_at(key, '_') {
                        if key == "annotation" {
                            task.set_annotation(dt, value)?;
                            continue;
                        } else if key == "tags" {
                            // future format for 3.0, ignore for now
                            continue;
                        }
                    }

                    return Err(TaskError::UnknownKey(key.to_owned()));
                }
            }
        }

        Ok(task)
    }

    pub fn parse_datetime(value: &str) -> Result<DateTime<Utc>> {
        if let Ok(timestamp) = value.parse::<i64>() {
            if let Some(naive) = NaiveDateTime::from_timestamp_opt(timestamp, 0) {
                return Ok(DateTime::from_utc(naive, Utc));
            }
        } else if value.len() == 16 {
            let dt =
                String::from(&value[0..4]) +
                "-" +
                &value[4..6] +
                "-" +
                &value[6..11] +
                ":" +
                &value[11..13] +
                ":" +
                &value[13..];
            if let Ok(dt) = dt.parse::<DateTime<Utc>>() {
                return Ok(dt);
            }
        }

        Err(TaskError::FailedToParseDateTime(value.to_owned()))
    }
}

impl Task {
    pub fn get_annotations(&self) -> &HashMap<DateTime<Utc>, String> {
        &self.annotations
    }

    pub fn set_annotation(&mut self, datetime: &str, value: &str) -> Result<()> {
        let datetime = Self::parse_datetime(datetime)?;

        if value.is_empty() {
            return Err(TaskError::EmptyString);
        }

        self.annotations.insert(datetime, value.to_owned());

        Ok(())
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
    pub fn get_end(&self) -> Option<DateTime<Utc>> {
        self.end
    }

    pub fn set_end(&mut self, value: &str) -> Result<()> {
        self.end = Some(Self::parse_datetime(value)?);
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
    pub fn get_start(&self) -> Option<DateTime<Utc>> {
        self.start
    }

    pub fn set_start(&mut self, value: &str) -> Result<()> {
        self.start = Some(Self::parse_datetime(value)?);
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
    pub fn get_subtask_of(&self) -> Option<Uuid> {
        self.subtask_of
    }

    pub fn set_subtask_of(&mut self, value: &str) -> Result<()> {
        self.subtask_of = Some(
            Uuid::parse_str(value).or_else(|err| Err(TaskError::BadUuid(value.to_owned(), err)))?
        );

        Ok(())
    }
}

impl Task {
    pub fn get_subtasks(&self) -> &Vec<Uuid> {
        &self.subtasks
    }

    pub fn set_subtasks(&mut self, value: &str) -> Result<()> {
        self.subtasks.clear();

        for dep in value.split(',') {
            let uuid = Uuid::parse_str(dep).or_else(|err|
                Err(TaskError::BadUuid(dep.to_owned(), err))
            )?;

            self.subtasks.push(uuid);
        }

        Ok(())
    }
}

impl Task {
    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn set_tags(&mut self, value: &str) -> Result<()> {
        self.tags = value
            .split(',')
            .map(|s| s.to_owned())
            .collect();

        Ok(())
    }
}

impl Task {
    pub fn get_until(&self) -> Option<DateTime<Utc>> {
        self.until
    }

    pub fn set_until(&mut self, value: &str) -> Result<()> {
        self.until = Some(Self::parse_datetime(value)?);
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
