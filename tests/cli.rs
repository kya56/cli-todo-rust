use todo::cli::{Command, ListMode, RunResult, run};
use todo::prompter::Prompter;
use todo::shared::todo::TodoList;

struct FakePrompter {
    selection: Option<usize>,
    input: Option<String>,
    confirm: bool,
}

impl FakePrompter {
    pub fn nothing_selected() -> FakePrompter {
        FakePrompter {
            selection: None,
            input: None,
            confirm: false,
        }
    }

    pub fn select_first_and_confirm() -> FakePrompter {
        FakePrompter {
            selection: Some(0),
            input: None,
            confirm: true,
        }
    }

    pub fn select_first_and_not_confirm() -> FakePrompter {
        FakePrompter {
            selection: Some(0),
            input: None,
            confirm: false,
        }
    }

    pub fn select_first_and_give_input() -> FakePrompter {
        FakePrompter {
            selection: Some(0),
            input: Some(String::from("Changed")),
            confirm: false,
        }
    }

    pub fn select_first_and_no_input() -> FakePrompter {
        FakePrompter {
            selection: Some(0),
            input: None,
            confirm: false,
        }
    }
}

impl Prompter for FakePrompter {
    fn select(&self, _: &[String], _: &str) -> Result<Option<usize>, String> {
        Ok(self.selection)
    }

    fn input(&self, _: &str, _: &str) -> Result<Option<String>, String> {
        Ok(self.input.clone())
    }

    fn confirm(&self, _: &str) -> Result<bool, String> {
        Ok(self.confirm)
    }
}

#[test]
fn add() {
    let mut todos = TodoList::new();

    let result = run(
        Command::Add {
            key: String::from("First task"),
        },
        &mut todos,
        &FakePrompter::select_first_and_confirm(),
    )
    .unwrap();

    assert_changed(&result);
    assert_eq!(todos.items.len(), 1);
}

#[test]
fn mark_done() {
    let mut todos = TodoList::new();
    todos.add("Test".into());

    let prompter = FakePrompter::select_first_and_confirm();
    let result = run(Command::MarkDone, &mut todos, &prompter).unwrap();

    assert_changed(&result);
    assert!(todos.items.first().unwrap().done);
}

#[test]
fn mark_done_not_selected() {
    let mut todos = TodoList::new();
    todos.add("Test".into());

    let prompter = FakePrompter::nothing_selected();
    let result = run(Command::MarkDone, &mut todos, &prompter).unwrap();

    assert_no_change(&result);
    assert_eq!(todos.items.first().unwrap().done, false);
}

#[test]
fn mark_done_no_todo() {
    let mut todos = TodoList::new();

    let prompter = FakePrompter::select_first_and_confirm();
    let result = run(Command::MarkDone, &mut todos, &prompter).unwrap();

    assert_no_change(&result);
}

#[test]
fn undo_done() {
    let mut todos = TodoList::new();
    todos.add("Test".into());
    let _ = todos.mark(1, true);

    let prompter = FakePrompter::select_first_and_confirm();
    let result = run(Command::UndoDone, &mut todos, &prompter).unwrap();

    assert_changed(&result);
    assert_eq!(todos.items.first().unwrap().done, false);
}

#[test]
fn undo_done_not_selected() {
    let mut todos = TodoList::new();
    todos.add("Test".into());
    let _ = todos.mark(1, true);

    let prompter = FakePrompter::nothing_selected();
    let result = run(Command::UndoDone, &mut todos, &prompter).unwrap();

    assert_no_change(&result);
    assert!(todos.items.first().unwrap().done);
}

#[test]
fn undo_done_no_todo() {
    let mut todos = TodoList::new();
    todos.add("Test".into());

    let prompter = FakePrompter::select_first_and_confirm();
    let result = run(Command::UndoDone, &mut todos, &prompter).unwrap();

    assert_no_change(&result);
}

#[test]
fn list() {
    let mut todos = TodoList::new();
    todos.add("Test1".into());
    todos.add("Test2".into());
    todos.add("Test3".into());
    let _ = todos.mark(1, true);

    let prompter = FakePrompter::nothing_selected();
    assert_no_change(
        &run(
            Command::List {
                mode: ListMode::All,
            },
            &mut todos,
            &prompter,
        )
        .unwrap(),
    );

    assert_no_change(
        &run(
            Command::List {
                mode: ListMode::Todo,
            },
            &mut todos,
            &prompter,
        )
        .unwrap(),
    );

    assert_no_change(
        &run(
            Command::List {
                mode: ListMode::Done,
            },
            &mut todos,
            &prompter,
        )
        .unwrap(),
    );
}

#[test]
fn update() {
    let mut todos = TodoList::new();
    todos.add("Test".into());

    let prompter = FakePrompter::select_first_and_give_input();
    let result = run(Command::Update, &mut todos, &prompter).unwrap();

    assert_changed(&result);
    assert_eq!(todos.items.first().unwrap().title, "Changed");
}

#[test]
fn update_no_input() {
    let mut todos = TodoList::new();
    todos.add("Test".into());

    let prompter = FakePrompter::select_first_and_no_input();
    let result = run(Command::Update, &mut todos, &prompter).unwrap();

    assert_no_change(&result);
    assert_eq!(todos.items.first().unwrap().title, "Test");
}

#[test]
fn update_not_selected() {
    let mut todos = TodoList::new();
    todos.add("Test".into());

    let prompter = FakePrompter::nothing_selected();
    let result = run(Command::Update, &mut todos, &prompter).unwrap();

    assert_no_change(&result);
    assert_eq!(todos.items.first().unwrap().title, "Test");
}

#[test]
fn update_no_todo() {
    let mut todos = TodoList::new();

    let prompter = FakePrompter::select_first_and_give_input();
    let result = run(Command::Update, &mut todos, &prompter).unwrap();

    assert_no_change(&result);
}

#[test]
fn delete_confirmed_removes_item() {
    let mut todos = TodoList::new();
    todos.add("Test".into());

    let prompter = FakePrompter::select_first_and_confirm();
    let result = run(Command::Delete, &mut todos, &prompter).unwrap();

    assert_changed(&result);
    assert!(todos.items.is_empty());
}

#[test]
fn delete_not_confirmed_dont_removes_item() {
    let mut todos = TodoList::new();
    todos.add("Test".into());

    let prompter = FakePrompter::select_first_and_not_confirm();
    let result = run(Command::Delete, &mut todos, &prompter).unwrap();

    assert_no_change(&result);
    assert_eq!(todos.items.len(), 1);
}

#[test]
fn delete_not_selected() {
    let mut todos = TodoList::new();
    todos.add("Test".into());

    let prompter = FakePrompter::nothing_selected();
    let result = run(Command::Delete, &mut todos, &prompter).unwrap();

    assert_no_change(&result);
    assert_eq!(todos.items.len(), 1);
}

#[test]
fn delete_no_todo() {
    let mut todos = TodoList::new();

    let prompter = FakePrompter::select_first_and_confirm();
    let result = run(Command::Delete, &mut todos, &prompter).unwrap();

    assert_no_change(&result);
}

fn assert_changed(result: &RunResult) {
    assert_eq!(*result, RunResult::Changed);
}

fn assert_no_change(result: &RunResult) {
    assert_eq!(*result, RunResult::NoChange);
}
