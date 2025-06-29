// cargo new todo created a Cargo.toml file which has name="todo"
// Cargo.toml file means this is a package named todo. (package is collection of 1 or more crates)

// convention that src/main.rs is the crate root of a binary crate with the same name as the package
// convention that src/lib.rs, is the crate root of a library crate with the same name as the package

// to use external crates
// use <crate_name>::<module>::<item>;
use clap::{Parser, Subcommand};
use std::{
    io::{self},
    process,
};

// we do not have crate here since this module is in the same crate
// declare the module and use it
mod csv_io;
use csv_io::{get_todo_file_path, load_list, save_list};

// to use public components of library crate
// directly use package name followed by what you want to use
// works because package can have at most 1 library crate
use todo::TodoList;

// if we have something in main that we wanted to use elsewhere
// pub struct Trial(i32);
// we would import is using
// use crate::Trial;
// crate points to root src/

#[derive(Parser)]
// generates code that can parse command-line arguments into Args struct
#[command(
    version,
    about = "Todo list to add items and mark them as done",
    long_about = "A simple CLI todo list manager. Use subcommands to add, list, mark as done, remove, clear, or sort your tasks."
)]
struct Args {
    // this field will hold subcommands
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
// generates code for parsing subcommands (like add, ls, done, etc.) into enum.
enum Commands {
    #[command(about = "Add a new todo item")]
    Add {
        #[arg(help = "Title of the todo item")]
        title: String,
        #[arg(help = "Priority (high, med or medium, low)")]
        priority: String,
    },
    #[command(about = "List all todo items")]
    Ls,
    #[command(about = "Mark a todo item as done")]
    Done {
        #[arg(help = "ID of the item to mark as done")]
        item_id: i32,
    },
    #[command(about = "Remove a todo item")]
    Rm {
        #[arg(help = "ID of the item to remove")]
        item_id: i32,
    },
    #[command(about = "Clear all done items")]
    Clear,
    #[command(about = "Sort items by status or priority")]
    Sort {
        #[arg(help = "Sort by 'status' or 'priority'")]
        sort_by: String,
    },
}

fn execute(args: Args, todo_list: &mut TodoList) -> Result<(), io::Error> {
    match args.cmd {
        Commands::Add { title, priority } => {
            todo_list.add_item(title, priority)?;
        }
        Commands::Ls {} => {
            todo_list.list_items()?;
        }
        Commands::Rm { item_id } => {
            todo_list.remove_item(item_id)?;
        }
        Commands::Done { item_id } => {
            todo_list.mark_as_done(item_id)?;
        }
        Commands::Clear => {
            todo_list.clear_done_items()?;
        }
        Commands::Sort { sort_by } => {
            todo_list.sort(sort_by)?;
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();

    let todo_file_path = match get_todo_file_path() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    let mut todo_list = TodoList {
        list: match load_list(&todo_file_path) {
            Ok(list) => list,
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::Other => {
                        eprintln!("Todo list csv file has been corrupted.");
                    }
                    _ => eprintln!("{e}"),
                };
                process::exit(1);
            }
        },
    };

    match execute(args, &mut todo_list) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    match save_list(&todo_file_path, &todo_list) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
}
