use clap::{Parser, Subcommand, ValueEnum};

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
    DoneOnly,
    Todo,
}

#[derive(Subcommand)]
pub enum Command {
    Add {
        key: String,
    },
    #[command(name = "done")]
    MarkDone {
        key: String,
    },
    List {
        #[arg(
            long,
            value_enum,
            default_value_t = ListMode::All,
        )]
        mode: ListMode,
    },
}
