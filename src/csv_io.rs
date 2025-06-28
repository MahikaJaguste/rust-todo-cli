use csv;
use dirs_next::data_local_dir;
use std::fs::{self, OpenOptions};
use std::io::Error;
use std::path::PathBuf;
use todo::{TodoItem, TodoList};

fn get_todo_file_path() -> Result<PathBuf, Error> {
    let mut dir = match data_local_dir() {
        Some(dir) => dir,
        None => {
            return Err(Error::new(
                std::io::ErrorKind::NotADirectory,
                "Could not find a local data directory",
            ));
        }
    };

    dir.push("todo_cli");
    match fs::create_dir_all(&dir) {
        Ok(d) => d,
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::NotADirectory,
                "Could not create directory todo_cli",
            ));
        }
    }

    dir.push("todo_list.csv");

    Ok(dir)
}

pub fn load_list() -> Result<Vec<TodoItem>, Error> {
    let todo_file_path = get_todo_file_path()?;

    // using ? only when function returns result
    // ? basically returns error in case of Err arm, or does Ok(..) and returns value
    // since these are file operations, error is of type io.Error
    // we have nothing to return, so Ok(()) returns a Result object with no value
    // if it was Result<bool, ...>, then Ok(true) would work
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(todo_file_path)?;
    // for create to work, write or append must be enabled

    let mut rdr = csv::Reader::from_reader(file);

    let mut list: Vec<TodoItem> = vec![];

    // in case of error, ? calls From(err) to convert csv::Error to io:Error of variant Other
    for result in rdr.deserialize() {
        let item: TodoItem = result?;
        list.push(item);
    }

    Ok(list)

    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;

    // if list of String, why String instead of &str?
    // since size of str not known at compile time, we need to make vector of &str
}

pub fn save_list(todo_list: TodoList) -> Result<(), Error> {
    let todo_file_path = get_todo_file_path()?;

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(todo_file_path)?;
    // truncate is needed, as if current content is shorter than existing,
    // the leftover characters of original file will not be removed

    let mut wtr = csv::Writer::from_writer(file);

    for item in todo_list.list {
        // serialise, each row is written/buffered in memory
        wtr.serialize(item)?;
    }

    // ensures all data (buffered in memory) is persistend onto disk ie. saved in file
    wtr.flush()?;
    Ok(())
}
