use std::path::Path;
use shellexpand;

use task::{ parse_task_data_file, Result, TaskError };

fn main() -> Result<()> {
    let path = "~/.task/pending.data";
    let path = shellexpand
        ::full(path)
        .or_else(|err| Err(TaskError::FailedToExpandPath(path.to_owned(), err)))?;
    let path = Path::new(&*path);

    parse_task_data_file(path)?;

    Ok(())
}