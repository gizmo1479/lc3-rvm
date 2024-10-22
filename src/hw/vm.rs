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
            let instruction_bytes: u16 = self.memory[PC_REG as usize];
            // TODO: perform instruction
            self.perform_instruction(instruction_bytes);
        }
    }
}

impl VM {
    fn perform_instruction(&mut self, instruction: u16) {
        let opcode: Option<OpCode> = OpCode::from_u16(&instruction);
        match opcode {
            Some(OpCode::OpAdd) => self.add(instruction),
            _ => (),
        }
    }

    fn add(&mut self, full_instruction: u16) {
        todo!()
    }

    fn and(&mut self, full_instruction: u16) {
        todo!()
    }

    fn br(&mut self, full_instruction: u16) {
        todo!()
    }
}
