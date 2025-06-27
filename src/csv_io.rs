use csv;
use std::error::Error;
use std::fs::OpenOptions;
use todo::{TodoItem, TodoList};

// TODO:: what is box dyn error
pub fn load_list() -> Result<Vec<TodoItem>, Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("tasks.csv")?;
    // for create to work, write or append must be enabled

    let mut rdr = csv::Reader::from_reader(file);

    let mut list: Vec<TodoItem> = vec![];

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

pub fn save_list(todo_list: TodoList) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("tasks.csv")?;
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
