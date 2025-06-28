use fancy::printcoln;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    title: String,
    priority: TodoPriority,
    status: TodoStatus,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
enum TodoStatus {
    Pending,
    Done,
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
pub enum TodoPriority {
    #[strum(ascii_case_insensitive)]
    High,
    #[strum(ascii_case_insensitive, serialize = "med")]
    Medium,
    #[strum(ascii_case_insensitive)]
    Low,
}

impl fmt::Display for TodoPriority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TodoPriority::High => write!(f, "‼️"),
            TodoPriority::Medium => write!(f, "❕"), // ❗
            TodoPriority::Low => write!(f, ""),
        }
    }
}

pub struct TodoList {
    pub list: Vec<TodoItem>,
}

impl TodoList {
    pub fn add_item(self: &mut Self, title: String, priority: String) -> Result<(), Error> {
        // taking reference (immutable) to title to just access it

        // TODO - understand how the from_str enumstr works
        let priority: TodoPriority = match TodoPriority::from_str(&priority) {
            Ok(p) => p,
            Err(_e) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Priority can be high, medium, or low",
                ));
            }
        };

        let item = TodoItem {
            title,
            priority,
            status: TodoStatus::Pending,
        };

        self.list.push(item);

        Ok(())
    }

    pub fn list_items(self: &Self) -> Result<(), Error> {
        for index in 0..(self.list.len()) {
            let item = &(self.list[index]);
            if item.status == TodoStatus::Done {
                printcoln!(
                    "[strikethrough]{}. {} {}",
                    index + 1,
                    item.title,
                    item.priority
                );
            } else {
                printcoln!("{}. {} {}", index + 1, item.title, item.priority);
            }
        }
        Ok(())
    }

    pub fn mark_as_done(self: &mut Self, item_id: i32) -> Result<(), Error> {
        if item_id <= 0 || item_id > self.list.len().try_into().unwrap() {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid item id"));
        } else if self.list[(item_id - 1) as usize].status == TodoStatus::Done {
            return Err(Error::new(
                ErrorKind::Other,
                "Item is already marked as done",
            ));
        } else {
            self.list[(item_id - 1) as usize].status = TodoStatus::Done;
        }
        Ok(())
    }

    pub fn remove_item(self: &mut Self, item_id: i32) -> Result<(), Error> {
        if item_id <= 0 || item_id > self.list.len().try_into().unwrap() {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid item id"));
        } else {
            self.list.remove((item_id - 1) as usize);
        }
        Ok(())
    }
}
