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

    #[structopt(name = "no-sys-init", long)]
    no_sys_init: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();

    let input_path = args.input;

    let mut output_path = input_path.clone();
    if output_path.is_dir() {
        let out_clone = output_path.clone();
        let file_name = out_clone.file_name().unwrap();
        output_path.push(file_name);
    }
    output_path.set_extension("asm");

    let out_file = File::create(output_path)?;

    let buf_writer = BufWriter::new(out_file);
    let mut writer = code::CodeWriter::new(buf_writer, args.no_sys_init);

    let files = if input_path.is_dir() {
        input_path
            .read_dir()
            .unwrap()
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .map(|x| x.path())
            .filter(|x| x.extension().unwrap().to_str().unwrap() == "vm")
            .collect()
    } else {
        vec![input_path]
    };

    for file in files {
        writer.on_new_file();
        let in_file = File::open(file.clone())?;
        let reader = BufReader::new(in_file);
        let mut parser = parser::Parser::new(reader.lines(), file.to_str().unwrap().to_string());

        while parser.has_more_lines() {
            let cmd = parser.command().as_ref().unwrap();

            writer.write(cmd)?;

            parser.advance();
        }
    }

    writer.close()?;

    Ok(())
}
