use clap::Parser;
use todo_cli::cli::{Cli, RunResult, run};
use todo_cli::file::{load_todos, save_todos};

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let mut todo = load_todos();

    if let RunResult::Changed = run(cli.command, &mut todo)? {
        save_todos(&todo);
    };

    Ok(())
}
