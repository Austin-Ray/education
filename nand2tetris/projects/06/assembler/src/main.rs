use assembler::code;
use assembler::parser;
use assembler::symbol;
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

    let mut parser = parser::Parser::new(lines);
    let mut writer = BufWriter::new(out_file);
    let mut symbol_table = symbol::SymbolTable::new();

    init_symbol_table(&mut parser, &mut symbol_table);

    parser.reset();
    emit_assembly(&mut parser, &mut symbol_table, &mut writer)?;

    writer.flush()?;
    Ok(())
}

fn init_symbol_table(parser: &mut parser::Parser, symbol_table: &mut symbol::SymbolTable) {
    let mut label_count = 0;

    while parser.has_more_lines() {
        let current_line = parser.current_line();
        if parser.instruction_type() == parser::InstructionType::L {
            let parse_symbol = parser.symbol();
            symbol_table.add_entry(&parse_symbol, current_line - label_count);
            label_count += 1;
        }

        parser.advance();
    }
}

fn emit_assembly(
    parser: &mut parser::Parser,
    symbol_table: &mut symbol::SymbolTable,
    writer: &mut BufWriter<File>,
) -> std::io::Result<()> {
    let mut variable_count = 0;
    while parser.has_more_lines() {
        let line_out = match parser.instruction_type() {
            parser::InstructionType::C => {
                let comp = parser.comp();
                let dest = parser.dest();
                let jump = parser.jump();

                let a_bit = if comp.contains('M') { 1 } else { 0 };

                format!(
                    "111{}{}{}{}",
                    a_bit,
                    code::comp(&comp),
                    code::dest(&dest),
                    code::jump(&jump)
                )
            }
            parser::InstructionType::A => {
                let parse_symbol = parser.symbol();
                let final_symbol = match parse_symbol.parse::<usize>() {
                    Ok(address) => address,
                    Err(_) => {
                        if !symbol_table.contains(&parse_symbol) {
                            symbol_table.add_entry(&parse_symbol, 16 + variable_count);
                            variable_count += 1;
                        }

                        symbol_table.get_address(&parse_symbol)
                    }
                };
                format!("{:016b}", final_symbol)
            }
            parser::InstructionType::L => {
                // Skip over.
                "".to_string()
            }
        };

        if !line_out.is_empty() {
            writeln!(writer, "{}", line_out)?;
        }
        parser.advance();
    }
    Ok(())
}
