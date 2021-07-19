use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::os::windows::prelude::FileExt;

use instructions::instruction_to_byte;

pub type Byte = u8;
pub type Word = u32;

mod instructions;

fn write_output_byte(
    output: &mut File,
    data: Byte,
    total_bytes_written: &mut u32,
) -> Result<(), String> {
    match output.write_all(&data.to_le_bytes()) {
        Ok(_) => {
            *total_bytes_written += 1;
            Ok(())
        }
        Err(_) => Err("Failed to write to output file".to_string()),
    }
}

fn write_output_word(
    output: &mut File,
    data: Word,
    total_bytes_written: &mut u32,
) -> Result<(), String> {
    match output.write_all(&data.to_le_bytes()) {
        Ok(_) => {
            *total_bytes_written += 4;
            Ok(())
        }
        Err(_) => Err("Failed to write to output file".to_string()),
    }
}

fn write_parsed_number(
    output: &mut File,
    total_bytes_written: &mut u32,
    operand: &str,
    prefix: &str,
    radix: u32,
    name: &str,
    n: usize,
) -> Result<(), String> {
    match Word::from_str_radix(operand.trim_start_matches(prefix), radix) {
        Ok(op) => write_output_word(output, op, total_bytes_written),
        Err(_) => Err(format!(
            "Error parsing {} operand at line {}: {}",
            name, n, operand
        )),
    }
}

fn parse_line(
    output: &mut File,
    mut n: usize,
    line: &String,
    jump_label_locations: &mut Vec<(Word, String)>,
    total_bytes_written: &mut u32,
) -> Result<(), String> {
    n += 1;

    let mut split = line.splitn(2, ' ');
    let instruction = split.next();
    if instruction.is_none() {
        return Err(format!("Error reading instruction at line {}", n));
    }

    let instruction = instruction.unwrap();

    let instruction_code = instruction_to_byte(instruction);
    if instruction_code.is_none() {
        return Err(format!(
            "Unknown instruction at line {}: {}",
            n, instruction
        ));
    }

    // write instruction byte
    write_output_byte(output, instruction_code.unwrap().0, total_bytes_written)?;

    let operands = split.next();
    // if we have no operands but expect one, then error
    if operands.is_none() && instruction_code.unwrap().1 != 0 {
        return Err(format!(
            "Missing parameter for instruction {} at line {}",
            instruction, n
        ));
    // else if we have no operands and don't expect any, then return
    } else if operands.is_none() {
        return Ok(());
    }

    let operands = operands.unwrap().replace(" ", "");
    let operands_split = operands.split(',');
    // keep track of number of operands for error checking
    let mut operand_count = 0;
    for operand in operands_split {
        let op: Word;

        if operand.starts_with("0x") {
            write_parsed_number(output, total_bytes_written, operand, "0x", 16, "hex", n)?;
        } else if operand.starts_with("0b") {
            write_parsed_number(output, total_bytes_written, operand, "0b", 2, "bin", n)?;
        } else if operand.to_uppercase().starts_with("R") {
            // remove the R part of the register name
            let reg_name = operand[1..].to_string();

            let reg_num = reg_name.parse::<Word>();
            if reg_num.is_err() {
                return Err(format!("Error parsing register at line {}: {}", n, operand));
            }

            // check if it is inbounds and calculate the offset
            let reg_num = reg_num.unwrap();
            if reg_num > 0 && reg_num <= 8 {
                op = (reg_num - 1) * 4;
            } else {
                return Err(format!("Invalid register at line {}: {}", n, operand));
            }

            write_output_word(output, op, total_bytes_written)?;
        } else if operand.starts_with(":") {
            // add the location of the label and the labels name for later and fill it with 0xFFFF FFFF FFFF FFFF
            jump_label_locations.push((*total_bytes_written, operand[1..].to_string()));
            write_output_word(output, Word::MAX, total_bytes_written)?;
        } else {
            write_parsed_number(output, total_bytes_written, operand, "", 10, "dec", n)?;
        }

        operand_count += 1;
    }

    // of the operator count of the current instruction is not equal to the number of operands expected, then error
    if operand_count != instruction_code.unwrap().1 {
        return Err(format!(
            "Wrong number of operands for instruction {} at line {}",
            instruction, n
        ));
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <input> <output>", args.get(0).unwrap());
        return Err("Invalid arguments".to_string());
    }

    let mut output = match File::create(args.get(2).unwrap()) {
        Ok(file) => file,
        Err(_) => {
            return Err(format!(
                "Error creating output file: {}",
                args.get(2).unwrap()
            ))
        }
    };
    let input = match File::open(args.get(1).unwrap()) {
        Ok(file) => BufReader::new(file),
        Err(_) => {
            return Err(format!(
                "Error opening input file: {}",
                args.get(1).unwrap()
            ))
        }
    };

    let mut jump_label_map: HashMap<String, Word> = HashMap::new();
    let mut jump_label_locations: Vec<(Word, String)> = Vec::new();
    let mut total_bytes_written: u32 = 0;

    // Parse the input file
    for (n, line) in input.lines().enumerate() {
        if line.is_err() {
            return Err(format!("Error reading line {}", n));
        }

        let line = line.unwrap();

        if !line.is_empty() {
            // comments with ; are allowed
            if line.starts_with(";") {
                continue;
            } else if line.starts_with("::") {
                // catch label definition and save the label plus its location in the output file
                let label = line[2..].to_string();

                jump_label_map.insert(label, total_bytes_written);

                continue;
            }

            match parse_line(
                &mut output,
                n,
                &line,
                &mut jump_label_locations,
                &mut total_bytes_written,
            ) {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                    break;
                }
            };
        }
    }

    for (location, label) in jump_label_locations {
        let address = *jump_label_map.get(&label).unwrap();
        match output.seek_write(&address.to_le_bytes(), location as u64) {
            Ok(_) => {}
            Err(_) => return Err(format!("Error writing jump label {}", label)),
        }
    }

    Ok(())
}
