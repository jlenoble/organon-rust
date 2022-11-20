use std::{ fs::File, io::{ BufRead, BufReader }, path::Path };
use crate::{ Entry, Result, Task, TaskError };

pub fn parse_task_data_file(path: &Path) -> Result<Vec<Task>> {
    let mut tasks: Vec<Task> = vec![];

    let f = File::open(path).or_else(|err|
        Err(TaskError::FailedToOpenFile(path.to_str().unwrap_or("").to_owned(), err))
    )?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let mut entry = Entry::new(line.or_else(|err| Err(TaskError::FailedToReadLine(err)))?);
        entry.parse()?;

        let props = entry.get_props();
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
                _ => {
                    return Err(TaskError::UnknownKey(key.to_owned()));
                }
            }
        }

        tasks.push(task);
    }

    Ok(tasks)
}