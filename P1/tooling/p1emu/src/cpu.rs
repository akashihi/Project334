use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CpuError {
    #[error("NOP treated as stop condition")]
    NOP,
    #[error("Invalid opcode {0}")]
    InvalidOpcode(u8),
    #[error("Tried to return without a CALL made")]
    InvalidReturn,
    #[error("Too depp CALL is made")]
    DeepCall
}

pub struct CPU {
    pc: u8,
    x: u8,
    y: u8,
    z: u8,
    t: u8,
    x0: u8,
    registers: Vec<u8>,
    stack: Vec<u8>,
    stack_ptr: usize,
}

impl CPU {
    pub fn new() -> Self {
        return CPU {
            pc: 0,
            x: 0,
            y: 0,
            z: 0,
            t: 0,
            x0: 0,
            registers: vec![0;15],
            stack: vec![0;4],
            stack_ptr: 0
        }
    }

    pub fn step(&mut self, prog_mem: &[u8], data_mem: &mut[u8]) -> Result<()> {
        // Fetch instruction
        self.dump_state(data_mem);
        let opcode:u8 = prog_mem[self.pc as usize];
        print!("Opcode: {:08b}, Instruction: ", opcode);

        // Advance PC
        self.pc = self.pc.wrapping_add(1);

        // Decode
        if opcode == 0x00 {
            println!("NOP");
            return Err(CpuError::NOP.into());
        }

        if (opcode & 0b11110000) == 0b11000000 {
            match opcode {
                0b11000000 => self.pts()?,
                0b11000001 => self.exy()?,
                0b11000010 => self.rts()?,
                0b11000011 => self.rpv()?,
                0b11000100 => self.add()?,
                0b11000101 => self.sub()?,
                0b11000110 => self.mul()?,
                0b11000111 => self.lsh()?,
                0b11001000 => self.rsh()?,
                0b11001001 => self.and()?,
                0b11001010 => self.or()?,
                0b11001011 => self.xor()?,
                0b11001100 => self.not()?,
                0b11001101 => self.ret()?,
                0b11001110 => self.clr()?,
                0b11001111 => self.pop()?,
                _ => return Err(CpuError::InvalidOpcode(opcode).into())
            }
        } else if (opcode & 0b11000000) == 0 {
            // Direct loading of 63 immediate
            println!("Immediate: {}", opcode);
            self.x = opcode;
        } else {
            let op = (opcode & 0b11110000) >> 4;
            let operand = opcode & 0b00001111;

            match op {
                0b00000100 => self.lup(operand)?,
                0b00000101 => self.pup(operand)?,
                0b00000110 => self.rvr(operand)?,
                0b00001101 => self.wvr(operand)?,
                0b00001000 => self.ld(operand, data_mem)?,
                0b00001001 => self.st(operand, data_mem)?,
                0b00001010 => self.jz(operand)?,
                0b00001011 => self.jmp(operand)?,
                0b00001111 => self.call(operand)?,
                0b00001110 => self.jgt(operand)?,
                _ => return Err(CpuError::InvalidOpcode(opcode).into())
            }
        }

        Ok(())
    }

    fn dump_state(&self, data_mem: &[u8]) {
        println!("PC: {:03}, X0: {:03}, X: {:03}, Y: {:03}, Z: {:03}, T: {:03}, S0: {:03}, S1: {:03}, S2: {:03}, S3: {:03}", self.pc, self.x0, self.x, self.y, self.z, self.t, self.stack[0], self.stack[1], self.stack[2], self.stack[3]);
        for i in 0..15 {
            print!("R{}: {:03} ", i+1, self.registers[i])
        }
        println!();
        print!("Output: ");
        for i in 246..255 {
            print!("{:03} ", data_mem[i])
        }
        print!(" : ");
        for i in 246..255 {
            print!("{} ", char::from_u32(data_mem[i] as u32).unwrap_or(' '));
        }
        println!();
    }

    // Instruction helpers
    fn push_up(&mut self) {
        self.t = self.z;
        self.z = self.y;
        self.y = self.x;
    }

    fn push_down(&mut self, op: impl Fn() -> u8) {
        self.x0 = self.x;
        self.x = op();
        self.y = self.z;
        self.z = self.t;
    }

    fn indirect_address(&mut self, operand: u8) -> usize {
        let mut address = self.y as usize;
        if operand > 0 {
            let reg = (operand - 1) as usize;
            address = self.registers[reg] as usize;
            if operand>=1 && operand <=5 {
                self.registers[reg] = self.registers[reg] - 1;
            } else if operand>=11 && operand <= 15 {
                self.registers[reg] = self.registers[reg] + 1;
            }
        }
        address
    }

    // Instructions
    fn pts(&mut self) -> Result<()> {
        println!("PTS");
        self.push_up();
        Ok(())
    }

    fn pop(&mut self) -> Result<()> {
        println!("POP");
        self.x = self.y;
        self.y = self.z;
        self.z = self.t;
        Ok(())
    }

    fn exy(&mut self) -> Result<()>{
        println!("EXY");
        let (x, y) = (self.x, self.y);
        self.x = y;
        self.y = x;
        Ok(())
    }

    fn rts(&mut self) -> Result<()>{
        println!("RTS");
        let x = self.x;
        self.x = self.y;
        self.y = self.z;
        self.z = self.t;
        self.t = x;
        Ok(())
    }

    fn rpv(&mut self) -> Result<()>{
        println!("RPV");
        self.push_up();
        self.x = self.x0;
        Ok(())
    }

    fn add(&mut self) -> Result<()>{
        println!("ADD");
        let (x,y) = (self.x, self.y);
        self.push_down(|| {x.wrapping_add(y)});
        Ok(())
    }

    fn sub(&mut self) -> Result<()>{
        println!("SUB");
        let (x,y) = (self.x, self.y);
        self.push_down(|| {x.wrapping_sub(y)});
        Ok(())
    }

    fn mul(&mut self) -> Result<()>{
        println!("MUL (do NOT use!)");
        let (x,y) = (self.x, self.y);
        self.push_down(|| {x.wrapping_mul(y)});
        Ok(())
    }

    fn lsh(&mut self) -> Result<()>{
        println!("LSH");
        self.x0 = self.x;
        self.x = self.x >> 1;
        Ok(())
    }

    fn rsh(&mut self) -> Result<()>{
        println!("RSH");
        self.x0 = self.x;
        self.x = self.x << 1;
        Ok(())
    }

    fn and(&mut self) -> Result<()>{
        println!("AND");
        let (x,y) = (self.x, self.y);
        self.push_down(|| {x & y});
        Ok(())
    }

    fn or(&mut self) -> Result<()>{
        println!("OR");
        let (x,y) = (self.x, self.y);
        self.push_down(|| {x | y});
        Ok(())
    }

    fn xor(&mut self) -> Result<()>{
        println!("XOR");
        let (x,y) = (self.x, self.y);
        self.push_down(|| {x ^ y});
        Ok(())
    }

    fn not(&mut self) -> Result<()>{
        println!("NOT");
        self.x0 = self.x;
        self.x = !self.x;
        Ok(())
    }

    fn ret(&mut self) -> Result<()>{
        println!("RET");
        if self.stack_ptr == 0 {
            return Err(CpuError::InvalidReturn.into())
        }
        self.pc = self.stack[self.stack_ptr - 1];
        self.stack[self.stack_ptr - 1] = 0;
        self.stack_ptr = self.stack_ptr -1;
        Ok(())
    }

    fn clr(&mut self) -> Result<()>{
        println!("CLR");
        self.x = 0;
        Ok(())
    }

    fn lup(&mut self, operand: u8) -> Result<()> {
        println!("LUP {}", operand << 4);
        self.x = (self.x & 0b00001111) | (operand << 4);
        Ok(())
    }

    fn pup(&mut self, operand: u8) -> Result<()> {
        println!("PUP {}", operand);
        self.x = (self.x & 0b11110000) | operand;
        Ok(())
    }

    fn rvr(&mut self, operand: u8) -> Result<()> {
        println!("RVR {}", operand);
        if operand != 0 {
            self.x = self.registers[(operand - 1) as usize];
        }
        Ok(())
    }

    fn wvr(&mut self, operand: u8) -> Result<()> {
        println!("WVR {}", operand);
        if operand != 0 {
            self.registers[(operand - 1) as usize] = self.x;
        }
        Ok(())
    }

    fn ld(&mut self, operand: u8, data_mem: &[u8]) -> Result<()> {
        println!("LD {}", operand);
        let address = self.indirect_address(operand);
        self.x = data_mem[address];
        Ok(())
    }

    fn st(&mut self, operand: u8, data_mem: &mut [u8]) -> Result<()> {
        println!("ST {}", operand);
        let address = self.indirect_address(operand);
        data_mem[address] = self.x;
        Ok(())
    }

    fn jz(&mut self, operand: u8) -> Result<()> {
        println!("JZ {}", operand);
        let address = self.indirect_address(operand);
        if self.x == 0 {
            self.pc = address as u8;
        }
        Ok(())
    }

    fn jmp(&mut self, operand: u8) -> Result<()> {
        println!("JMP {}", operand);
        let address = self.indirect_address(operand);
        self.pc = address as u8;
        Ok(())
    }

    fn call(&mut self, operand: u8) -> Result<()> {
        println!("CALL {}", operand);
        if self.stack_ptr >= 4 {
            return Err(CpuError::DeepCall.into())
        }
        let address = self.indirect_address(operand);
        self.stack_ptr = self.stack_ptr + 1;
        self.stack[self.stack_ptr - 1] = self.pc;
        self.pc = address as u8;
        Ok(())
    }

    fn jgt(&mut self, operand: u8) -> Result<()> {
        println!("JGT {}", operand);
        let address = self.indirect_address(operand);
        if (self.x & 0b10000000) == 0 {
            self.pc = address as u8;
        }
        Ok(())
    }

}