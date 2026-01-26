use serde::{Deserialize, Serialize};

use crate::cli::ListMode;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    // true = todo, false = done
    pub items: Vec<Todo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub done: bool,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    pub fn add(&mut self, title: String) {
        self.items.push(Todo { title, done: false });
    }

    pub fn mark(&mut self, title: &str, value: bool) -> Result<(), String> {
        let todo = self
            .items
            .iter_mut()
            .find(|x| x.title == title)
            .ok_or_else(|| format!("Key '{}' is not found.", title))?;
        todo.done = value;
        Ok(())
    }

    pub fn list(&self, mode: ListMode) -> Vec<&Todo> {
        self.items
            .iter()
            .filter(|x| match mode {
                ListMode::All => true,
                ListMode::Done => x.done,
                ListMode::Todo => !x.done,
            })
            .collect()
    }

    pub fn remove(&mut self, title: &str) -> Result<(), String> {
        let index = self
            .items
            .iter()
            .position(|x| x.title == title)
            .ok_or_else(|| format!("Key '{}' is not found", title))?;

        self.items.remove(index);
        Ok(())
    }
}
