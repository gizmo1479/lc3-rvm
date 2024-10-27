pub enum OpCode {
    OpBr = 0, /* branch */
    OpAdd,    /* add  */
    OpLd,     /* load */
    OpSt,     /* store */
    OpJsr,    /* jump reg */
    OpAnd,    /* bitwise and */
    OpLdr,    /* load reg */
    OpStr,    /* store reg */
    OpRti,    /* unused */
    OpNot,    /* bitwise not */
    OpLdi,    /* load indirect */
    OpSti,    /* store indirect */
    OpJmp,    /* jump */
    OpRes,    /* reserved (unused) */
    OpLea,    /* load effective address */
    OpTrap,   /* execute trap */
}

impl OpCode {
    pub fn from_u16(instruction: &u16) -> Option<Self> {
        // instruction opcode is bits 15-12
        match instruction >> 12 {
            0 => Some(Self::OpBr),
            1 => Some(Self::OpAdd),
            2 => Some(Self::OpLd),
            3 => Some(Self::OpSt),
            4 => Some(Self::OpJsr),
            5 => Some(Self::OpAnd),
            6 => Some(Self::OpLdr),
            7 => Some(Self::OpStr),
            8 => Some(Self::OpRti),
            9 => Some(Self::OpNot),
            10 => Some(Self::OpLdi),
            11 => Some(Self::OpSti),
            12 => Some(Self::OpJmp),
            13 => Some(Self::OpRes),
            14 => Some(Self::OpLea),
            15 => Some(Self::OpTrap),
            _ => None,
        }
    }
}

pub fn sign_extend(num: u16, bit_count: u8) -> u16 {
    let mut ret: u16 = num;
    // if num is negative, need to pad with zeroes
    println!("NUM SHIFTED: {:?}", num >> (bit_count - 1));
    if ((num >> (bit_count - 1)) & 1) != 0 {
        println!("OLD NUM BIN: {:#b}", ret);
        ret |= 0xffff << bit_count;
        println!("NEW NUM BIN: {:#b}", ret);
    }
    // if num is positive, it will already be padded with zeroes
    ret
}
