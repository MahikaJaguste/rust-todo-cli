use std::io::ErrorKind;
use todo::TodoStatus;

mod common;

#[test]
fn add_item() {
    let mut todo_list = common::setup();

    let before_len = todo_list.list.len();

    assert_eq!(
        todo_list
            .add_item(String::from("test task"), String::from("medi"))
            .unwrap_err()
            .kind(),
        ErrorKind::InvalidData
    );

    todo_list
        .add_item(String::from("test task"), String::from("med"))
        .unwrap();

    let after_len = todo_list.list.len();
    assert_eq!(after_len, before_len + 1);
    assert_eq!(todo_list.list[after_len - 1].status, TodoStatus::Pending);
}

#[test]
fn mark_item_as_done() {
    let mut todo_list = common::setup();

    let before_len: i32 = todo_list.list.len() as i32;

    assert_eq!(
        todo_list.mark_as_done(before_len + 1).unwrap_err().kind(),
        ErrorKind::InvalidData
    );

    todo_list.mark_as_done(1).unwrap();

    assert_eq!(todo_list.list[0].status, TodoStatus::Done);

    assert_eq!(
        todo_list.mark_as_done(1).unwrap_err().kind(),
        ErrorKind::Other
    );
}
