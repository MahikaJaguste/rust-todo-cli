use todo::{TodoItem, TodoList, TodoPriority, TodoStatus};

pub fn setup() -> TodoList {
    let list: Vec<TodoItem> = vec![
        TodoItem {
            title: String::from("task1"),
            priority: TodoPriority::High,
            status: TodoStatus::Pending,
        },
        TodoItem {
            title: String::from("task2"),
            priority: TodoPriority::Low,
            status: TodoStatus::Done,
        },
        TodoItem {
            title: String::from("task3"),
            priority: TodoPriority::Low,
            status: TodoStatus::Pending,
        },
        TodoItem {
            title: String::from("task4"),
            priority: TodoPriority::Medium,
            status: TodoStatus::Pending,
        },
        TodoItem {
            title: String::from("task5"),
            priority: TodoPriority::Medium,
            status: TodoStatus::Done,
        },
    ];

    return TodoList { list };
}
