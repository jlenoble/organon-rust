use std::io;
use chrono::{ DateTime, Utc };
use crate::{ Entry, parse_task_line, Result, Task, TaskError };

pub struct ChangeEntry {
    time: DateTime<Utc>,
    new: Task,
    old: Option<Task>,
}

impl ChangeEntry {
    pub fn new(time: DateTime<Utc>, new: Task, old: Option<Task>) -> Self {
        Self { time, new, old }
    }

    pub fn unwrap_line(line: Option<io::Result<String>>) -> Result<String> {
        if line.is_none() {
            return Err(TaskError::PrematureEndOfFile);
        }

        let line = line.unwrap().or_else(|err| Err(TaskError::FailedToReadLine(err)))?;

        Ok(line)
    }

    pub fn parse_time_line(line: String) -> Result<DateTime<Utc>> {
        let (key, datetime) = Entry::split_at(line.as_str(), ' ')?;

        if key != "time" {
            return Err(
                TaskError::BadLineFormat(line, "Expected leading keyword `time`".to_owned())
            );
        }

        Ok(Task::parse_datetime(datetime)?)
    }

    pub fn parse_task_line(line: String, keyword: &'static str) -> Result<Task> {
        let (key, task) = Entry::split_at(line.as_str(), ' ')?;

        if key != keyword {
            return Err(
                TaskError::BadLineFormat(line, format!("Expected leading keyword `{}`", keyword))
            );
        }

        Ok(parse_task_line(task.to_owned())?)
    }

    pub fn parse_dash3(line: &String) -> Result<()> {
        if line == "---" {
            Ok(())
        } else {
            Err(TaskError::BadLineFormat(line.clone(), "Expected `---` separator".to_owned()))
        }
    }
}

impl ChangeEntry {
    pub fn get_time(&self) -> DateTime<Utc> {
        self.time
    }

    pub fn get_new(&self) -> &Task {
        &self.new
    }

    pub fn get_old(&self) -> Option<&Task> {
        if let Some(ref old) = self.old { Some(old) } else { None }
    }
}