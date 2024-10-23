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
        todo!()
    }
}

pub fn sign_extend(num: u16, bit_count: u8) -> u16 {
    let mut ret = num;
    // if num is negative, need to pad with zeroes
    if ((num >> (bit_count - 1)) & 1) != 0 {
        ret |= 0xffff >> bit_count;
    }
    // if num is positive, it will already be padded with zeroes
    ret
}
