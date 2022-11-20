use uuid::Uuid;

use crate::{ Result, TaskError };

#[derive(Debug, PartialEq)]
pub struct Task {
    depends: Vec<Uuid>,
    description: String,
}

impl Task {
    pub fn new() -> Self {
        Self {
            depends: vec![],
            description: String::new(),
        }
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

#[test]
fn can_set_depends_property() {
    use uuid::uuid;

    let mut task = Task::new();

    assert!(task.set_depends("dummy string").is_err());

    assert!(task.set_depends("\"67e55044-10b1-426f-9247-bb680e5fe0c8\"").is_err());

    assert!(
        task
            .set_depends(
                "\"67e55044-10b1-426f-9247-bb680e5fe0c8,91ebfab9-5d73-408a-bfc4-5c0652e55cee\""
            )
            .is_err()
    );

    assert!(task.set_depends("67e55044-10b1-426f-9247-bb680e5fe0c8").is_ok());
    assert_eq!(*task.get_depends(), vec![uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8")]);

    assert!(
        task
            .set_depends(
                "67e55044-10b1-426f-9247-bb680e5fe0c8,91ebfab9-5d73-408a-bfc4-5c0652e55cee"
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

#[test]
fn can_set_description_property() {
    let mut task = Task::new();

    assert!(task.set_description("unquoted string").is_ok());
    assert_eq!(*task.get_description(), "unquoted string");

    assert!(task.set_description("\"quoted string\"").is_ok());
    assert_eq!(*task.get_description(), "\"quoted string\"");
}