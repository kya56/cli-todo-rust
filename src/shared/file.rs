use super::todo::TodoList;
use std::fs;
use std::path::Path;

const TODO_FILE: &str = "resource/todo.json";

pub fn load_todos() -> TodoList {
    if !Path::new(TODO_FILE).exists() {
        return TodoList::new();
    }

    let data = fs::read_to_string(TODO_FILE).expect("Failed to read todo.json");

    serde_json::from_str(&data).unwrap_or_else(|_| TodoList::new())
}

pub fn save_todos(todo: &TodoList) {
    let data = serde_json::to_string_pretty(todo).expect("Failed to serialize todos");

    fs::write(TODO_FILE, data).expect("Failed to write todos to file")
}
