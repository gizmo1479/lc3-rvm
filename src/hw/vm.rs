use crate::hw::instruction;
use crate::hw::register;

use super::instruction::OpCode;
use super::register::PC_REG;

const MEMORY_MAX: u16 = u16::MAX;
pub struct VM {
    pub memory: [u16; MEMORY_MAX as usize],
    pub registers: register::Registers,
}

impl VM {
    pub fn new() -> Self {
        VM {
            memory: [0; MEMORY_MAX as usize],
            registers: register::Registers::new(),
        }
    }

    // TODO: ideally returns a Result and checks index
    pub fn write_memory(&mut self, addr_to_write: usize, value: u16) {
        self.memory[addr_to_write] = value;
    }

    pub fn read_memory(&mut self, addr_to_read: usize) -> Option<u16> {
        if addr_to_read >= MEMORY_MAX as usize {
            return None;
        }

        Some(self.memory[addr_to_read])
    }

    // executes the program contained in Memory starting at PC_START
    pub fn execute_program(&mut self) {
        while self.registers.get_val(PC_REG) < MEMORY_MAX {
            // read instruction
            let instruction_bytes: u16 = self.memory[PC_REG as usize];

            // increment PC
            self.registers
                .update_register(PC_REG, self.registers.get_val(PC_REG) + 1);

            // perform instruction
            self.perform_instruction(instruction_bytes);
        }
    }
}

// VM: impl of instruction related code
impl VM {
    fn perform_instruction(&mut self, instruction: u16) {
        let opcode: Option<OpCode> = OpCode::from_u16(&instruction);
        match opcode {
            Some(OpCode::OpAdd) => self.add(instruction),
            Some(OpCode::OpAnd) => self.and(instruction),
            Some(OpCode::OpBr) => self.br(instruction),
            Some(OpCode::OpJmp) => self.jmp(instruction),
            Some(OpCode::OpJsr) => self.jsr(instruction),
            Some(OpCode::OpLd) => self.ld(instruction),
            Some(OpCode::OpLdi) => self.ldi(instruction),
            Some(OpCode::OpLdr) => self.ldr(instruction),
            Some(OpCode::OpLea) => self.lea(instruction),
            Some(OpCode::OpNot) => self.not(instruction),
            Some(OpCode::OpRes) => self.res(instruction),
            Some(OpCode::OpRti) => self.rti(instruction),
            Some(OpCode::OpSt) => self.st(instruction),
            Some(OpCode::OpSti) => self.sti(instruction),
            Some(OpCode::OpStr) => self.str(instruction),
            Some(OpCode::OpTrap) => self.trap(instruction),
            None => (),
        }
    }

    // ADD instruction layout
    // 15 - 12: 0001, 11-9: DR, 8-6: SR1, 5-3: 0, 2-0: SR2
    // 15 - 12: 0001, 11-9: DR, 8-6: SR1, 5: 1, 4-0: imm5
    fn add(&mut self, full_instruction: u16) {
        let dest_reg: u8 = ((full_instruction >> 9) & 0x7) as u8;
        let source_reg_1: u8 = ((full_instruction >> 6) & 0x7) as u8;

        // check if in immediate or register mode
        if (full_instruction >> 5) & 0x1 == 1 {
            todo!()
        } else {
            let source_reg_2: u8 = (full_instruction & 0x7) as u8;

            // TODO: is overflow an issue?
            let val: u16 =
                self.registers.get_val(source_reg_1) + self.registers.get_val(source_reg_2);

            // update register
            self.registers.update_register(dest_reg, val);
        }

        // ADD sets condition register flags
        self.registers.update_cond_register(dest_reg);
    }

    fn and(&mut self, full_instruction: u16) {
        todo!()
    }

    fn br(&mut self, full_instruction: u16) {
        todo!()
    }

    fn jmp(&mut self, full_instruction: u16) {
        todo!()
    }

    fn jsr(&mut self, full_instruction: u16) {
        todo!()
    }

    fn ld(&mut self, full_instruction: u16) {
        todo!()
    }

    fn ldi(&mut self, full_instruction: u16) {
        todo!()
    }

    fn ldr(&mut self, full_instruction: u16) {
        todo!()
    }

    fn lea(&mut self, full_instruction: u16) {
        todo!()
    }

    fn not(&mut self, full_instruction: u16) {
        todo!()
    }

    fn res(&mut self, full_instruction: u16) {
        todo!()
    }

    fn rti(&mut self, full_instruction: u16) {
        todo!()
    }

    fn st(&mut self, full_instruction: u16) {
        todo!()
    }

    fn sti(&mut self, full_instruction: u16) {
        todo!()
    }

    fn str(&mut self, full_instruction: u16) {
        todo!()
    }

    fn trap(&mut self, full_instruction: u16) {
        todo!()
    }
}
