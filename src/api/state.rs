use crate::shared::todo::TodoList;
use std::sync::{Arc, Mutex};

pub type AppState = Arc<Mutex<TodoList>>;
