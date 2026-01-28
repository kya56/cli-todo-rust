use clap::Parser;
use todo::cli::{Cli, RunResult, run};
use todo::file::{load_todos, save_todos};
use todo::prompter::DialoguerPrompter;

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let mut todo = load_todos();

    if let RunResult::Changed = run(cli.command, &mut todo, &DialoguerPrompter)? {
        save_todos(&todo);
    };

    Ok(())
}
