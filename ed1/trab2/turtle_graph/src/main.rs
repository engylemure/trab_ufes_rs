pub mod turtle;
use std::{fs::File, io::prelude::*, str::FromStr};

pub use turtle::*;
pub mod list;
pub use list::*;
pub mod tree;
pub use tree::*;

struct TurtleArgs {
    input_file_path: String,
    output_file_path: String,
    syntax_file_path: String,
}

fn get_args() -> Option<TurtleArgs> {
    let mut args = std::env::args();
    args.next()?;
    let input_file_path = args.next()?;
    let output_file_path = args.next()?;
    let syntax_file_path = args.next()?;
    Some(TurtleArgs {
        input_file_path,
        output_file_path,
        syntax_file_path,
    })
}

fn main() -> Result<(), std::io::Error> {
    let args = get_args().expect(r#"This programm has three arguments {input_file_path} {output_file_path} {syntax_file_path} "#);
    let mut input = String::new();
    File::open(args.input_file_path)?.read_to_string(&mut input)?;
    let config = TurtleGraphConfig::from_str(&input).unwrap();
    let syntax = config.generate_syntax();
    let mut syntax_file = File::create(args.syntax_file_path)?;
    syntax_file.write_all(syntax.string().as_bytes())?;
    let mut output_file = File::create(&args.output_file_path)?;
    println!("Generating output file: {}", args.output_file_path);
    output_file.write_all(syntax.convert().as_bytes())?;
    Ok(())
}
