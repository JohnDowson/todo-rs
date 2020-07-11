use druid::{Data, Lens};
use nanoserde::{DeJson, SerJson};
#[derive(Clone, Data, Lens, Debug, DeJson, SerJson)]
pub struct TodoItem {
    pub desc: String,
    /* Magic strings instead of enums,
    because nanoserde doesn't support serializing enums yet
    TODO: possibly can be worked around using string as poxy?*/
    pub state: String,
}
impl TodoItem {
    pub fn new(desc: String) -> TodoItem {
        TodoItem {
            desc,
            state: "PENDING".to_owned(),
        }
    }
}

pub fn load_or_new(path: &str) -> Vec<TodoItem> {
    match std::fs::read_to_string(path) {
        Ok(f) => match Vec::<TodoItem>::deserialize_json(&f) {
            Ok(todo) => todo,
            Err(_) => Vec::<TodoItem>::new(),
        },
        Err(_) => Vec::<TodoItem>::new(),
    }
}
pub fn save_todo(todo: &Vec<TodoItem>, path: &str) -> Result<(), String> {
    println!("Saving to {}", path);
    use std::fs;
    let s = Vec::<TodoItem>::serialize_json(todo);
    println!("Saving {}", s);
    match fs::write(path, s) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
