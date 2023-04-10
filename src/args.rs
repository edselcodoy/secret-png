
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name="secret-png", about="Slip secret messages into PNG files")]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(StructOpt, Debug)]
#[structopt(name="encode", about="Insert the message into PNG")]
pub struct EncodeArgs {
    #[structopt(parse(from_os_str), help="Path to the input PNG")]
    pub file_path: PathBuf,
    #[structopt(help="4-byte chunk type, e.g. 'ruSt'")]
    pub chunk_type: String,
    #[structopt(help="Secret message")]
    pub message: String,
    #[structopt(parse(from_os_str), help = "Optional path to the output PNG file")]
    pub output_file: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "decode", about = "Read the message from PNG")]
pub struct DecodeArgs {
    #[structopt(parse(from_os_str), help = "Path to the input PNG")]
    pub file_path: PathBuf,
    #[structopt(help="4-byte chunk type, e.g. 'ruSt'")]
    pub chunk_type: String,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "remove", about = "Remove a chunk from a PNG file")]
pub struct RemoveArgs {
    #[structopt(parse(from_os_str), help = "Path to the input PNG")]
    pub file_path: PathBuf,
    #[structopt(help="4-byte chunk type, e.g. 'ruSt'")]
    pub chunk_type: String,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "print", about = "Print all chunks in a PNG file")]
pub struct PrintArgs {
    #[structopt(parse(from_os_str), help = "Path to the input PNG")]
    pub file_path: PathBuf,
}