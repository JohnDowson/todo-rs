use druid::{Data, Lens};
use serde::{Deserialize, Serialize};
#[derive(Clone, Data, Serialize, Deserialize, Lens, Debug)]
pub struct TodoItem {
    pub desc: String,
    pub state: TodoState,
    pub to_be_removed: bool,
}
impl TodoItem {
    pub fn new(desc: String) -> TodoItem {
        TodoItem {
            desc,
            state: TodoState::Pending,
            to_be_removed: false,
        }
    }
}

#[derive(Clone, Data, PartialEq, Serialize, Deserialize, Debug)]
pub enum TodoState {
    Pending,
    Started,
    Done,
}

pub fn load_or_new() -> Vec<TodoItem> {
    match std::fs::File::open("./todo.json") {
        Ok(f) => match serde_json::from_reader(f) {
            Ok(todo) => todo,
            Err(_) => Vec::<TodoItem>::new(),
        },
        Err(_) => Vec::<TodoItem>::new(),
    }
}
pub fn save_todo(todo: &Vec<TodoItem>, path: String) -> Result<(), String> {
    println!("Saving to {}", path);
    use std::fs;
    match serde_json::to_string(todo) {
        Ok(s) => {
            println!("Saving {}", s);
            match fs::write(path, s) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
