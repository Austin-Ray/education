use assembler::code;
use assembler::parser::InstructionType;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();

    let input_path = args.input;
    let mut output_path = input_path.clone();
    output_path.set_extension("hack");

    let in_file = File::open(input_path)?;
    let out_file = File::create(output_path)?;

    let reader = BufReader::new(in_file);

    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let mut parser = assembler::parser::Parser::new(lines);
    let mut writer = BufWriter::new(out_file);

    while parser.has_more_lines() {
        let line_out = match parser.instruction_type() {
            InstructionType::A => format!("{:016b}", parser.symbol().parse::<usize>().unwrap()),
            InstructionType::C => {
                let comp = parser.comp();
                let dest = parser.dest();
                let jump = parser.jump();

                let a_bit = if comp.contains("M") { 1 } else { 0 };

                format!(
                    "111{}{}{}{}",
                    a_bit,
                    code::comp(&comp),
                    code::dest(&dest),
                    code::jump(&jump)
                )
            }
            _ => "".to_string(),
        };

        writeln!(&mut writer, "{}", line_out)?;
        parser.advance();
    }

    writer.flush()?;
    Ok(())
}
