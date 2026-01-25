use clap::Parser;
use dialoguer::{Confirm, Select};
use todo_cli::cli::{Cli, Command, ListMode};
use todo_cli::file::{load_todos, save_todos};

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let mut todo = load_todos();

    match cli.command {
        Command::Add { key } => {
            todo.add(key);
            save_todos(&todo);
        }
        Command::MarkDone => {
            let items = todo.pending();

            if items.is_empty() {
                println!("No todos to mark as done");
                return Ok(());
            }

            let selection = match Select::new()
                .with_prompt("Select todo to mark as done")
                .items(&items)
                .default(0)
                .interact()
            {
                Ok(index) => index,
                Err(_) => {
                    println!("Action cancelled");
                    return Ok(());
                }
            };

            let key = items[selection].as_str();
            todo.mark(key, false)?;

            save_todos(&todo);
        }
        Command::UndoDone => {
            let items = todo.done();

            if items.is_empty() {
                println!("No todos to undo done");
                return Ok(());
            }

            let selection = match Select::new()
                .with_prompt("Select completed todo to undo done")
                .items(&items)
                .default(0)
                .interact()
            {
                Ok(index) => index,
                Err(_) => {
                    println!("Action cancelled");
                    return Ok(());
                }
            };

            let key = items[selection].as_str();
            todo.mark(key, true)?;
            save_todos(&todo);
        }
        Command::List { mode } => {
            let items = todo.list(mode.clone());

            match mode {
                ListMode::All => {
                    println!("# TODO");
                    items
                        .iter()
                        .filter(|x| x.1 == &true)
                        .for_each(|x| println!("* {}", *x.0));
                    println!();

                    println!("# DONE");
                    items
                        .iter()
                        .filter(|x| x.1 == &false)
                        .for_each(|x| println!("* {}", *x.0));
                }
                ListMode::Done => {
                    println!("# DONE");
                    items.iter().for_each(|x| println!(" * {}", *x.0));
                }
                ListMode::Todo => {
                    println!("# TODO");
                    items.iter().for_each(|x| println!(" * {}", *x.0));
                }
            }
        }
        Command::Delete => {
            let items = todo.all();

            if items.is_empty() {
                println!("No todos to delete");
                return Ok(());
            }

            let selection = match Select::new()
                .with_prompt("Select todo to delete")
                .items(&items)
                .default(0)
                .interact()
            {
                Ok(index) => index,
                Err(_) => {
                    println!("Action cancelled");
                    return Ok(());
                }
            };

            let key = items[selection].as_str();

            let confirm = Confirm::new()
                .with_prompt(format!("Are you sure you want to delete '{}'?", key))
                .default(false)
                .interact()
                .map_err(|e| e.to_string())?;

            if !confirm {
                println!("Delete cancelled");
                return Ok(());
            }

            todo.remove(key)?;
            save_todos(&todo);
            println!("Deleted '{}'", key);
        }
    };

    Ok(())
}
