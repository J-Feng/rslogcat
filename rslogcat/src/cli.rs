use clap::Parser;

/// rslogcat [-f <file>] [pattern] [directory1 directory2 ...]
#[derive(Parser, Debug, Clone)]
#[command(version="0.1.0", author = "Feng Jie", about, long_about = None)]
pub(crate) struct Args {
    /// File to search
    #[arg(short, long)]
    pub file: Option<String>,

    /// Search pattern
    pub pattern: String,

    /// Directories to search
    pub directories: Vec<String>
}
