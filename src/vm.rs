#[derive(Debug, PartialEq)]
pub enum InstrSet {
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

#[derive(Debug, PartialEq)]
pub enum Flag {
    OVERFLOW,
    ZERO,
    NEGATIVE,
    CARRY,
    DEFAULT,
}

pub struct VM {
    A: u8,
    B: u8,
    X: u8,
    Y: u8, 
    SP: u8,
    PC: *const InstrSet,
    CC: Flag,

    mem: [u8; 256],
}


impl VM {
    fn new(program: &[InstrSet]) -> VM {
        VM {
           A: 0,
           B: 0,
           X: 0,
           Y: 0,
           SP: 255,
           PC: &program[0],
           CC: Flag::DEFAULT,

           mem: [0; 256],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn initialize_new_VM() {
        let program = [InstrSet::HALT];
        let vm = VM::new(&program);
        assert_eq!(vm.A, 0);
        assert_eq!(vm.B, 0);
        assert_eq!(vm.X, 0);
        assert_eq!(vm.Y, 0);
        assert_eq!(vm.SP, 255);
        assert_eq!(vm.PC, &program[0]);
        assert_eq!(vm.CC, Flag::DEFAULT);

        let mut size = 0;
        for x in vm.mem.iter() {
            size += 1;
            assert_eq!(*x, 0);
        }
        assert_eq!(size, 256);
    }
}