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
        None
    }
}
