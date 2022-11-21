use std::path::Path;
use shellexpand;

use task::{ parse_undo_data_file, Result, TaskError };

fn main() -> Result<()> {
    let path = "~/.task/undo.data";
    let path = shellexpand
        ::full(path)
        .or_else(|err| Err(TaskError::FailedToExpandPath(path.to_owned(), err)))?;
    let path = Path::new(&*path);

    parse_undo_data_file(path)?;

    Ok(())
}