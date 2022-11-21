use std::{ fs::File, io::{ BufRead, BufReader }, path::Path };
use crate::{ ChangeEntry, Entry, LogEntry, Result, Task, TaskError };

pub fn parse_task_line(line: String) -> Result<Task> {
    let mut entry = Entry::new(line);
    entry.parse()?;
    let props = entry.get_props();
    Task::build(props)
}

pub fn parse_json_task_line(line: String) -> Result<Task> {
    let mut entry = LogEntry::new(line);
    entry.parse()?;
    let props = entry.get_props();
    Task::build(props)
}

/// Parse pending.data and completed.data
pub fn parse_task_data_file(path: &Path) -> Result<Vec<Task>> {
    let mut tasks: Vec<Task> = vec![];

    let f = File::open(path).or_else(|err|
        Err(TaskError::FailedToOpenFile(path.to_str().unwrap_or("").to_owned(), err))
    )?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let task = parse_task_line(line.or_else(|err| Err(TaskError::FailedToReadLine(err)))?)?;
        tasks.push(task);
    }

    Ok(tasks)
}

/// Parse backlog.data
pub fn parse_backlog_data_file(path: &Path) -> Result<Vec<Task>> {
    let mut tasks: Vec<Task> = vec![];

    let f = File::open(path).or_else(|err|
        Err(TaskError::FailedToOpenFile(path.to_str().unwrap_or("").to_owned(), err))
    )?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let task = parse_json_task_line(
            line.or_else(|err| Err(TaskError::FailedToReadLine(err)))?
        )?;
        tasks.push(task);
    }

    Ok(tasks)
}

/// Parse undo.data
pub fn parse_undo_data_file(path: &Path) -> Result<Vec<ChangeEntry>> {
    let mut changes: Vec<ChangeEntry> = vec![];

    let f = File::open(path).or_else(|err|
        Err(TaskError::FailedToOpenFile(path.to_str().unwrap_or("").to_owned(), err))
    )?;
    let f = BufReader::new(f);

    let mut iter = f.lines().into_iter();

    loop {
        let line = iter.next();
        if line.is_none() {
            break;
        }
        let time = ChangeEntry::parse_time_line(ChangeEntry::unwrap_line(line)?)?;

        let task = ChangeEntry::unwrap_line(iter.next())?;
        let dash3 = ChangeEntry::unwrap_line(iter.next())?;

        let (new, old) = if ChangeEntry::parse_dash3(&dash3).is_ok() {
            (ChangeEntry::parse_task_line(task, "new")?, None)
        } else {
            // Consume next line (expecting separator "---") to keep parsing in sync
            ChangeEntry::parse_dash3(&ChangeEntry::unwrap_line(iter.next())?)?;

            (
                ChangeEntry::parse_task_line(dash3, "new")?,
                Some(ChangeEntry::parse_task_line(task, "old")?),
            )
        };

        changes.push(ChangeEntry::new(time, new, old));
    }

    Ok(changes)
}