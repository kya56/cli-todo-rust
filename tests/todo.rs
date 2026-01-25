use todo_cli::cli::ListMode;
use todo_cli::todo::TodoList;

#[test]
fn add_item() {
    let mut todo = TodoList::new();
    todo.add(String::from("Take a dog out"));
    assert_eq!(todo.items.get("Take a dog out"), Some(&true))
}

#[test]
fn add_item_already_exist() {
    let mut todo = TodoList::new();
    todo.add(String::from("Take a dog out"));
    todo.add(String::from("Take a dog out"));
    assert_eq!(todo.items.get("Take a dog out"), Some(&true));
    assert_eq!(todo.items.len(), 1);
}

#[test]
fn add_item_does_not_change_value() {
    let mut todo = TodoList::new();
    todo.add(String::from("Take a dog out"));

    if let Some(x) = todo.items.get_mut("Take a dog out") {
        *x = false;
    }

    todo.add(String::from("Take a dog out"));
    assert_eq!(todo.items.get("Take a dog out"), Some(&false));
    assert_eq!(todo.items.len(), 1);
}

#[test]
fn mark_item() {
    let mut todo = TodoList::new();
    todo.add(String::from("Take a dog out"));
    let _ = todo.mark("Take a dog out", false);
    assert_eq!(todo.items.get("Take a dog out"), Some(&false));
    let _ = todo.mark("Take a dog out", true);
    assert_eq!(todo.items.get("Take a dog out"), Some(&true));
}

#[test]
fn mark_item_does_not_exist() {
    let mut todo = TodoList::new();
    assert_eq!(
        todo.mark("Not existing", false),
        Err(String::from("Key 'Not existing' is not found."))
    );
}

#[test]
fn list_items_all() {
    let mut todo = TodoList::new();
    todo.add(String::from("First task"));
    todo.add(String::from("Second task"));
    todo.add(String::from("Third task"));
    let _ = todo.mark("Third task", false);

    let items = todo.list(ListMode::All);

    assert!(items.iter().any(|e| e.0 == "First task"));
    assert!(items.iter().any(|e| e.0 == "Second task"));
    assert!(items.iter().any(|e| e.0 == "Third task"));
    assert_eq!(items.len(), 3);
}

#[test]
fn list_items_done() {
    let mut todo = TodoList::new();
    todo.add(String::from("First task"));
    todo.add(String::from("Second task"));
    todo.add(String::from("Third task"));
    let _ = todo.mark("Third task", false);

    let items = todo.list(ListMode::Done);

    assert!(items.iter().any(|e| *e.0 == "Third task"));
    assert_eq!(items.len(), 1);
}

#[test]
fn list_items_todo() {
    let mut todo = TodoList::new();
    todo.add(String::from("First task"));
    todo.add(String::from("Second task"));
    todo.add(String::from("Third task"));
    let _ = todo.mark("Third task", false);

    let items = todo.list(ListMode::Todo);

    assert!(items.iter().any(|e| *e.0 == "First task"));
    assert!(items.iter().any(|e| *e.0 == "Second task"));
    assert_eq!(items.len(), 2);
}

#[test]
fn pending() {
    let mut todo = TodoList::new();
    todo.add(String::from("First task"));
    todo.add(String::from("Second task"));
    todo.add(String::from("Third task"));
    let _ = todo.mark("Third task", false);

    let pending = todo.pending();
    assert!(pending.iter().any(|x| x == "First task"));
    assert!(pending.iter().any(|x| x == "Second task"));
    assert_eq!(pending.len(), 2);
}

#[test]
fn remove_item() {
    let mut todo = TodoList::new();
    todo.add(String::from("First task"));

    assert_eq!(todo.items.get("First task"), Some(&true));

    let _ = todo.remove(&String::from("First task"));
    assert_eq!(todo.items.get("First task"), None);
}
