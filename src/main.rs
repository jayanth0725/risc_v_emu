use std::fs::File;
use std::io::Read;
use std::io::{self, Write};

struct CPU {
    /* 32 general-purpose registers. */
    registers: [u32; 32],

    /* The Program Counter */
    pc: usize,

    /* Simulated RAM: 1 Megabyte of bytes */
    memory: Vec<u8>,
}

impl CPU {
    fn new() -> Self {
        CPU {
            registers: [0; 32],
            pc: 0,
            memory: vec![0; 1024 * 1024],
        }
    }

    fn write_register(&mut self, index: usize, value: u32) {
        /* Enforcing x0 to always be 0. */
        if index != 0 {
            self.registers[index] = value;
        }
    }

    fn fetch(&mut self) -> u32 {
        let index = self.pc;

        /* Read 4 bytes from memory (Little Endian format). */
        let byte0 = self.memory[index] as u32;
        let byte1: u32 = self.memory[index + 1] as u32;
        let byte2: u32 = self.memory[index + 2] as u32;
        let byte3: u32 = self.memory[index + 3] as u32;

        /* Combine them into a 32-bit instruction */
        let instruction = byte0 | (byte1 << 8) | (byte2 << 16) | (byte3 << 24);

        return instruction;
    }

    fn step(&mut self) {
        let instruction = self.fetch();
        let opcode = instruction & 0x7F;

        let mut next_pc = self.pc + 4;

        match opcode {
            /*  Opcode for Arithemtic R-type instructions. */
            0x33 => {
                /* Extracting the rd, rs1 & rs2 registers. */
                let rd = ((instruction >> 7) & 0x1F) as usize;
                let rs1 = ((instruction >> 15) & 0x1F) as usize;
                let rs2 = ((instruction >> 20) & 0x1F) as usize;

                /* Extract funct3 & funct7 to know if it's ADD, SUB, XOR, etc. */
                let funct3 = (instruction >> 12) & 0x07;
                let funct7 = (instruction >> 25) & 0x7F;

                if funct3 == 0x0 && funct7 == 0x00 { // ADD
                    let result = self.registers[rs1].wrapping_add(self.registers[rs2]);
                    self.write_register(rd, result);
                    println!("Executed: ADD x{}, x{}, x{}", rd, rs1, rs2);
                }
                else if funct3 == 0x0 && funct7 == 0x20 { // SUB
                    let result: u32 = self.registers[rs1].wrapping_sub(self.registers[rs2]);
                    self.write_register(rd, result);
                    println!("Executed: SUB x{}, x{}, x{}", rd, rs1, rs2);                    
                }
                else if funct3 == 0x6 && funct7 == 0x00 { // OR
                    let result = self.registers[rs1] | self.registers[rs2];
                    self.write_register(rd, result);
                    println!("Executed: OR x{}, x{}, x{}", rd, rs1, rs2);
                }
                else if funct3 == 0x7 && funct7 == 0x00 { // AND
                    let result = self.registers[rs1] & self.registers[rs2];
                    self.write_register(rd, result);
                    println!("Executed: AND x{}, x{}, x{}", rd, rs1, rs2);
                }
                else if funct3 == 0x4 && funct7 == 0x00 { // XOR
                    let result = self.registers[rs1] ^ self.registers[rs2];
                    self.write_register(rd, result);
                    println!("Executed: XOR x{}, x{}, x{}", rd, rs1, rs2);
                }
                else if funct3 == 0x1 && funct7 == 0x00 { // SLL
                    let sh_amt = self.registers[rs2] & 0x1F;
                    let result = self.registers[rs1] << sh_amt;
                    self.write_register(rd, result);
                    println!("Executed: SLL x{}, x{}, x{}", rd, rs1, rs2);
                }
                else if funct3 == 0x5 && funct7 == 0x00 { // SRL
                    let sh_amt = self.registers[rs2] & 0x1F;
                    let result = self.registers[rs1] >> sh_amt;
                    self.write_register(rd, result);
                    println!("Executed: SRL x{}, x{}, x{}", rd, rs1, rs2);
                }
                else if funct3 == 0x5 && funct7 == 0x20 { // SRA
                    let sh_amt = self.registers[rs2] & 0x1F;
                    let result = (self.registers[rs1] as i32 >> sh_amt) as u32;
                    self.write_register(rd, result);
                    println!("Executed: SRA x{}, x{}, x{}", rd, rs1, rs2);
                }
                else {
                    println!("Unknown R-type instruction!");
                }
            },

            /* Opcode for Arithmetic I-type instructions. */
            0x13 => {
                /* Extracting the rd, rs1 registers. */
                let rd = ((instruction >> 7) & 0x1F) as usize;
                let rs1 = ((instruction >> 15) & 0x1F) as usize;

                /* Extract funct3 to know if it's ADDI, ORI, XORI, etc. */
                /* Extract 12-bit immediate, shift left by 20 and right by 20 while casting to an i32. THis forces Rust to perform an Arithmetic Shift, and automatically sign-extend negative numbers. */
                let funct3 = (instruction >> 12) & 0x07;
                let imm = ((instruction >> 20) & 0xFFF) as u32;
                let imm_extend = ((imm << 20) as i32 >> 20) as u32;
                let sh_amt = imm & 0x1F;

                match funct3 {
                    0x0 => { // ADDI
                        let result = self.registers[rs1].wrapping_add(imm_extend);
                        self.write_register(rd, result);
                        println!("Executed: ADDI x{}, x{}, {}", rd, rs1, imm_extend as i32);
                    },

                    0x4 => { // XORI
                        let result = self.registers[rs1] ^ imm_extend;
                        self.write_register(rd, result);
                        println!("Executed: XORI x{}, x{}, {}", rd, rs1, imm_extend as i32);
                    },
                    
                    0x6 => { // ORI
                        let result = self.registers[rs1] | imm_extend;
                        self.write_register(rd, result);
                        println!("Executed: ORI x{}, x{}, {}", rd, rs1, imm_extend as i32);
                    },

                    0x7 => { // ANDI
                        let result = self.registers[rs1] & imm_extend;
                        self.write_register(rd, result);
                        println!("Executed: ANDI x{}, x{}, {}", rd, rs1, imm_extend as i32);
                    },

                    0x1 => { // SLLI
                        let result = self.registers[rs1] << sh_amt;
                        self.write_register(rd, result);
                        println!("Executed: SLLI x{}, x{}, {}", rd, rs1, sh_amt);
                    },

                    0x5 => { // SRLI & SRAI
                        let funct7 = (imm >> 5) & 0x7F;
                        if funct7 == 0x00 { // SRLI
                            let result = self.registers[rs1] >> sh_amt;
                            self.write_register(rd, result);
                            println!("Executed: SRLI x{}, x{}, {}", rd, rs1, sh_amt);
                        }
                        else { // SRAI
                            let result = (self.registers[rs1] as i32 >> sh_amt) as u32;
                            self.write_register(rd, result);
                            println!("Executed: SRAI x{}, x{}, {}", rd, rs1, sh_amt);
                        }
                    },

                    _ => {
                        println!("Unknown I-type ARITHMETIC instruction!");
                    }
                }

            },

            /* Opcode for Load I-type instructions. */
            0x03 => {
                /* Extracting the rd, rs1 registers. */
                let rd = ((instruction >> 7) & 0x1F) as usize;
                let rs1 = ((instruction >> 15) & 0x1F) as usize;

                /* Extract funct3, extended immediate and address in memory from where to load the value into rd. */
                let funct3 = (instruction >> 12) & 0x07;
                let imm = ((instruction >> 20) & 0xFFF) as u32;
                let imm_extend = ((imm << 20) as i32 >> 20) as u32;
                let addr = self.registers[rs1].wrapping_add(imm_extend) as usize;

                match funct3 {
                    0x0 => { // LB
                        let val = self.memory[addr] as u32;
                        let result = ((val << 24) as i32 >> 24) as u32;
                        self.write_register(rd, result);
                        println!("Executed: LB x{}, {} => x{}", rd, imm_extend as i32, rs1);
                    },

                    0x1 => { // LH
                        let val = (self.memory[addr] as u32) | ((self.memory[addr+1] as u32) << 8);
                        let result = ((val << 16) as i32 >> 16) as u32;
                        self.write_register(rd, result);
                        println!("Executed: LH x{}, {} => x{}", rd, imm_extend as i32, rs1);
                    },

                    0x2 => { // LW
                        let result = (self.memory[addr] as u32) | ((self.memory[addr+1] as u32) << 8) | ((self.memory[addr+2] as u32) << 16) | ((self.memory[addr+3] as u32) << 24);
                        self.write_register(rd, result);
                        println!("Executed: LW x{}, {} => x{}", rd, imm_extend as i32, rs1);
                    },

                    0x4 => { // LBU
                        let result = self.memory[addr] as u32;
                        self.write_register(rd, result);
                        println!("Executed: LBU x{}, {} => x{}", rd, imm_extend as i32, rs1);
                    },

                    0x5 => { // LHU
                        let result = (self.memory[addr] as u32) | ((self.memory[addr+1] as u32) << 8);
                        self.write_register(rd, result);
                        println!("Executed: LHU x{}, {} => x{}", rd, imm_extend as i32, rs1);
                    },

                    _ => {
                        println!("Unknown I-type LOAD instruction!");
                    }
                }
            },

            /* Opcode for Jump-and-Link-Register J-type instruction. */
            0x67 => {   
                /* Extracting the rd, rs1 registers. */
                let rd = ((instruction >> 7) & 0x1F) as usize;
                let rs1 = ((instruction >> 15) & 0x1F) as usize;

                /* Extract imm and sign extend it to imm_extend. */
                let imm = ((instruction >> 20) & 0xFFF) as u32;
                let imm_extend = ((imm << 20) as i32 >> 20) as u32;

                self.write_register(rd, self.pc as u32 + 4);
                next_pc = (self.registers[rs1].wrapping_add(imm_extend) & !1) as usize;
                println!("Executed: JALR {} => x{}, PC => M({})", self.pc + 4, rd, next_pc);
            },

            /* Opcode for Store S-type instructions. */
            0x23 => {
                /* Extracting the rs1, rs2 registers. */
                let rs1 = ((instruction >> 15) & 0x1F) as usize;
                let rs2 = ((instruction >> 20) & 0x1F) as usize;

                /* Extract funct3, extended immediate, address in memory and value to be stored. */
                let funct3 = (instruction >> 12) & 0x07;
                let imm = ((((instruction >> 25) & 0x7F) << 5) | ((instruction >> 7) & 0x1F)) as u32;
                let imm_extend = ((imm << 20) as i32 >> 20) as u32;
                let addr = self.registers[rs1].wrapping_add(imm_extend) as usize;

                let val = self.registers[rs2];

                match funct3 {
                    0x0 => { // SB
                        /* Memory-Mapped I/O (MMIO): If the CPU tries to write to the address 0x10000000, it is intercepted and printed to the host terminal. */
                        if addr == 0x1000_0000 {
                            print!("{}", (val & 0xFF) as u8 as char);
                            io::stdout().flush().unwrap();
                        }
                        else {
                            self.memory[addr] = (val & 0xFF) as u8;
                            println!("Executed: SB x{}, x{} => M({})", rs2, imm_extend as i32, rs1);
                        }
                    },

                    0x1 => { // SH
                        self.memory[addr] = (val & 0xFF) as u8;
                        self.memory[addr+1] = ((val >> 8) & 0xFF) as u8;
                        println!("Executed: SH x{}, x{} => M({})", rs2, imm_extend as i32, rs1);
                    },

                    0x2 => { // SW
                        self.memory[addr] = (val & 0xFF) as u8;
                        self.memory[addr+1] = ((val >> 8) & 0xFF) as u8;
                        self.memory[addr+2] = ((val >> 16) & 0xFF) as u8;
                        self.memory[addr+3] = ((val >> 24) & 0xFF) as u8;
                        println!("Executed: SW x{}, x{} => M({})", rs2, imm_extend as i32, rs1);
                    },

                    _ => {
                        println!("Unknown S-type instruction!");
                    }
                }
            },

            /* Opcode for Branch B-type instructions. */
            0x63 => { 
                /* Extracting the rs1, rs2 registers. */
                let rs1 = ((instruction >> 15) & 0x1F) as usize;
                let rs2 = ((instruction >> 20) & 0x1F) as usize;

                /* Extract funct3, extended immediate, address in memory and value to be stored. */
                let funct3 = (instruction >> 12) & 0x07;
                let imm = (((instruction >> 31) & 1) << 12) | (((instruction >> 7) & 0x1) << 11) | (((instruction >> 25) & 0x3F) << 5) | (((instruction >> 8) & 0x0F) << 1);
                let imm_extend = ((imm << 19) as i32 >> 19) as u32;

                let mut branch_taken = false;
                match funct3 {
                    0x0 => { // BEQ
                        if self.registers[rs1] == self.registers[rs2] {
                            branch_taken = true;
                        }
                    },

                    0x1 => { // BNE
                        if self.registers[rs1] != self.registers[rs2] {
                            branch_taken = true;
                        }
                    },

                    0x4 => { // BLT
                        if (self.registers[rs1] as i32) < (self.registers[rs2] as i32) {
                            branch_taken = true;
                        }
                    },

                    0x5 => { // BGE
                        if (self.registers[rs1] as i32) >= (self.registers[rs2] as i32) {
                            branch_taken = true;
                        }
                    },

                    0x6 => { // BLTU
                        if self.registers[rs1] < self.registers[rs2] {
                            branch_taken = true;
                        }
                    },

                    0x7 => { // BGEU
                        if self.registers[rs1] >= self.registers[rs2] {
                            branch_taken = true;
                        }
                    },

                    _ => {
                        println!("Unknown B-type instruction!");
                    }
                }

                if branch_taken {
                    next_pc = self.pc.wrapping_add((imm_extend as i32) as usize);
                    println!("Executed: BRANCH taken to {}", imm_extend as i32);
                }
                else {
                    println!("Executed: BRANCH not taken");
                }
            },

            /* Opcode for Jump-and-Link I-type instruction. */
            0x6F => {
                /* Extract rd, extended immediate, address in memory and value to be stored. */
                let rd = ((instruction >> 7) & 0x1F) as usize;
                let imm = (((instruction >> 31) & 1) << 20) | (((instruction >> 12) & 0xFF) << 12) | (((instruction >> 20) & 1) << 11) | (((instruction >> 21) & 0x3FF) << 1);
                let imm_extend = ((imm << 11) as i32 >> 11) as u32;

                self.write_register(rd, self.pc as u32 + 4);
                next_pc = self.pc.wrapping_add((imm_extend as i32) as usize);
                println!("Executed: JAL {} => x{}, PC => M({})", self.registers[rd], self.pc as u32 + 4, next_pc);
            },

            /* Opcode for Environment Call (ecall) I-type instructions. */
            0x73 => {
                if instruction == 0x00000073 {
                    let syscall_id = self.registers[17];

                    match syscall_id {
                        64 => { // Custom print to terminal using syscall 64
                            let fd = self.registers[10];
                            let buffer_addr = self.registers[11] as usize;
                            let len = self.registers[12] as usize;

                            if fd == 1 || fd == 2 {
                                for i in 0..len {
                                    let c = self.memory[buffer_addr + i] as char;
                                    print!("{}", c);
                                }
                                io::stdout().flush().unwrap();
                            }
                            self.registers[10] = len as u32;
                        },

                        93 => { // Custom program exit using syscall 93
                            let exit_code = self.registers[10];
                            println!("\n[Emulator] Program exited with code {}", exit_code);
                            std::process::exit(exit_code as i32);
                        },

                        _ => {
                            println!("Unknown syscall: {}", syscall_id);
                        }
                    }
                }
            },

            /* Opcode for Load Upper Immediate U-type instruction. */
            0x37 => {   // LUI
                let rd = ((instruction >> 7) & 0x1F) as usize;

                let imm = instruction & 0xFFFFF000;

                self.write_register(rd, imm);
                println!("Executed: LUI x{}, {}", rd, imm);
            }

            _ => {
                println!("Unknown opcode: {}", opcode);
            }
        }

        self.pc = next_pc;
    }

    /* Function to write a raw 32-bit instruction directly into the memory: 
    fn write_mem_u32(&mut self, addr: usize, data: u32) {
        self.memory[addr] = (data & 0xFF) as u8;
        self.memory[addr + 1] = ((data >> 8) & 0xFF) as u8;
        self.memory[addr + 2] = ((data >> 16) & 0xFF) as u8;
        self.memory[addr + 3] = ((data >> 24) & 0xFF) as u8;
    } */

}

fn main() -> std::io::Result<()> {
    let mut cpu = CPU::new();

    // Initialise the Stack Pointer (x2) to point to the very top of the 1MB simulated RAM.
    cpu.registers[2] = cpu.memory.len() as u32;

    let mut file = File::open("test.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    for(i, byte) in buffer.iter().enumerate() {
        if i < cpu.memory.len() {
            cpu.memory[i] = *byte;
        }
    }

    while cpu.pc < buffer.len() {
        cpu.step();
    }

    Ok(())
}