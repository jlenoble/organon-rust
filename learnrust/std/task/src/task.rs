use uuid::Uuid;

use crate::{ Result, TaskError };

#[derive(Debug, PartialEq)]
pub struct Task {
    depends: Vec<Uuid>,
}

impl Task {
    pub fn new() -> Self {
        Self {
            depends: vec![],
        }
    }
}

impl Task {
    pub fn get_depends(&self) -> &Vec<Uuid> {
        &self.depends
    }

    pub fn set_depends(&mut self, value: &str) -> Result<()> {
        self.depends.clear();

        if value.as_bytes()[0] == b'"' && value.as_bytes()[value.as_bytes().len() - 1] == b'"' {
            let value = &value[1..value.len() - 1];

            for dep in value.split(',') {
                let uuid = Uuid::parse_str(dep).or_else(|err|
                    Err(TaskError::BadUuid(dep.to_owned(), err))
                )?;

                self.depends.push(uuid);
            }
        } else {
            return Err(TaskError::UnquotedString(value.to_owned()));
        }

        Ok(())
    }
}

#[test]
fn can_set_depends_property() {
    use uuid::uuid;

    let mut task = Task::new();

    assert!(task.set_depends("dummy string").is_err());

    assert!(task.set_depends("67e55044-10b1-426f-9247-bb680e5fe0c8").is_err());

    assert!(
        task
            .set_depends(
                "67e55044-10b1-426f-9247-bb680e5fe0c8,91ebfab9-5d73-408a-bfc4-5c0652e55cee"
            )
            .is_err()
    );

    assert!(task.set_depends("\"67e55044-10b1-426f-9247-bb680e5fe0c8\"").is_ok());
    assert_eq!(*task.get_depends(), vec![uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8")]);

    assert!(
        task
            .set_depends(
                "\"67e55044-10b1-426f-9247-bb680e5fe0c8,91ebfab9-5d73-408a-bfc4-5c0652e55cee\""
            )
            .is_ok()
    );
    assert_eq!(
        *task.get_depends(),
        vec![
            uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
            uuid!("91ebfab9-5d73-408a-bfc4-5c0652e55cee")
        ]
    );
}