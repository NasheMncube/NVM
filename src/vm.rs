pub enum INSTR_SET {
    PUSHi, // PUSH argument to stack
    PUSHA, // PUSH value in register A to stack
    PUSHB, // ...................... B to stack
    PUSHX, // ...................... X to stack
    PUSHY, // ...................... Y to stack
    POPA,  // POP value on stack and store in A
    POPB,  // ............................... B
    POPX,  // ............................... X
    POPY,  // ............................... Y
    ADDA,  // ADD argument to register A 
    ADDB,  // ........................ B
    ADDX,  // ........................ X
    ADDY,  // ........................ Y
    SUBA,  // SUB argument from register A
    SUBB,  // .......................... B
    SUBX,  // .......................... X
    SUBY,  // .......................... Y
    BRZ,   // Branch if CC register set FLAG::ZERO
    BRN,   // Branch if CC register set to FLAG::NEGATIVE
    BRO,   // Branch if CC register set to FLAG::OVERFLOW
    SETA,  // SET register A to argument
    SETB,  // SET register B to argument
    SETX,  // SET regiseter X to argument
    SETY,  // SET register Y to argument
    HALT,  // HALT execution of VM
}

pub struct VM {
    A: u8,
    B: u8,
    X: u8,
    Y: u8, 
    SP: u8,
    PC: u8,
    CC: FLAG,

    mem: [u8; 256],
}

pub enum FLAG {
    OVERFLOW
    ZERO,
    NEGATIVE,
    CARRY,
}

