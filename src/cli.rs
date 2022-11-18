use clap::Parser;

/// Print a short but unique string representing the current working directory
#[derive(Parser, Debug)]
pub struct Cli {
    /// Set the shortest length that a directory can be truncated
    #[arg(short, long, default_value_t = 3)]
    pub shortest: usize,

    /// Set the truncation glyph
    #[arg(short, long, default_value_t = '…')]
    pub indicator: char,
}
