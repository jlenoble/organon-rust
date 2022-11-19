use std::{ fs::File, io::{ BufRead, BufReader }, path::Path };
use crate::{ Result, Task, TaskError };

pub fn parse_task_data_file(path: &Path) -> Result<Vec<Task>> {
    let mut tasks: Vec<Task> = vec![];

    let f = File::open(path).or_else(|err|
        Err(TaskError::FailedToOpenFile(path.to_str().unwrap_or("").to_owned(), err))
    )?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.or_else(|err| Err(TaskError::FailedToReadLine(err)))?;

        if line.as_bytes()[0] == b'[' && line.as_bytes()[line.as_bytes().len() - 1] == b']' {
            let line = &line[1..line.len() - 1];
            let props = line.split(' ');
            let mut task = Task::new();

            for prop in props {
                if let Some(pos) = prop.find(':') {
                    let key = &prop[0..pos];
                    let value = &prop[pos + 1..];

                    match key {
                        "depends" => {
                            task.set_depends(value)?;
                        }
                        _ => {
                            return Err(TaskError::UnknownKey(key.to_owned()));
                        }
                    }
                } else {
                    return Err(
                        TaskError::BadPropertyFormat(prop.to_owned(), "Missing `:`".to_owned())
                    );
                }
            }

            tasks.push(task);
        } else {
            return Err(TaskError::BadLineFormat(line, "Not within brackets `[...]`".to_owned()));
        }
    }

    Ok(tasks)
}