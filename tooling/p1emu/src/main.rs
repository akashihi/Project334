use std::fs;
use std::fs::File;
use std::io::Read;
use clap::{command, Arg};
use anyhow::Result;
use thiserror::Error;

mod cpu;

#[derive(Error, Debug)]
pub enum MemError {
    #[error("Memory snapshot file {0} is too big")]
    InvalidDataSize(String),
}

fn mem_loader(file: &str, target: &mut [u8]) -> Result<()> {
    let metadata = fs::metadata(file)?;
    if metadata.len() > 256 {
        return Err(MemError::InvalidDataSize(file.to_string()).into())
    }
    let mut f = File::open(&file)?;
    let read_bytes = f.read(target)?;
    Ok(println!("Loaded {} bytes", read_bytes))
}

fn main() {
    // Memory pages for both program and data
    let mut prog_mem: Vec<u8> = vec![0;256];
    let mut data_mem: Vec<u8> = vec![0;256];

    // Get arguments
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("program").required(true))
        .arg(Arg::new("ram"))
        .get_matches();

    // Load program and data snapshots
    let program_file = matches.get_one::<String>("program").unwrap();
    println!("Loading program code from {}", program_file);
    mem_loader(program_file, &mut prog_mem).unwrap();
    if let Some(data_file) = matches.get_one::<String>("ram") {
        println!("Loading data memory snapshot from {}", data_file);
        mem_loader(data_file, &mut data_mem).unwrap();
    }

    // CPU interface
    let mut cpu = cpu::CPU::new();
    loop {
        match cpu.step(&prog_mem, &mut data_mem) {
            Ok(_) => {}
            Err(e) => {println!("Stopping CPU due to: {}", e); break}
        }
    }
}
