use std::error::Error;
use std::fs;
use clap::Parser;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::commands::Commands;
use crate::png::Png;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

fn main() -> Result<(), Box<dyn Error>> {
    let cli: commands::Cli = commands::Cli::parse();

    match &cli.command {
        Commands::Encode(encode_args) => {
            let mut png: Png = Png::from_file(encode_args.filename.as_os_str())?;
            let chunk: Chunk = Chunk::new(
                ChunkType::try_from(
                    <&[u8] as TryInto<[u8; 4]>>::try_into(encode_args.chunk_type.as_bytes()).unwrap()
                )?,
                encode_args.message.as_bytes().to_vec()
            );
            png.append_chunk_somewhere(chunk);
            fs::write(encode_args.output_filename.to_owned().unwrap(), png.as_bytes())?;
        }
        Commands::Decode(decode_args) => {
            let png: Png = Png::from_file(decode_args.filename.as_os_str())?;
            let chunk: Option<&Chunk> = png.chunk_by_type(decode_args.chunk_type.as_str());
            match chunk {
                Some(chunk) => {
                    println!("The following chunk has been found for you: {}", chunk);
                }
                None => {
                    println!("No message for you here!");
                }
            }
        }
        Commands::Remove(remove_args) => {
            let mut png: Png = Png::from_file(remove_args.filename.as_os_str())?;
            let result_removal: Result<Chunk, Box<dyn Error>> = png.remove_first_chunk(
                remove_args.chunk_type.as_str()
            );
            match result_removal {
                Ok(chunk) => {
                    println!("Removing: {}", chunk);
                    fs::write(remove_args.filename.to_owned(), png.as_bytes())?;
                }
                Err(err) => {println!("{}", err);}
            }
        }
        Commands::Print(print_args) => {
            let png: Png = Png::from_file(print_args.filename.as_os_str())?;
            println!("{}", png);
        }
    }

    Ok(())
}
