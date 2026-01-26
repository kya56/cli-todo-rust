use clap::Parser;
use dialoguer::{Confirm, Select};
use todo_cli::cli::{Cli, Command, ListMode};
use todo_cli::file::{load_todos, save_todos};
use todo_cli::todo::Todo;

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let mut todo = load_todos();

    match cli.command {
        Command::Add { key } => {
            todo.add(key);
            save_todos(&todo);
        }
        Command::MarkDone => {
            let items: Vec<&Todo> = todo.todo().collect();

            if items.is_empty() {
                println!("No todos to mark as done");
                return Ok(());
            }

            let labels: Vec<String> = items
                .iter()
                .map(|x| format!("[{}] {}", x.id, x.title))
                .collect();

            let selection = match Select::new()
                .with_prompt("Select todo to mark as done")
                .items(&labels)
                .interact()
            {
                Ok(index) => index,
                Err(_) => {
                    println!("Action cancelled");
                    return Ok(());
                }
            };

            let id = items[selection].id;
            todo.mark(id, true)?;

            save_todos(&todo);
        }
        Command::UndoDone => {
            let items: Vec<&Todo> = todo.done().collect();

            if items.is_empty() {
                println!("No todos to undo done");
                return Ok(());
            }

            let labels: Vec<String> = items
                .iter()
                .map(|x| format!("[{}] {}", x.id, x.title))
                .collect();

            let selection = match Select::new()
                .with_prompt("Select completed todo to undo done")
                .items(&labels)
                .interact()
            {
                Ok(index) => index,
                Err(_) => {
                    println!("Action cancelled");
                    return Ok(());
                }
            };

            let id = items[selection].id;
            todo.mark(id, false)?;
            save_todos(&todo);
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
            }
        }
        Command::Delete => {
            let items = todo.list();

            if items.is_empty() {
                println!("No todos to delete");
                return Ok(());
            }

            let labels: Vec<String> = items
                .iter()
                .map(|x| format!("[{}] {}", x.id, x.title))
                .collect();

            let selection = match Select::new()
                .with_prompt("Select todo to delete")
                .items(&labels)
                .interact()
            {
                Ok(index) => index,
                Err(_) => {
                    println!("Action cancelled");
                    return Ok(());
                }
            };

            let id = items[selection].id;
            let title = items[selection].title.clone();

            let confirm = Confirm::new()
                .with_prompt(format!("Are you sure you want to delete '{}'?", title))
                .default(false)
                .interact()
                .map_err(|e| e.to_string())?;

            if !confirm {
                println!("Delete cancelled");
                return Ok(());
            }

            todo.remove(id)?;
            save_todos(&todo);
            println!("Deleted '[{}] {}'", id, title);
        }
    };

    Ok(())
}
