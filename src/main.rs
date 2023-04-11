mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use structopt::StructOpt;

fn main() -> Result<()> {
    match args::PngMeArgs::from_args() {
        args::PngMeArgs::Encode(args) => commands::encode(args),
        args::PngMeArgs::Decode(args) => commands::decode(args),
        args::PngMeArgs::Remove(args) => commands::remove(args),
        args::PngMeArgs::Print(args) => commands::print_chunks(args),
    }
}