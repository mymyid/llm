use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "ITeung")]
#[command(about = "ITeung is AI assistant which is tailored just for you", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Ask a question
    Ask {
        /// The question to ask
        query: String,
    },
    /// Tell Tera something to remember
    Remember {
        /// The content to remember
        content: String,
    },
}