use std::error::Error;
use clap::Parser;
use crate::commands::Commands;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

fn main() -> Result<(), Box<dyn Error>> {
    let cli: commands::Cli = commands::Cli::parse();

    match &cli.command {
        Commands::Encode(encode_args) => {
            println!(
                "encode used: args are {:?} {} {} {:?}",
                encode_args.filename, encode_args.chunk_type,
                encode_args.message, encode_args.output_filename
            );
        }
        Commands::Decode(decode_args) => {
            println!(
                "decode used: args are {:?} {}",
                decode_args.filename, decode_args.chunk_type
            );
        }
        Commands::Remove(remove_args) => {
            println!(
                "remove used: args are {:?} {}",
                remove_args.filename, remove_args.chunk_type
            );
        }
        Commands::Print(print_args) => {
            println!(
                "print used: file to print {:?}",
                print_args.filename
            );
        }
    }

    Ok(())
}
