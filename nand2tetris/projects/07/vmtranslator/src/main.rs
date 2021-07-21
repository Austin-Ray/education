use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::PathBuf;
use structopt::StructOpt;
use vmtranslator::code;
use vmtranslator::parser;

#[derive(StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();

    let input_path = args.input;
    let mut output_path = input_path.clone();
    output_path.set_extension("asm");

    let in_file = File::open(input_path.clone())?;
    let out_file = File::create(output_path)?;

    let reader = BufReader::new(in_file);

    let mut parser = parser::Parser::new(reader.lines());
    let buf_writer = BufWriter::new(out_file);
    let mut writer = code::CodeWriter::new(buf_writer);

    while parser.has_more_lines() {
        let cmd = parser.command().as_ref().unwrap();

        writer.write(cmd)?;

        parser.advance();
    }

    writer.close()?;

    Ok(())
}
