use clap::{Parser, Subcommand};
use std::io::{self};

// crate is root /src, lib is file/module name, then whatever we are using
use todo::TodoList;

mod csv_io;
use csv_io::{load_list, save_list};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
// TODO What is this derive
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Add { title: String, priority: String },
    Ls,
    Done { item_id: i32 },
    Rm { item_id: i32 },
    Clear,
    Sort { sort_by: String },
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

    let mut todo_list = TodoList {
        list: match load_list() {
            Ok(list) => list,
            Err(e) => match e.kind() {
                io::ErrorKind::Other => {
                    panic!("Todo list csv file has been corrupted.");
                }
                _ => panic!("{e}"),
            },
        },
    };

    match execute(args, &mut todo_list) {
        Ok(()) => {}
        Err(e) => {
            panic!("{e}");
        }
    };

    match save_list(todo_list) {
        Ok(()) => {}
        Err(e) => {
            panic!("{e}");
        }
    };
}
