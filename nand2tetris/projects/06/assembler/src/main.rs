use assembler::code;
use assembler::instruction::Instruction;
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

    let mut in_file = File::open(input_path.clone())?;
    let out_file = File::create(output_path)?;

    let mut reader = BufReader::new(in_file);

    let mut parser = parser::Parser::new(reader.lines());
    let mut writer = BufWriter::new(out_file);
    let mut symbol_table = symbol::SymbolTable::new();

    parser.advance();
    init_symbol_table(&mut parser, &mut symbol_table);

    // Reset the parser.
    in_file = File::open(input_path)?;
    reader = BufReader::new(in_file);
    parser = parser::Parser::new(reader.lines());

    parser.advance();
    emit_assembly(&mut parser, &mut symbol_table, &mut writer)?;

    writer.flush()?;
    Ok(())
}

fn init_symbol_table(parser: &mut parser::Parser, symbol_table: &mut symbol::SymbolTable) {
    while parser.has_more_lines() {
        let current_line = parser.current_line_number();
        let curr_inst = parser.get_current_instruction();

        if let Some(Instruction::L(symbol)) = curr_inst {
            symbol_table.add_entry(symbol, current_line);
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
        let line_out = match parser.get_current_instruction().as_ref().unwrap() {
            Instruction::C { dest, comp, jump } => {
                let a_bit = if comp.contains('M') { 1 } else { 0 };

                Some(format!(
                    "111{}{}{}{}",
                    a_bit,
                    code::comp(comp),
                    code::dest(dest.as_ref().unwrap_or(&"".to_string())),
                    code::jump(jump.as_ref().unwrap_or(&"".to_string()))
                ))
            }
            Instruction::AConst(num) => Some(format!("{:016b}", num)),
            Instruction::AVar(var) => {
                if !symbol_table.contains(var) {
                    symbol_table.add_entry(var, 16 + variable_count);
                    variable_count += 1;
                }

                Some(format!("{:016b}", symbol_table.get_address(var)))
            }
            Instruction::L(_) => {
                // Skip over.
                None
            }
        };

        if let Some(contents) = line_out {
            writeln!(writer, "{}", contents)?;
        }

        parser.advance();
    }

    Ok(())
}
