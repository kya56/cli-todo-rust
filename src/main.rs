use clap::Parser;
use dialoguer::Input;
use todo_cli::cli::{Cli, Command, ListMode, prompt_confirm, prompt_select};
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

            let Some(selection) = prompt_select(&items, "Select a todo to mark as done")? else {
                println!("Action cancelled");
                return Ok(());
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

            let Some(selection) = prompt_select(&items, "Select completed todo to undo done")?
            else {
                println!("Action cancelled");
                return Ok(());
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
        Command::Update => {
            let items = todo.list();

            if items.is_empty() {
                println!("No todos to update");
                return Ok(());
            }

            let Some(selection) = prompt_select(&items, "Select toddo to update")? else {
                println!("Action cancelled");
                return Ok(());
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
                    return Ok(());
                }
            };

            if new_title.trim() == title {
                println!("Title unchanged");
                return Ok(());
            }

            todo.update_title(id, &new_title)?;
            save_todos(&todo);

            println!("Todo '[{}] {}' updated to {}", id, title, new_title);
        }
        Command::Delete => {
            let items = todo.list();

            if items.is_empty() {
                println!("No todos to delete");
                return Ok(());
            }

            let Some(selection) = prompt_select(&items, "Select todo to delete")? else {
                println!("Action cancelled");
                return Ok(());
            };

            let id = items[selection].id;
            let title = items[selection].title.clone();

            let confirm =
                prompt_confirm(format!("Are you sure you want to delete '{}'?", title).as_str())?;

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
