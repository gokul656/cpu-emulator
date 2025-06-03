const MAX_MEMORY: usize = 1024;

enum Instruction {
    LoadRegImm(usize, u8),   // LOAD reg, imm8
    AddRegReg(usize, usize), // ADD reg_dst, reg_src
    SubRegReg(usize, usize), // SUB reg_dst, reg_src
    Jmp(usize),              // JMP addr
    Halt,                    // HALT
    Unknown,
}

struct VM {
    memory: [u8; MAX_MEMORY],
    registers: [u8; 4],
    pc: usize,
}

impl VM {
    fn new() -> Self {
        VM {
            memory: [0; MAX_MEMORY],
            registers: [0; 4],
            pc: 0,
        }
    }

    fn decode(&self) -> Instruction {
        let opcode = self.memory[self.pc];
        match opcode {
            0x01 => {
                let register = self.memory[self.pc + 1] as usize;
                let immediate_value = self.memory[self.pc + 2];
                Instruction::LoadRegImm(register, immediate_value)
            }
            0x02 => {
                let reg_dst = self.memory[self.pc + 1] as usize;
                let reg_src = self.memory[self.pc + 2] as usize;
                Instruction::AddRegReg(reg_dst, reg_src)
            }
            0x04 => {
                let reg_dst = self.memory[self.pc + 1] as usize;
                let reg_src = self.memory[self.pc + 2] as usize;
                Instruction::SubRegReg(reg_dst, reg_src)
            }
            0x03 => {
                let addr = self.memory[self.pc + 1] as usize;
                Instruction::Jmp(addr)
            }
            0xFF => Instruction::Halt,
            _ => Instruction::Unknown,
        }
    }

    fn exec(&mut self) -> bool {
        let instruction = self.decode();
        match instruction {
            Instruction::LoadRegImm(reg, imm_val) => {
                // set value in register (bug fix)
                if reg < self.registers.len() {
                    self.registers[reg] = imm_val;
                }
                self.pc += 3;
            }
            Instruction::AddRegReg(dst, src) => {
                if dst < self.registers.len() && src < self.registers.len() {
                    let (res, _) = self.registers[dst].overflowing_add(self.registers[src]);
                    self.registers[dst] = res;
                }
                self.pc += 3;
            }
            Instruction::SubRegReg(dst, src) => {
                if dst < self.registers.len() && src < self.registers.len() {
                    let (res, _) = self.registers[dst].overflowing_sub(self.registers[src]);
                    self.registers[dst] = res;
                }
                self.pc += 3;
            }
            Instruction::Jmp(addr) => {
                self.pc = addr;
            }
            Instruction::Halt => {
                return false;
            }
            Instruction::Unknown => {
                println!("Unknown instruction at PC={}", self.pc);
                return false;
            }
        }

        true
    }

    fn run(&mut self) {
        while self.exec() {}
    }
}

fn main() {
    let mut vm = VM::new();

    vm.memory[0..16].copy_from_slice(&[
        0x01, 0x00, 10, // LOAD R0, 10
        0x01, 0x01, 20, // LOAD R1, 20
        0x01, 0x02, 30, // LOAD R1, 20
        0x02, 0x00, 0x01, // ADD R0, R1
        0x04, 0x00, 0x02,   // SUB R0, R1
        0xFF, // HALT
    ]);

    vm.run();

    println!("Result: {}", vm.registers[0]);
}
