use ansi_term::Style;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TodoItem {
    pub title: String,
    pub priority: TodoPriority,
    pub status: TodoStatus,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TodoStatus {
    Pending,
    Done,
}

#[derive(Debug, Serialize, Deserialize, EnumString, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum TodoPriority {
    High,
    #[strum(serialize = "med", serialize = "medium")]
    Medium,
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

#[derive(Debug, PartialEq)]
pub struct TodoList {
    pub list: Vec<TodoItem>,
}

impl TodoItem {
    pub fn print_item(self: &Self, index: usize) {
        let item_line = format!("{}. {} {}", index + 1, self.title, self.priority);
        if self.status == TodoStatus::Done {
            println!("{}", Style::new().dimmed().strikethrough().paint(item_line));
        } else {
            println!("{item_line}");
        }
    }
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
        println!("Added item");
        self.list_items()?;

        Ok(())
    }

    pub fn list_items(self: &Self) -> Result<(), Error> {
        for index in 0..(self.list.len()) {
            let item = &(self.list[index]);
            item.print_item(index);
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
            println!("Marked item {} as done", item_id);
            self.list_items()?;
        }
        Ok(())
    }

    pub fn remove_item(self: &mut Self, item_id: i32) -> Result<(), Error> {
        if item_id <= 0 || item_id > self.list.len().try_into().unwrap() {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid item id"));
        } else {
            self.list.remove((item_id - 1) as usize);
            println!("Removed item {}", item_id);
            self.list_items()?;
        }
        Ok(())
    }

    pub fn clear_done_items(self: &mut Self) -> Result<(), Error> {
        // to filter in place, retain is used
        self.list.retain(|item| item.status != TodoStatus::Done);
        println!("Cleared done items");
        self.list_items()?;
        Ok(())
    }

    pub fn sort(self: &mut Self, sort_by: String) -> Result<(), Error> {
        match sort_by.to_lowercase().as_str() {
            "status" => self.sort_by_status()?,
            "priority" => self.sort_by_priority()?,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Sort_by key can be status or priority",
                ));
            }
        }
        self.list_items()?;
        Ok(())
    }

    fn sort_by_status(self: &mut Self) -> Result<(), Error> {
        // sort by key is stable
        self.list.sort_by_key(|item| match item.status {
            TodoStatus::Pending => 0,
            TodoStatus::Done => 1,
        });
        Ok(())
    }

    fn sort_by_priority(self: &mut Self) -> Result<(), Error> {
        self.list.sort_by_key(|item| match item.priority {
            TodoPriority::High => 0,
            TodoPriority::Medium => 1,
            TodoPriority::Low => 2,
        });
        Ok(())
    }
}
