use std::collections::HashMap;

// was considering using an enum but it is too cumbersome to go between
// enums and other types
const NUM_REGISTERS: u8 = 9;
/* LC3 Registers
 * R0 = 0
 * R1
 * R2
 * R3
 * R4
 * R5
 * R6
 * R7
 * PC
 * COND = 9
*/

pub const PC_REG: u8 = 8;
pub const PC_START: u16 = 0x3000;

pub enum ConditionFlag {
    POS = 1,
    NEG = 2,
    ZERO = 4,
}

pub struct Registers {
    pub regs: HashMap<u8, u16>,
}

impl Registers {
    pub fn new() -> Self {
        let mut regs = HashMap::new();
        for r in 0..NUM_REGISTERS {
            regs.insert(r, 0);
        }

        regs.insert(PC_REG, PC_START);
        Registers { regs }
    }

    pub fn update_register(&mut self, register: u8, value: u16) {
        // technically this allows a value to be placed into the cond_reg
        if register >= NUM_REGISTERS {
            panic!("INVALID REGISTER: {:?}", register)
        }

        self.regs.insert(register, value);
    }

    pub fn get_val(&self, register: u8) -> u16 {
        if register >= NUM_REGISTERS {
            panic!("INVALID REGISTER: {:?}", register)
        }

        *self.regs.get(&register).unwrap()
    }

    pub fn update_cond_register(&mut self, register: u8) {
        let val = self.get_val(register);
        match val {
            0 => self.regs.insert(NUM_REGISTERS, ConditionFlag::ZERO as u16),
            x if (x >> 15) != 0 => self.regs.insert(NUM_REGISTERS, ConditionFlag::NEG as u16),
            _ => self.regs.insert(NUM_REGISTERS, ConditionFlag::POS as u16),
        };
    }
}
