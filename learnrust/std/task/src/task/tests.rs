#[test]
fn can_set_depends_property() {
    use uuid::uuid;
    use crate::Task;

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

#[test]
fn can_set_description_property() {
    use crate::Task;

    let mut task = Task::new();

    assert!(task.set_description("unquoted string").is_ok());
    assert_eq!(*task.get_description(), "unquoted string");

    assert!(task.set_description("\"quoted string\"").is_ok());
    assert_eq!(*task.get_description(), "\"quoted string\"");
}

#[test]
fn can_set_due_property() {
    use chrono::{ FixedOffset, TimeZone };
    use crate::Task;

    let mut task = Task::new();

    assert!(task.set_due("bad date").is_err());

    assert!(task.set_due("1669028400").is_ok());
    assert_eq!(
        task.get_due().unwrap(),
        FixedOffset::east_opt(3600).unwrap().with_ymd_and_hms(2022, 11, 21, 12, 0, 0).unwrap()
    )
}

#[test]
fn can_set_entry_property() {
    use chrono::{ FixedOffset, TimeZone };
    use crate::Task;

    let mut task = Task::new();

    assert!(task.set_entry("bad date").is_err());

    assert!(task.set_entry("1669028400").is_ok());
    assert_eq!(
        task.get_entry(),
        FixedOffset::east_opt(3600).unwrap().with_ymd_and_hms(2022, 11, 21, 12, 0, 0).unwrap()
    )
}

#[test]
fn can_set_mask_property() {
    use crate::{ Mask, Task };

    let mut task = Task::new();

    assert!(task.set_mask("bad mask").is_err());

    assert!(task.set_mask("").is_ok());
    assert_eq!(*task.get_mask(), vec![]);

    assert!(task.set_mask("-").is_ok());
    assert_eq!(*task.get_mask(), vec![Mask::Pending]);

    assert!(task.set_mask("+").is_ok());
    assert_eq!(*task.get_mask(), vec![Mask::Completed]);

    assert!(task.set_mask("X").is_ok());
    assert_eq!(*task.get_mask(), vec![Mask::Deleted]);

    assert!(task.set_mask("W").is_ok());
    assert_eq!(*task.get_mask(), vec![Mask::Waiting]);

    assert!(task.set_mask("+-").is_ok());
    assert_eq!(*task.get_mask(), vec![Mask::Completed, Mask::Pending]);

    assert!(task.set_mask("+--").is_ok());
    assert_eq!(*task.get_mask(), vec![Mask::Completed, Mask::Pending, Mask::Pending]);

    assert!(task.set_mask("++X+W-").is_ok());
    assert_eq!(
        *task.get_mask(),
        vec![
            Mask::Completed,
            Mask::Completed,
            Mask::Deleted,
            Mask::Completed,
            Mask::Waiting,
            Mask::Pending
        ]
    );

    assert!(task.set_mask("+XXX++WXWW--XW").is_ok());
    assert_eq!(
        *task.get_mask(),
        vec![
            Mask::Completed,
            Mask::Deleted,
            Mask::Deleted,
            Mask::Deleted,
            Mask::Completed,
            Mask::Completed,
            Mask::Waiting,
            Mask::Deleted,
            Mask::Waiting,
            Mask::Waiting,
            Mask::Pending,
            Mask::Pending,
            Mask::Deleted,
            Mask::Waiting
        ]
    );

    assert!(task.set_mask("+XXX++WXWW--XW!").is_err());
    assert!(task.set_mask("+XXX++W!WW--XW").is_err());
    assert!(task.set_mask("!+XXX++WXWW--XW").is_err());
}

#[test]
fn can_set_modified_property() {
    use chrono::{ FixedOffset, TimeZone };
    use crate::Task;

    let mut task = Task::new();

    assert!(task.set_modified("bad date").is_err());

    assert!(task.set_modified("1669028400").is_ok());
    assert_eq!(
        task.get_modified().unwrap(),
        FixedOffset::east_opt(3600).unwrap().with_ymd_and_hms(2022, 11, 21, 12, 0, 0).unwrap()
    )
}

#[test]
fn can_set_priority_property() {
    use crate::{ Priority, Task };

    let mut task = Task::new();

    assert!(task.set_priority("").is_ok());
    assert_eq!(task.get_priority(), Priority::NotSet);

    assert!(task.set_priority("foo").is_err());

    assert!(task.set_priority("L").is_ok());
    assert_eq!(task.get_priority(), Priority::Low);

    assert!(task.set_priority("M").is_ok());
    assert_eq!(task.get_priority(), Priority::Medium);

    assert!(task.set_priority("H").is_ok());
    assert_eq!(task.get_priority(), Priority::High);
}

#[test]
fn can_set_project_property() {
    use crate::Task;

    let mut task = Task::new();

    assert!(task.set_project("unquoted string").is_ok());
    assert_eq!(*task.get_project(), "unquoted string");

    assert!(task.set_project("\"quoted string\"").is_ok());
    assert_eq!(*task.get_project(), "\"quoted string\"");
}

#[test]
fn can_set_recur_property() {
    use crate::{ Recur, Task };

    let mut task = Task::new();

    assert!(task.set_recur("").is_ok());
    assert_eq!(task.get_recur(), Recur::NotSet);

    assert!(task.set_recur("daily").is_ok());
    assert_eq!(task.get_recur(), Recur::Daily);

    assert!(task.set_recur("weekly").is_ok());
    assert_eq!(task.get_recur(), Recur::Weekly);

    assert!(task.set_recur("monthly").is_ok());
    assert_eq!(task.get_recur(), Recur::Monthly);

    assert!(task.set_recur("foo").is_err());
}

#[test]
fn can_set_scheduled_property() {
    use chrono::{ FixedOffset, TimeZone };
    use crate::Task;

    let mut task = Task::new();

    assert!(task.set_scheduled("bad date").is_err());

    assert!(task.set_scheduled("1669028400").is_ok());
    assert_eq!(
        task.get_scheduled().unwrap(),
        FixedOffset::east_opt(3600).unwrap().with_ymd_and_hms(2022, 11, 21, 12, 0, 0).unwrap()
    )
}

#[test]
fn can_set_status_property() {
    use crate::{ Status, Task };

    let mut task = Task::new();

    assert!(task.set_status("").is_err());

    assert!(task.set_status("foo").is_err());

    assert!(task.set_status("pending").is_ok());
    assert_eq!(task.get_status(), Status::Pending);

    assert!(task.set_status("recurring").is_ok());
    assert_eq!(task.get_status(), Status::Recurring);
}

#[test]
fn can_set_uuid_property() {
    use uuid::uuid;
    use crate::Task;

    let mut task = Task::new();

    assert!(task.set_uuid("dummy string").is_err());

    assert!(task.set_uuid("\"67e55044-10b1-426f-9247-bb680e5fe0c8\"").is_err());

    assert!(task.set_uuid("67e55044-10b1-426f-9247-bb680e5fe0c8").is_ok());
    assert_eq!(*task.get_uuid(), uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"));

    assert!(
        task
            .set_uuid(
                "\"67e55044-10b1-426f-9247-bb680e5fe0c8,91ebfab9-5d73-408a-bfc4-5c0652e55cee\""
            )
            .is_err()
    );

    assert!(
        task
            .set_uuid("67e55044-10b1-426f-9247-bb680e5fe0c8,91ebfab9-5d73-408a-bfc4-5c0652e55cee")
            .is_err()
    );
}