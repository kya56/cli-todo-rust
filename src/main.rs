use clap::Parser;
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
        Command::MarkDone { key } => {
            todo.mark(key, false)
                .map_err(|e| format!("Invalid key {}", e))?;

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
        Command::Delete { key } => {
            todo.remove(&key)?;
            save_todos(&todo);
        }
    };

    Ok(())
}
