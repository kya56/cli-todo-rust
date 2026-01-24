use clap::Parser;
use todo_cli::cli::{Cli, Command, ListMode};
use todo_cli::todo::TodoList;

fn main() {
    let cli = Cli::parse();

    let mut todo = TodoList::new();

    todo.add("Something to do".to_string());
    todo.add("Something else to do".to_string());
    todo.add("Something done".to_string());
    todo.mark("Something done".to_string(), false).unwrap();

    let result = match cli.command {
        Command::Add { key } => {
            todo.add(key);
            Ok(())
        }
        Command::MarkDone { key } => todo
            .mark(key, false)
            .map_err(|e| format!("Invalid key {}", e))
            .and(Ok(())),
        Command::List { mode } => {
            let items = todo.list(mode.clone());

            match mode {
                ListMode::All => {
                    println!("# TODO");
                    items
                        .iter()
                        .filter(|x| x.1 == &false)
                        .for_each(|x| println!("* {}", *x.0));
                    println!();

                    println!("# DONE");
                    items
                        .iter()
                        .filter(|x| x.1 == &true)
                        .for_each(|x| println!("* {}", *x.0));
                }
                ListMode::DoneOnly => {
                    println!("# DONE");
                    items.iter().for_each(|x| println!(" * {}", *x.0));
                }
                ListMode::Todo => {
                    println!("# TODO");
                    items.iter().for_each(|x| println!(" * {}", *x.0));
                }
            }
            Ok(())
        }
    };

    match result {
        Err(e) => println!("ERROR: {}", e),
        Ok(_) => println!("SUCCESS"),
    }
}
