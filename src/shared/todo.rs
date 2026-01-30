use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    // true = todo, false = done
    pub items: Vec<Todo>,
    next_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            items: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, title: String) -> Todo {
        let todo = Todo {
            id: self.next_id,
            title,
            done: false,
        };
        self.next_id += 1;
        self.items.push(todo.clone());

        todo
    }

    pub fn mark(&mut self, id: u64, value: bool) -> Result<(), String> {
        let todo = self
            .items
            .iter_mut()
            .find(|x| x.id == id)
            .ok_or_else(|| format!("Todo '{}' is not found.", id))?;
        todo.done = value;
        Ok(())
    }

    pub fn todo(&self) -> impl Iterator<Item = &Todo> {
        self.items.iter().filter(|x| !x.done)
    }

    pub fn done(&self) -> impl Iterator<Item = &Todo> {
        self.items.iter().filter(|x| x.done)
    }

    pub fn list(&self) -> &[Todo] {
        &self.items
    }

    pub fn update_title(&mut self, id: u64, title: &str) -> Result<(), String> {
        let todo = self
            .items
            .iter_mut()
            .find(|x| x.id == id)
            .ok_or_else(|| format!("Todo {} not found", id))?;

        todo.title = title.to_string();
        Ok(())
    }

    pub fn remove(&mut self, id: u64) -> Result<(), String> {
        let index = self
            .items
            .iter()
            .position(|x| x.id == id)
            .ok_or_else(|| format!("Todo '{}' is not found", id))?;

        self.items.remove(index);
        Ok(())
    }
}

impl Todo {
    pub fn fmt(&self) -> String {
        format!("[{}] {}", self.id, self.title)
    }
}
