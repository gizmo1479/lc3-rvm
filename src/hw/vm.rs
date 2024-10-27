use std::io;
use std::io::Write;
use std::process::exit;

use crate::hw::register;

use super::instruction::sign_extend;
use super::instruction::OpCode;
use super::register::COND_REG;
use super::register::PC_REG;

const MEMORY_MAX: u16 = u16::MAX;
pub struct VM {
    pub memory: [u16; MEMORY_MAX as usize],
    pub registers: register::Registers,
}

impl VM {
    pub fn new() -> Self {
        println!("INITTTTT");
        VM {
            memory: [0; MEMORY_MAX as usize],
            registers: register::Registers::new(),
        }
    }

    // TODO: ideally returns a Result and checks index
    pub fn write_memory(&mut self, addr_to_write: usize, value: u16) {
        self.memory[addr_to_write] = value;
    }

    pub fn read_memory(&self, addr_to_read: usize) -> Option<u16> {
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
        println!("ADD INSTRUCTION");
        let dest_reg: u8 = ((full_instruction >> 9) & 0x7) as u8;
        let source_reg_1: u8 = ((full_instruction >> 6) & 0x7) as u8;

        // check if in immediate or register mode
        if (full_instruction >> 5) & 0x1 == 1 {
            let imm5 = (full_instruction & 0x1f) as u16;
            println!("IMM5: {:?}", imm5);
            println!("SIGNEXT IMM5: {:?}", sign_extend(imm5, 5));

            // second source operand obtained by sign-extending imm5
            let val: u16 = self.registers.get_val(source_reg_1) + sign_extend(imm5, 5);

            // update register
            self.registers.update_register(dest_reg, val);
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

    // AND instruction layout
    // 15 - 12: 0101, 11-9: DR, 8-6: SR1, 5-3: 0, 2-0: SR2
    // 15 - 12: 0101, 11-9: DR, 8-6: SR1, 5: 1, 4-0: imm5
    fn and(&mut self, full_instruction: u16) {
        let dest_reg: u8 = ((full_instruction >> 9) & 0x7) as u8;
        let source_reg_1: u8 = ((full_instruction >> 6) & 0x7) as u8;

        // check if in immediate or register mode
        if (full_instruction >> 5) & 0x1 == 1 {
            let imm5 = (full_instruction & 0x1f) as u16;

            // second source operand obtained by sign-extending imm5
            let val: u16 = self.registers.get_val(source_reg_1) & sign_extend(imm5, 5);

            // update register
            self.registers.update_register(dest_reg, val);
        } else {
            let source_reg_2: u8 = (full_instruction & 0x7) as u8;

            let val: u16 =
                self.registers.get_val(source_reg_1) & self.registers.get_val(source_reg_2);

            // update register
            self.registers.update_register(dest_reg, val);
        }

        // AND sets condition register flags
        self.registers.update_cond_register(dest_reg);
    }

    // BR instruction layout
    // 15 - 12: 0000, 11: n, 10: z, 9: p, 8-0: PC offset9
    // For 11-9: if bit set is set then test cond, if any cond code is set
    // branch to location specifie dby adding sign-extended PCoffset9 and PC
    fn br(&mut self, full_instruction: u16) {
        let conds = (full_instruction >> 9) & 0x7;
        if (conds & self.registers.get_val(COND_REG)) != 0 {
            let pcoff = sign_extend(full_instruction & 0x1ff, 9);
            let new_pc = self.registers.get_val(PC_REG) + pcoff;
            self.registers.update_register(PC_REG, new_pc)
        }
    }

    // JMP
    // 15-12: 1100, 11-9: 000, 8-6: BaseR(PC)
    // RET, 8-6: 111, if this then jmp to R7
    fn jmp(&mut self, full_instruction: u16) {
        let to_jmp = (full_instruction >> 6) & 0x7;
        // 111 -> 7, which corresponds to R7
        self.registers
            .update_register(PC_REG, self.registers.get_val(to_jmp as u8))
    }

    // JSR (Jump Sub routine)
    // 15-12: 0100, 11: 1, 10-0: PCoffset11
    // JSRR
    // 15-12: 0100, 11-9: 000, 8-6: BaseR, 5-0: 0
    fn jsr(&mut self, full_instruction: u16) {
        // save PC in R7
        self.registers
            .update_register(7, self.registers.get_val(PC_REG));
        if ((full_instruction >> 11) & 1) == 1 {
            // JSR - jump to PC + sign-extend(bits 10-0);
            let new_pc = self.registers.get_val(PC_REG) + sign_extend(full_instruction & 0x7FF, 11);
            self.registers.update_register(PC_REG, new_pc);
        } else {
            // JSRR - jump to base reg
            self.registers.update_register(
                PC_REG,
                self.registers
                    .get_val(((full_instruction >> 6) & 0x7) as u8),
            );
        }
    }

    // LD (Load)
    // 15-12: 0010, 11-9: DR, 8-0: pcoffset9
    // Contents of memory loaded into DR and cond codes are set
    // Address of memory is sign_extend(Pcoffset9) + 16
    fn ld(&mut self, full_instruction: u16) {
        // add as usize to avoid overflow, so we can detect if mem loc is out of bounds
        let mem_addr = sign_extend(full_instruction & 0x1ff, 9) as usize
            + self.registers.get_val(PC_REG) as usize;
        let dr: u8 = ((full_instruction >> 9) & 0x7) as u8;
        self.registers
            .update_register(dr, self.read_memory(mem_addr).unwrap());
        self.registers.update_cond_register(dr)
    }

    // LDI (Load Indirect)
    // 15-12: 1010, 11-9: DR, 8-0: PCOffset9
    // DR = mem[mem[PC + sign_ext(PCOffset9)]]
    fn ldi(&mut self, full_instruction: u16) {
        let mem_addr_1 = sign_extend(full_instruction & 0x1ff, 9) as usize
            + self.registers.get_val(PC_REG) as usize;
        let mem_addr_2 = self.read_memory(mem_addr_1).unwrap() as usize;
        let dr = ((full_instruction >> 9) & 0x7) as u8;
        self.registers
            .update_register(dr, self.read_memory(mem_addr_2).unwrap());
        self.registers.update_cond_register(dr)
    }

    // LDR (Load Base+Offset)
    // 15-12: 0110, 11-9: DR, 8-6: BaseR, 5-0: Offset6
    // DR = mem[BaseR + sign_ext(Offset6)], set cond codes
    fn ldr(&mut self, full_instruction: u16) {
        let base_r = ((full_instruction >> 6) & 0x7) as u8;
        // add as usize to avoid overflow, so we can detect if mem loc is out of bounds
        let mem_addr = sign_extend(full_instruction & 0x3f, 6) as usize
            + self.registers.get_val(base_r) as usize;
        let dr: u8 = ((full_instruction >> 9) & 0x7) as u8;
        self.registers
            .update_register(dr, self.read_memory(mem_addr).unwrap());
        self.registers.update_cond_register(dr)
    }

    // LEA (Load Effective Address)
    // 15-12: 1110, 11-9: DR, 8-0: PCoffset9
    // DR = PC + sign_ext(PCOffset9), set cond codes
    fn lea(&mut self, full_instruction: u16) {
        // TODO: what do we do about out of bounds here?
        let new_addr = sign_extend(full_instruction & 0x1ff, 9) + self.registers.get_val(PC_REG);
        let dr: u8 = ((full_instruction >> 9) & 0x7) as u8;
        self.registers.update_register(dr, new_addr);
        self.registers.update_cond_register(dr)
    }

    // NOT
    // 15-12: 1001, 11-9: dr, 8-6: SR, 5-0: 1
    // bitwise complement as 2's complement int, set cond codes
    fn not(&mut self, full_instruction: u16) {
        let dr: u8 = ((full_instruction >> 9) & 0x7) as u8;
        let sr: u8 = ((full_instruction >> 6) & 0x7) as u8;
        self.registers
            .update_register(dr, !(self.registers.get_val(sr)));
        self.registers.update_cond_register(dr)
    }

    fn res(&mut self, _full_instruction: u16) {
        panic!("ERROR: unused opcode for RES")
    }

    // RTI command is only available for a processor "Supervisor mode"
    fn rti(&mut self, _full_instruction: u16) {
        panic!("ERROR: must be in Supervisor mode for RTI")
    }

    // ST (Store)
    // 15-12: 0011, 11-9: SR, 8-0: PCOffset9
    // mem[PC + sign_ext(PCOffset9)] = SR
    fn st(&mut self, full_instruction: u16) {
        let sr: u8 = ((full_instruction >> 9) & 0x7) as u8;
        // add as usize to avoid overflow, so we can detect if mem loc is out of bounds
        let new_addr = sign_extend(full_instruction & 0x1ff, 9) as usize
            + self.registers.get_val(PC_REG) as usize;
        self.write_memory(new_addr, self.registers.get_val(sr))
    }

    // STI (Store Indirect)
    // 15-12: 1011, 11-9: SR, 8-0: PCOffset9
    // mem[mem[PC + sign_ext(PCOffset9)]] = SR;
    fn sti(&mut self, full_instruction: u16) {
        let sr: u8 = ((full_instruction >> 9) & 0x7) as u8;
        // add as usize to avoid overflow, so we can detect if mem loc is out of bounds
        let mem_addr_1 = sign_extend(full_instruction & 0x1ff, 9) as usize
            + self.registers.get_val(PC_REG) as usize;
        self.write_memory(
            self.read_memory(mem_addr_1).unwrap() as usize,
            self.registers.get_val(sr),
        )
    }

    // STR (Store Base + Offset)
    // 15-12: 1011, 11-9: SR, 8-6: BaseR, 5-0: Offset6
    // mem[BaseR + sign_ext(Offset6)] = SR;
    fn str(&mut self, full_instruction: u16) {
        let sr: u8 = ((full_instruction >> 9) & 0x7) as u8;
        let base_r: u8 = ((full_instruction >> 6) & 0x7) as u8;
        // add as usize to avoid overflow, so we can detect if mem loc is out of bounds
        let mem_addr = sign_extend(full_instruction & 0x3f, 6) as usize
            + self.registers.get_val(base_r) as usize;
        self.write_memory(mem_addr, self.registers.get_val(sr))
    }

    // TRAP (System Call)
    // 15-12: 1111, 11-8: 0000, 7-0: trapvect8
    // Mem locations x0000 -> 0x00FF are available to contain
    // starting addrs for system calls specified by their trap vectors
    fn trap(&mut self, full_instruction: u16) {
        // save PC
        self.registers
            .update_register(7, self.registers.get_val(PC_REG));
        let mem_loc = full_instruction & 0xFF;
        match mem_loc {
            // GETC
            0x20 => todo!(),
            // OUT
            0x21 => todo!(),
            // PUTS
            0x22 => todo!(),
            // IN
            0x23 => todo!(),
            // PUTSP
            0x24 => todo!(),
            // HALT
            0x25 => {
                println!("HALT detected");
                io::stdout().flush().expect("Failed to flush STDOUT");
                exit(1);
            }
            _ => panic!("ERROR: TRAP NOT FOUND"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hw::register::{ConditionFlag, PC_START};

    #[test]
    fn test_add() {
        let mut vm = VM::new();

        // add 0 to 0: COND_REG should have Zero set
        vm.add(0b0001000000000000);
        assert_eq!(vm.registers.get_val(0), 0);
        assert_eq!(vm.registers.get_val(COND_REG), ConditionFlag::ZERO as u16);

        // ADD R2 R3 31
        // instr: 0b0001_010_011_1_11111
        vm.add(0b0001010011111111);
        // sign_ext(31) => 65535
        assert_eq!(vm.registers.get_val(2), 65535);
        // COND_REG negative as "11111" is negative in signed two's complemen
        assert_eq!(vm.registers.get_val(COND_REG), ConditionFlag::NEG as u16);

        // ADD R0 R2 R4
        // instr: 0b0001_000_010_000_100
        vm.add(0b0001000010000100);
        assert_eq!(vm.registers.get_val(0), 65535);
    }

    #[test]
    fn test_and() {
        let mut vm = VM::new();

        // ADD R0 R0 1
        vm.add(0b0001000000100001);
        assert_eq!(vm.registers.get_val(0), 1);
        // ADD R1 R1 2
        vm.add(0b0001001001100011);
        assert_eq!(vm.registers.get_val(1), 3);

        // AND R0 R0 R1
        // instr: 0b0101_000_000_000_001
        vm.and(0b0101000000000001);
        assert_eq!(vm.registers.get_val(0), 1);
        assert_eq!(vm.registers.get_val(COND_REG), ConditionFlag::POS as u16);

        // AND R0 R0 0
        // instr: 0b0101_000_000_1_00000
        vm.and(0b0101000000100000);
        assert_eq!(vm.registers.get_val(0), 0);
        assert_eq!(vm.registers.get_val(COND_REG), ConditionFlag::ZERO as u16);
    }

    #[test]
    fn test_br() {
        let mut vm = VM::new();

        // ADD R0 R0 1 - set P cond flag
        vm.add(0b0001000000100001);
        assert_eq!(vm.registers.get_val(0), 1);

        // BR 255 - doesn't branch
        // instr: 0b0000_1_0_0_011111111
        vm.br(0b0000100011111111);
        assert_eq!(vm.registers.get_val(PC_REG), PC_START);
        // BR 255 - doesn't branch
        // instr: 0b0000_0_1_0_011111111
        vm.br(0b0000010011111111);
        assert_eq!(vm.registers.get_val(PC_REG), PC_START);
        // BR 255 - branches
        // instr: 0b0000_0_0_1_011111111
        vm.br(0b0000001011111111);
        assert_eq!(vm.registers.get_val(PC_REG), 255 + PC_START);
    }

    #[test]
    fn test_jmp() {
        let mut vm = VM::new();
        // JMP R1
        // instr: 0b1100_000_001_000000
        vm.jmp(0b1100000001000000);
        assert_eq!(vm.registers.get_val(PC_REG), 0);

        // ADD R7 R7 3
        vm.add(0b0001111111100011);
        // JMP RET
        // instr: 0b1100_000_111_000000
        vm.jmp(0b1100000111000000);
        assert_eq!(vm.registers.get_val(PC_REG), 3);
    }
}
