use clap::{Parser, Subcommand};
use std::fs::OpenOptions;
use std::io::{self, Read, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Add { task_title: String },
    Rm { task_id: u32 },
    Ls,
}

fn add_task(task_title: &String) -> Result<(), io::Error> {
    // taking reference (immutable) to task_title to just access it

    // using ? only when function returns result
    // ? basically returns error in case of Err arm, or does Ok(..) and returns value
    // since these are file operations, error is of type io.Error
    // we have nothing to return, so Ok(()) returns a Result object with no value
    // if it was Result<bool, ...>, then Ok(true) would work

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("tasks.txt")?;

    let task_item = format!("{task_title}\n");
    file.write_all(task_item.as_bytes())?;

    println!("{file:?}");
    return Ok(());
}

fn load_tasks() -> Result<Vec<String>, io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("tasks.txt")?;
    // for create to work, write or append must be enabled

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // since size of str not known at compile time, we need to make vector of &str

    // why String instead of &str?
    let mut list: Vec<String> = vec![];
    for task in contents.lines() {
        list.push(task.to_string());
    }

    return Ok(list);
}

fn list_tasks() -> Result<(), io::Error> {
    // why String instead of &str?
    let list: Vec<String> = load_tasks()?;

    for index in 0..(list.len()) {
        println!("{index}. {}", list[index]);
    }

    return Ok(());
}

fn remove_task(task_id: u32) {}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Add { task_title } => match add_task(&task_title) {
            Err(e) => {
                println!("Error in adding task, please try again");
                println!("{e}");
            }
            Ok(()) => {
                println!("Task added to file");
            }
        },
        Commands::Ls {} => match list_tasks() {
            Err(e) => {
                println!("Error in listing task, please try again");
                println!("{e}");
            }
            Ok(()) => {}
        },
        Commands::Rm { task_id } => {
            remove_task(task_id);
            println!("Task {task_id} removed");
        }
    }
}
