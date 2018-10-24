use either::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Instr {
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

#[derive(Debug, PartialEq, Clone)]
pub enum Flag {
    OVERFLOW,
    ZERO,
    NEGATIVE,
    CARRY,
    DEFAULT,
}

#[derive(Clone)]
pub struct VM {
    A: i8,
    B: i8,
    X: i8,
    Y: i8, 
    SP: u8,
    CC: Flag,
    PC: Option<Instr>,
    program: Vec<Either<i8, Instr>>,

    mem: [u8; 256],
}


impl VM {
    fn new(program: Vec<Either<i8, Instr>>) -> VM {
        VM {
           A: 0,
           B: 0,
           X: 0,
           Y: 0,
           SP: 255,
           CC: Flag::DEFAULT,
           PC: None,
           program,
           mem: [0; 256],
        }
    }

    fn execute(&mut self) {
        loop {
            match self.program.pop() {
                Some(Right(instr)) => {
                    self.PC = Some(instr.clone());
                    match instr {
                        Instr::ADDA 
                        | Instr::ADDB 
                        | Instr::ADDX
                        | Instr::ADDY => self.handle_add(),
                        Instr::SUBA
                        | Instr::SUBB
                        | Instr::SUBX
                        | Instr::SUBY => self.handle_sub(),

                        Instr::HALT   => break,
                        _             => break,
                    }
                },
                None => { self.PC = None; break; },
                _ => (),
            }

        }
    }

    fn handle_add(&mut self) {
        let arg = match self.program.pop().unwrap() {
            Left(x) => x,
            _       => 0,
        };
        let reg_value = match self.PC {
            Some(Instr::ADDA) => self.A,
            Some(Instr::ADDB) => self.B,
            Some(Instr::ADDX) => self.X,
            Some(Instr::ADDY) => self.Y,
            _           => 0,
        };

        let next_reg_value = {
            if 127 - reg_value < arg{ 
                self.CC = Flag::OVERFLOW;
                reg_value 
            } else if (reg_value + arg) == 0 { 
                self.CC = Flag::ZERO; 
                0
            } else {
                self.CC = Flag::DEFAULT;
                arg + reg_value
            }
        };

        match self.PC {
            Some(Instr::ADDA) => { self.A = next_reg_value; },
            Some(Instr::ADDB) => { self.B = next_reg_value; },
            Some(Instr::ADDX) => { self.X = next_reg_value; },
            Some(Instr::ADDY) => { self.Y = next_reg_value; },
            _                 => ()
        }
    }

    fn handle_sub(&mut self) {
        let arg = match self.program.pop().unwrap() {
            Left(x) => x,
            _       => 0,
        };

        let reg_value = match self.PC {
            Some(Instr::SUBA) => self.A,
            Some(Instr::SUBB) => self.B,
            Some(Instr::SUBX) => self.X,
            Some(Instr::SUBY) => self.Y,
            _ => 0,
        };

        let next_reg_value = {
            if reg_value < arg {
                self.CC = Flag::OVERFLOW;
                reg_value
            } else if reg_value - arg == 0 {
                self.CC = Flag::ZERO;
                0
            } else {
                self.CC = Flag::DEFAULT;
                reg_value - arg
            }
        };

        match self.PC {
            Some(Instr::SUBA) => {self.A = next_reg_value;},
            Some(Instr::SUBB) => {self.B = next_reg_value;},
            Some(Instr::SUBX) => {self.X = next_reg_value;},
            Some(Instr::SUBY) => {self.Y = next_reg_value;},
            _                 => ()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn initialize_new_vm() {
        let program = vec![Right(Instr::HALT)];
        let vm = VM::new(program);
        assert_eq!(vm.A, 0);
        assert_eq!(vm.B, 0);
        assert_eq!(vm.X, 0);
        assert_eq!(vm.Y, 0);
        assert_eq!(vm.SP, 255);
        assert_eq!(vm.CC, Flag::DEFAULT);

        let mut size = 0;
        for x in vm.mem.iter() {
            size += 1;
            assert_eq!(*x, 0);
        }
        assert_eq!(size, 256);
    }

    #[test]
    fn adding_to_registers() {
        let add_to_a = vec![Left(10), Right(Instr::ADDA)];
        let add_to_b = vec![Left(10), Right(Instr::ADDB)];
        let add_to_x = vec![Left(10), Right(Instr::ADDX)];
        let add_to_y = vec![Left(10), Right(Instr::ADDY)];

        let mut vm = VM::new(add_to_a);
        vm.execute();
        assert_eq!(10, vm.A);

        vm = VM::new(add_to_b);
        vm.execute();
        assert_eq!(10, vm.B);

        vm = VM::new(add_to_x);
        vm.execute();
        assert_eq!(10, vm.X);

        vm = VM::new(add_to_y);
        vm.execute();
        assert_eq!(10, vm.Y);
    }

    #[test]
    fn flag_setting_on_addition_to_register() {
        let overflow = vec![Left(127), Right(Instr::ADDA), Left(1), Right(Instr::ADDA)];
        let zero = vec![Left(0), Right(Instr::ADDA)];
        let default = vec![Left(1), Right(Instr::ADDA)];

        let mut vm = VM::new(overflow);
        vm.execute();
        assert_eq!(vm.CC, Flag::OVERFLOW);

        vm = VM::new(zero);
        vm.execute();
        assert_eq!(vm.CC, Flag::ZERO);

        vm = VM::new(default);
        vm.execute();
        assert_eq!(vm.CC, Flag::DEFAULT);
    }

    #[test]
    fn subtracting_from_registers() {
        let sub_from_a = vec![Left(10), Right(Instr::SUBA), Left(42), Right(Instr::ADDA)];
        let sub_from_b = vec![Left(10), Right(Instr::SUBB), Left(42), Right(Instr::ADDB)];
        let sub_from_x = vec![Left(10), Right(Instr::SUBX), Left(42), Right(Instr::ADDX)];
        let sub_from_y = vec![Left(10), Right(Instr::SUBY), Left(42), Right(Instr::ADDY)];

        let mut vm = VM::new(sub_from_a);
        vm.execute();
        assert_eq!(32, vm.A);

        vm = VM::new(sub_from_b);
        vm.execute();
        assert_eq!(32, vm.B);

        vm = VM::new(sub_from_x);
        vm.execute();
        assert_eq!(32, vm.X);

        vm = VM::new(sub_from_y);
        vm.execute();
        assert_eq!(32, vm.Y);
    }
}