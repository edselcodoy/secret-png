use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::{Error, Result};
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{Chunk, ChunkType, Png};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file_path)?;
    let new_chunk = Chunk::new(ChunkType::from_str(args.chunk_type.as_str())?,
        args.message.as_bytes().to_vec());
    let output = args.output_file.unwrap_or(args.file_path);
    png.append_chunk(new_chunk);
    fs::write(&output, &png.as_bytes())?;
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(&args.file_path)?;
    match png.chunk_by_type(args.chunk_type.as_str()) {
        Some(chunk) => println!("Secret Message: {}", chunk.data_as_string()?),
        None => println!("No message found with chunk type '{}'", args.chunk_type),
    };
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file_path)?;
    match png.remove_chunk(args.chunk_type.as_str()) {
        Ok(chunk) => {
            fs::write(&args.file_path, &png.as_bytes())?;
            println!("Removed chunk: {}", chunk);

        },
        Err(e) => println!("{}", e),
    };
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let png = Png::from_file(&args.file_path)?;
    for chunk in png.chunks().iter() {
        println!("{}", chunk.data_as_string()?);
    }
    Ok(())
}