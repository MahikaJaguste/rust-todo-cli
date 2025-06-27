use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{self};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    title: String,
    priority: TodoPriority,
    status: TodoStatus,
}

#[derive(Debug, Serialize, Deserialize)]
enum TodoStatus {
    Pending,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TodoPriority {
    High,
    Medium,
    Low,
}

impl fmt::Display for TodoPriority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TodoPriority::High => write!(f, "High"),
            TodoPriority::Medium => write!(f, "Medium"),
            TodoPriority::Low => write!(f, "Low"),
        }
    }
}

pub struct TodoList {
    pub list: Vec<TodoItem>,
}

impl TodoList {
    pub fn add_item(self: &mut Self, title: String, priority: String) -> Result<(), io::Error> {
        // taking reference (immutable) to title to just access it

        // using ? only when function returns result
        // ? basically returns error in case of Err arm, or does Ok(..) and returns value
        // since these are file operations, error is of type io.Error
        // we have nothing to return, so Ok(()) returns a Result object with no value
        // if it was Result<bool, ...>, then Ok(true) would work
        let priority_result = match priority.as_str() {
            "high" => Ok(TodoPriority::High),
            "medium" => Ok(TodoPriority::Medium),
            "low" => Ok(TodoPriority::Low),
            _ => Ok(TodoPriority::Low),
        };

        // TODO - error handling here
        let priority = match priority_result {
            Ok(p) => p,
            Err(e) => e,
        };

        let item = TodoItem {
            title,
            priority,
            status: TodoStatus::Pending,
        };

        self.list.push(item);

        Ok(())
    }

    pub fn list_items(self: &Self) -> Result<(), io::Error> {
        for index in 0..(self.list.len()) {
            let item = &(self.list[index]);
            // TODO
            println!("{index}. {} {}", item.title, item.priority);
        }
        Ok(())
    }

    pub fn mark_as_done(self: &mut Self, item_id: i32) -> Result<(), io::Error> {
        if item_id < 0 || item_id >= self.list.len().try_into().unwrap() {
            // TODO
            // return Result::err(Error::new(io::ErrorKind::InvalidInput, "Invalid item id"));
            println!("Invalid index")
        } else {
            self.list[item_id as usize].status = TodoStatus::Done;
        }
        Ok(())
    }

    pub fn remove_item(self: &mut Self, item_id: i32) -> Result<(), io::Error> {
        if item_id < 0 || item_id >= self.list.len().try_into().unwrap() {
            // TODO
            // return Result::err(Error::new(io::ErrorKind::InvalidInput, "Invalid item id"));
            println!("Invalid index")
        } else {
            self.list.remove(item_id as usize);
        }
        Ok(())
    }
}
