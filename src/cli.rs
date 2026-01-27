use crate::todo::{Todo, TodoList};
use clap::{Parser, Subcommand, ValueEnum};
use dialoguer::{Confirm, Input, Select};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo list cli", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ListMode {
    All,
    Done,
    Todo,
}

#[derive(Subcommand)]
pub enum Command {
    Add {
        key: String,
    },
    MarkDone,
    UndoDone,
    List {
        #[arg(
            long,
            value_enum,
            default_value_t = ListMode::All,
        )]
        mode: ListMode,
    },
    Delete,
    Update,
}

pub enum RunResult {
    NoChange,
    Changed,
}

pub fn run(command: Command, todo: &mut TodoList) -> Result<RunResult, String> {
    match command {
        Command::Add { key } => {
            todo.add(key);
            Ok(RunResult::Changed)
        }
        Command::MarkDone => {
            let items: Vec<&Todo> = todo.todo().collect();

            if items.is_empty() {
                println!("No todos to mark as done");
                return Ok(RunResult::NoChange);
            }

            let Some(selection) = prompt_select(&items, "Select a todo to mark as done")? else {
                println!("Action cancelled");
                return Ok(RunResult::NoChange);
            };

            let id = items[selection].id;
            todo.mark(id, true)?;

            Ok(RunResult::Changed)
        }
        Command::UndoDone => {
            let items: Vec<&Todo> = todo.done().collect();

            if items.is_empty() {
                println!("No todos to undo done");
                return Ok(RunResult::NoChange);
            }

            let Some(selection) = prompt_select(&items, "Select completed todo to undo done")?
            else {
                println!("Action cancelled");
                return Ok(RunResult::NoChange);
            };

            let id = items[selection].id;
            todo.mark(id, false)?;
            Ok(RunResult::Changed)
        }
        Command::List { mode } => {
            let items = todo.list();

            match mode {
                ListMode::All => {
                    println!("# TODO");
                    items
                        .iter()
                        .filter(|x| x.done == false)
                        .for_each(|x| println!("[{}] {}", x.id, x.title));
                    println!();

                    println!("# DONE");
                    items
                        .iter()
                        .filter(|x| x.done == true)
                        .for_each(|x| println!("[{}] {}", x.id, x.title));
                }
                ListMode::Done => {
                    println!("# DONE");
                    items
                        .iter()
                        .for_each(|x| println!("[{}] {}", x.id, x.title));
                }
                ListMode::Todo => {
                    println!("# TODO");
                    items
                        .iter()
                        .for_each(|x| println!("[{}] {}", x.id, x.title));
                }
            };

            Ok(RunResult::NoChange)
        }
        Command::Update => {
            let items = todo.list();

            if items.is_empty() {
                println!("No todos to update");
                return Ok(RunResult::NoChange);
            }

            let Some(selection) = prompt_select(&items, "Select toddo to update")? else {
                println!("Action cancelled");
                return Ok(RunResult::NoChange);
            };

            let id = items[selection].id;
            let title = items[selection].title.clone();
            let new_title = match Input::<String>::new()
                .with_prompt("Edit title")
                .with_initial_text(&title)
                .interact_text()
            {
                Ok(title) => title,
                Err(_) => {
                    println!("Action cancelled");
                    return Ok(RunResult::NoChange);
                }
            };

            if new_title.trim() == title {
                println!("Title unchanged");
                return Ok(RunResult::NoChange);
            }

            todo.update_title(id, &new_title)?;

            println!("Todo '[{}] {}' updated to {}", id, title, new_title);
            Ok(RunResult::Changed)
        }
        Command::Delete => {
            let items = todo.list();

            if items.is_empty() {
                println!("No todos to delete");
                return Ok(RunResult::NoChange);
            }

            let Some(selection) = prompt_select(&items, "Select todo to delete")? else {
                println!("Action cancelled");
                return Ok(RunResult::NoChange);
            };

            let id = items[selection].id;
            let title = items[selection].title.clone();

            let confirm =
                prompt_confirm(format!("Are you sure you want to delete '{}'?", title).as_str())?;

            if !confirm {
                println!("Delete cancelled");
                return Ok(RunResult::NoChange);
            }

            todo.remove(id)?;
            println!("Deleted '[{}] {}'", id, title);
            Ok(RunResult::Changed)
        }
    }
}

fn prompt_select<T: std::fmt::Display>(
    items: &[T],
    prompt_title: &str,
) -> Result<Option<usize>, String> {
    Select::new()
        .with_prompt(prompt_title)
        .items(items)
        .interact()
        .map(Some)
        .map_err(|e| e.to_string())
}

fn prompt_confirm(prompt_title: &str) -> Result<bool, String> {
    Confirm::new()
        .with_prompt(prompt_title)
        .default(false)
        .interact()
        .map_err(|e| e.to_string())
}
