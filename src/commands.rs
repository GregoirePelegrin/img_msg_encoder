use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args)]
pub struct EncodeArgs {
    pub filename: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_filename: Option<PathBuf>,
}
#[derive(Args)]
pub struct DecodeArgs {
    pub filename: PathBuf,
    pub chunk_type: String,
}
#[derive(Args)]
pub struct RemoveArgs {
    pub filename: PathBuf,
    pub chunk_type: String,
}
#[derive(Args)]
pub struct PrintArgs {
    pub filename: PathBuf,
}
