use todo::shared::todo::{Todo, TodoList};

#[test]
fn add_item() {
    let mut todos = TodoList::new();
    todos.add(String::from("Take a dog out"));
    assert!(exist(&todos.items, "Take a dog out", 1));
    assert_eq!(find(&todos.items, 1).unwrap().done, false);
}

#[test]
fn add_item_with_duplicate_title() {
    let mut todos = TodoList::new();
    todos.add(String::from("Take a dog out"));
    todos.add(String::from("Take a dog out"));
    assert!(exist(&todos.items, "Take a dog out", 1));
    assert!(exist(&todos.items, "Take a dog out", 2));
    assert_eq!(todos.items.len(), 2);
}

#[test]
fn mark_item() {
    let mut todo = TodoList::new();
    todo.add(String::from("Take a dog out"));
    let _ = todo.mark(1, true);
    assert_eq!(find(&todo.items, 1).unwrap().done, true);
    let _ = todo.mark(1, false);
    assert_eq!(find(&todo.items, 1).unwrap().done, false);
}

#[test]
fn mark_item_does_not_exist() {
    let mut todos = TodoList::new();
    assert_eq!(
        todos.mark(1, false),
        Err(String::from("Todo '1' is not found."))
    );
}

#[test]
fn list_items_all() {
    let mut todos = TodoList::new();
    todos.add(String::from("First task"));
    todos.add(String::from("Second task"));
    todos.add(String::from("Third task"));
    let _ = todos.mark(3, true);

    let items = todos.list();

    assert!(exist(items, "First task", 1));
    assert!(exist(items, "Second task", 2));
    assert!(exist(items, "Third task", 3));
    assert_eq!(items.len(), 3);
}

#[test]
fn list_items_done() {
    let mut todo = TodoList::new();
    todo.add(String::from("First task"));
    todo.add(String::from("Second task"));
    todo.add(String::from("Third task"));
    let _ = todo.mark(3, true);

    let items: Vec<&Todo> = todo.done().collect();

    assert!(exist(items.iter().copied(), "Third task", 3));
    assert_eq!(items.len(), 1);
}

#[test]
fn list_items_todo() {
    let mut todo = TodoList::new();
    todo.add(String::from("First task"));
    todo.add(String::from("Second task"));
    todo.add(String::from("Third task"));
    let _ = todo.mark(3, true);

    let items: Vec<&Todo> = todo.todo().collect();

    assert!(exist(items.iter().copied(), "First task", 1));
    assert!(exist(items.iter().copied(), "Second task", 2));
    assert_eq!(items.len(), 2);
}

#[test]
fn update_title() {
    let mut todo = TodoList::new();
    todo.add(String::from("First task"));
    let _ = todo.update_title(1, "Updated task");

    assert!(exist(&todo.items, "Updated task", 1));
    assert_eq!(exist(&todo.items, "First task", 1), false);
}

#[test]
fn update_title_not_existing() {
    let mut todo = TodoList::new();
    todo.add(String::from("First task"));
    assert_eq!(
        todo.update_title(2, "Updated task"),
        Err(String::from("Todo 2 not found"))
    );
}

#[test]
fn remove_item() {
    let mut todo = TodoList::new();
    todo.add(String::from("First task"));

    assert!(exist(&todo.items, "First task", 1));

    let _ = todo.remove(1);
    assert_eq!(exist(&todo.items, "First task", 1), false);
}

fn exist<'a, I>(todos: I, title: &str, id: u64) -> bool
where
    I: IntoIterator<Item = &'a Todo>,
{
    todos.into_iter().any(|t| t.title == title && t.id == id)
}

fn find<'a, I>(todos: I, id: u64) -> Option<&'a Todo>
where
    I: IntoIterator<Item = &'a Todo>,
{
    todos.into_iter().find(|t| t.id == id)
}
