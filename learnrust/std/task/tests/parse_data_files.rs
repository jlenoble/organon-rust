use std::path::Path;
use shellexpand;

use task::{ parse_task_data_file, Result, TaskError };

#[test]
fn parse_backlog_data() -> Result<()> {
    let path = "~/.task/backlog.data";
    let path = shellexpand
        ::full(path)
        .or_else(|err| Err(TaskError::FailedToExpandPath(path.to_owned(), err)))?;
    let path = Path::new(&*path);

    parse_task_data_file(path)?;

    Ok(())
}

#[test]
fn parse_completed_data() -> Result<()> {
    let path = "~/.task/completed.data";
    let path = shellexpand
        ::full(path)
        .or_else(|err| Err(TaskError::FailedToExpandPath(path.to_owned(), err)))?;
    let path = Path::new(&*path);

    parse_task_data_file(path)?;

    Ok(())
}

#[test]
fn parse_pending_data() -> Result<()> {
    let path = "~/.task/pending.data";
    let path = shellexpand
        ::full(path)
        .or_else(|err| Err(TaskError::FailedToExpandPath(path.to_owned(), err)))?;
    let path = Path::new(&*path);

    parse_task_data_file(path)?;

    Ok(())
}

#[test]
fn parse_undo_data() -> Result<()> {
    let path = "~/.task/undo.data";
    let path = shellexpand
        ::full(path)
        .or_else(|err| Err(TaskError::FailedToExpandPath(path.to_owned(), err)))?;
    let path = Path::new(&*path);

    parse_task_data_file(path)?;

    Ok(())
}