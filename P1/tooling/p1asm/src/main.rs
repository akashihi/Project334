use std::collections::HashMap;
use std::fs;
use std::path::Path;
use clap::{command, Arg};
use std::fs::read_to_string;
use std::io::Write;
use std::str::SplitWhitespace;

fn upper_nibble(v: u8) -> u8 {
    (v & 0b1111_0000) >> 4
}

fn lower_nibble(v: u8) -> u8 {
    v & 0b0000_1111
}

fn op_split(instruction: &str, opcode: u8, value: u8, nibbler: fn(u8) -> u8) -> u8 {
    let op =  opcode | nibbler(value);
    println!("{} {} -> {:08b}", instruction, value, op);
    op
}

fn op_up(instruction: &str, opcode: u8, operations: &mut SplitWhitespace, nibbler: fn(u8) -> u8) -> u8 {
    if let Some(value) = operations.next().and_then(|s| s.parse::<u8>().ok()) {
        op_split(instruction, opcode, value, nibbler)
    } else {
        panic!("Invalid value for {}", instruction);
    }
}

fn op_lup(instruction: &str, operations: &mut SplitWhitespace) -> u8 {
    op_up(instruction, 0b0100_0000, operations, upper_nibble)
}

fn op_pup(instruction: &str, operations: &mut SplitWhitespace) -> u8 {
    op_up(instruction, 0b0101_0000, operations, lower_nibble)
}

fn main() {
    let single_ops: HashMap<&str, u8> = HashMap::from([
        ("NOP", 0b0000_0000),
        ("PTS", 0b1100_0000),
        ("EXY", 0b1100_1011),
        ("RTS", 0b1100_1111),
        ("RPV", 0b1100_1101),
        ("ADD", 0b1100_0000),
        ("SUB", 0b1100_0001),
        ("MUL", 0b1100_1010),
        ("LSH", 0b1100_0011),
        ("RSH", 0b1100_0010),
        ("AND", 0b1100_0110),
        ("OR",  0b1100_0111),
        ("XOR", 0b1100_0100),
        ("NOT", 0b1100_0101),
        ("RET", 0b1100_1000),
        ("CLR", 0b1100_1001),
        ("POP", 0b1100_1110),
    ]);

    let reg_ops: HashMap<&str, u8> = HashMap::from([
        ("RVR",  0b0110_0000),
        ("WVR",  0b1101_0000),
        ("LD",   0b1000_0000),
        ("ST",   0b1001_0000),
        ("JZ",   0b1010_0000),
        ("JMP",  0b1011_0000),
        ("CALL", 0b1111_0000),
        ("JGT",  0b1110_0000),
    ]);

    let regs: HashMap<&str, u8> = HashMap::from([
        ("Y",  0b0000_0000), 
        ("R1", 0b0000_0001),
        ("R2", 0b0000_0010),
        ("R3", 0b0000_0011),
        ("R4", 0b0000_0100),
        ("R5", 0b0000_0101),
        ("R6", 0b0000_0110),
        ("R7", 0b0000_0111),
        ("R8", 0b0000_1000),
        ("R9", 0b0000_1001),
        ("R10", 0b0000_1010),
        ("R11", 0b0000_1011),
        ("R12", 0b0000_1100),
        ("R13", 0b0000_1101),
        ("R14", 0b0000_1110),
        ("R15", 0b0000_1111),
    ]);

    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("name"))
        .get_matches();

    let input_file = matches.get_one::<String>("name").expect("Usage: p1asm <source.p1s>");
    let output_file = Path::new(input_file).with_extension("p1b");
    let mut output: Vec<u8> = Vec::with_capacity(256); // Max program mem size

    println!("Converting {} input file to {:?} output binary", input_file, output_file);

    let source = read_to_string(input_file).unwrap().to_uppercase();
    for line in source.lines() {
        if line.starts_with('#') { // Handle comments in the beginning of the line
            continue;
        }
        let mut operations = line.split_whitespace();
        if let Some(instruction) = operations.next() {
            // Operand-less instructions
            if let Some(opcode) = single_ops.get(instruction) {
                output.push(*opcode);
                println!("{} -> {:08b}", instruction, opcode);
                continue
            }

            // Operands instructions
            if let Some(opcode) = reg_ops.get(instruction) {
                if let Some(reg) = operations.next() {
                    if let Some(reg_code) = regs.get(reg) {
                        let opcode_with_reg = opcode | reg_code;
                        output.push(opcode_with_reg);
                        println!("{} {} -> {:08b}", instruction, reg, opcode_with_reg)
                    } else {
                        panic!("Unknown register: {}", reg)
                    }
                } else {
                    println!("{} -> {:08b}", instruction, opcode)
                }
                continue
            }

            // Nibble loading instructions
            if instruction == "LUP" {
                output.push(op_lup(instruction, &mut operations));
            }
            if instruction == "PUP" {
                output.push(op_pup(instruction, &mut operations));
            }

            // SET shortcut for loading either a whole number or a pair of LUP/PUP
            if instruction == "SET" {
                if let Some(value) = operations.next().and_then(|s| s.parse::<u8>().ok()) {
                    if value > 0 && value < 64 {
                        output.push(value);
                        println!("{} {} -> {:08b}", instruction, value, value);
                    } else {
                        output.push(op_split("LUP", 0b0100_0000, value, upper_nibble));
                        output.push(op_split("PUP", 0b0101_0000, value, lower_nibble));
                    }
                } else {
                    panic!("Invalid value for SET");
                }
                continue
            }
        }

    }

    if output.len() > 256 {
        panic!("Generated too many commands: {}", output.len())
    }

    let mut file = fs::OpenOptions::new().create(true).write(true).truncate(true).open(output_file).unwrap();
    file.write_all(&output).unwrap();
}
