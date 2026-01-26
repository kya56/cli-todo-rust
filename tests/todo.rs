use todo_cli::cli::ListMode;
use todo_cli::todo::{Todo, TodoList};

#[test]
fn add_item() {
    let mut todos = TodoList::new();
    todos.add(String::from("Take a dog out"));
    assert!(exist(&todos.items, "Take a dog out"));
    assert_eq!(find(&todos.items, "Take a dog out").unwrap().done, false);
}

#[test]
fn add_item_with_duplicate_title() {
    let mut todos = TodoList::new();
    todos.add(String::from("Take a dog out"));
    todos.add(String::from("Take a dog out"));
    assert!(exist(&todos.items, "Take a dog out"));
    assert_eq!(todos.items.len(), 2);
}

#[test]
fn mark_item() {
    let mut todo = TodoList::new();
    todo.add(String::from("Take a dog out"));
    let _ = todo.mark("Take a dog out", true);
    assert_eq!(find(&todo.items, "Take a dog out").unwrap().done, true);
    let _ = todo.mark("Take a dog out", false);
    assert_eq!(find(&todo.items, "Take a dog out").unwrap().done, false);
}

#[test]
fn mark_item_does_not_exist() {
    let mut todos = TodoList::new();
    assert_eq!(
        todos.mark("Not existing", false),
        Err(String::from("Key 'Not existing' is not found."))
    );
}

#[test]
fn list_items_all() {
    let mut todos = TodoList::new();
    todos.add(String::from("First task"));
    todos.add(String::from("Second task"));
    todos.add(String::from("Third task"));
    let _ = todos.mark("Third task", true);

    let items = todos.list(ListMode::All);

    assert!(exist(items.iter().copied(), "First task"));
    assert!(exist(items.iter().copied(), "Second task"));
    assert!(exist(items.iter().copied(), "Third task"));
    assert_eq!(items.len(), 3);
}

#[test]
fn list_items_done() {
    let mut todo = TodoList::new();
    todo.add(String::from("First task"));
    todo.add(String::from("Second task"));
    todo.add(String::from("Third task"));
    let _ = todo.mark("Third task", true);

    let items = todo.list(ListMode::Done);

    assert!(exist(items.iter().copied(), "Third task"));
    assert_eq!(items.len(), 1);
}

#[test]
fn list_items_todo() {
    let mut todo = TodoList::new();
    todo.add(String::from("First task"));
    todo.add(String::from("Second task"));
    todo.add(String::from("Third task"));
    let _ = todo.mark("Third task", true);

    let items = todo.list(ListMode::Todo);

    assert!(exist(items.iter().copied(), "First task"));
    assert!(exist(items.iter().copied(), "Second task"));
    assert_eq!(items.len(), 2);
}

#[test]
fn remove_item() {
    let mut todo = TodoList::new();
    todo.add(String::from("First task"));

    assert!(exist(&todo.items, "First task"));

    let _ = todo.remove(&String::from("First task"));
    assert_eq!(exist(&todo.items, "First task"), false);
}

fn exist<'a, I>(todos: I, title: &str) -> bool
where
    I: IntoIterator<Item = &'a Todo>,
{
    todos.into_iter().any(|t| t.title == title)
}

fn find<'a, I>(todos: I, title: &str) -> Option<&'a Todo>
where
    I: IntoIterator<Item = &'a Todo>,
{
    todos.into_iter().find(|t| t.title == title)
}
