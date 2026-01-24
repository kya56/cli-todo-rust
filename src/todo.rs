use crate::cli::ListMode;
use std::collections::{HashMap, hash_map::Entry};

pub struct TodoList {
    // true = todo, false = done
    pub items: HashMap<String, bool>,
}

impl TodoList {
    pub fn new() -> TodoList {
        let items = HashMap::<String, bool>::new();
        TodoList { items: items }
    }

    pub fn add(&mut self, key: String) {
        if let Entry::Vacant(entry) = self.items.entry(key) {
            entry.insert(true);
        }
    }

    pub fn mark(&mut self, key: String, value: bool) -> Result<String, String> {
        let x = self.items.get_mut(&key).ok_or(&key)?;
        *x = value;

        Ok(key)
    }

    pub fn list(&self, mode: ListMode) -> HashMap<String, bool> {
        self.items
            .iter()
            .filter(|&(_, &value)| match mode {
                ListMode::All => true,
                ListMode::DoneOnly => !value,
                ListMode::Todo => value,
            })
            .map(|(key, value)| (key.clone(), *value))
            .collect()
    }

    pub fn remove(&mut self, key: String) {
        if let Entry::Occupied(entry) = self.items.entry(key) {
            entry.remove_entry();
        }
    }
}
