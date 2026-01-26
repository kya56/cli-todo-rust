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
            let items = todo.list(ListMode::Todo);

            if items.is_empty() {
                println!("No todos to mark as done");
                return Ok(());
            }

            let titles: Vec<&str> = items.iter().map(|x| x.title.as_str()).collect();

            let selection = match Select::new()
                .with_prompt("Select todo to mark as done")
                .items(&titles)
                .default(0)
                .interact()
            {
                Ok(index) => index,
                Err(_) => {
                    println!("Action cancelled");
                    return Ok(());
                }
            };

            let title = titles[selection].to_string();
            todo.mark(&title, false)?;

            save_todos(&todo);
        }
        Command::UndoDone => {
            let items = todo.list(ListMode::Done);

            if items.is_empty() {
                println!("No todos to undo done");
                return Ok(());
            }

            let titles: Vec<&str> = items.iter().map(|x| x.title.as_str()).collect();

            let selection = match Select::new()
                .with_prompt("Select completed todo to undo done")
                .items(&titles)
                .default(0)
                .interact()
            {
                Ok(index) => index,
                Err(_) => {
                    println!("Action cancelled");
                    return Ok(());
                }
            };

            let title = titles[selection].to_string();
            todo.mark(&title, true)?;
            save_todos(&todo);
        }
        Command::List { mode } => {
            let items = todo.list(mode.clone());

            match mode {
                ListMode::All => {
                    println!("# TODO");
                    items
                        .iter()
                        .filter(|x| x.done == false)
                        .for_each(|x| println!("* {}", x.title));
                    println!();

                    println!("# DONE");
                    items
                        .iter()
                        .filter(|x| x.done == true)
                        .for_each(|x| println!("* {}", x.title));
                }
                ListMode::Done => {
                    println!("# DONE");
                    items.iter().for_each(|x| println!(" * {}", x.title));
                }
                ListMode::Todo => {
                    println!("# TODO");
                    items.iter().for_each(|x| println!(" * {}", x.title));
                }
            }
        }
        Command::Delete => {
            let items = todo.list(ListMode::All);

            if items.is_empty() {
                println!("No todos to delete");
                return Ok(());
            }

            let titles: Vec<&str> = items.iter().map(|x| x.title.as_str()).collect();

            let selection = match Select::new()
                .with_prompt("Select todo to delete")
                .items(&titles)
                .default(0)
                .interact()
            {
                Ok(index) => index,
                Err(_) => {
                    println!("Action cancelled");
                    return Ok(());
                }
            };

            let title = titles[selection].to_string();

            let confirm = Confirm::new()
                .with_prompt(format!("Are you sure you want to delete '{}'?", title))
                .default(false)
                .interact()
                .map_err(|e| e.to_string())?;

            if !confirm {
                println!("Delete cancelled");
                return Ok(());
            }

            todo.remove(&title)?;
            save_todos(&todo);
            println!("Deleted '{}'", title);
        }
    };

    Ok(())
}
