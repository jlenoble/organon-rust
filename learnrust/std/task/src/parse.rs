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
                "status" => {
                    task.set_status(value)?;
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
                        }
                    }

                    return Err(TaskError::UnknownKey(key.to_owned()));
                }
            }
        }

        tasks.push(task);
    }

    Ok(tasks)
}