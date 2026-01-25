use serde::{Deserialize, Serialize};

use crate::cli::ListMode;
use std::collections::{HashMap, hash_map::Entry};

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn mark(&mut self, key: &str, value: bool) -> Result<(), String> {
        match self.items.get_mut(key) {
            Some(x) => {
                *x = value;
                Ok(())
            }
            None => Err(format!("Key '{}' is not found.", key)),
        }
    }

    pub fn list(&self, mode: ListMode) -> HashMap<String, bool> {
        self.items
            .iter()
            .filter(|&(_, &value)| match mode {
                ListMode::All => true,
                ListMode::Done => !value,
                ListMode::Todo => value,
            })
            .map(|(key, value)| (key.clone(), *value))
            .collect()
    }

    pub fn all(&self) -> Vec<String> {
        self.items.iter().map(|(key, _)| key.clone()).collect()
    }

    pub fn pending(&self) -> Vec<String> {
        self.items
            .iter()
            .filter(|x| x.1 == &true)
            .map(|(key, _)| key.clone())
            .collect()
    }

    pub fn done(&self) -> Vec<String> {
        self.items
            .iter()
            .filter(|x| !*x.1)
            .map(|(key, _)| key.clone())
            .collect()
    }

    pub fn remove(&mut self, key: &str) -> Result<(), String> {
        if self.items.remove(key).is_some() {
            Ok(())
        } else {
            Err(format!("Key {} is not found.", key))
        }
    }
}
