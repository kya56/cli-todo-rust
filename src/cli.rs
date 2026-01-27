use clap::{Parser, Subcommand, ValueEnum};
use dialoguer::{Confirm, Select};

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

pub fn prompt_select<T: std::fmt::Display>(
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

pub fn prompt_confirm(prompt_title: &str) -> Result<bool, String> {
    Confirm::new()
        .with_prompt(prompt_title)
        .default(false)
        .interact()
        .map_err(|e| e.to_string())
}
