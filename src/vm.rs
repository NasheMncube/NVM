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
#[allow(non_snake_case)]
pub struct VM {
    A: u8,
    B: u8,
    X: u8,
    Y: u8, 
    SP: usize,
    CC: Flag,
    PC: Option<Instr>,
    program: Vec<Either<u8, Instr>>,

    mem: [u8; 256],
}


impl VM {
    fn new(program: Vec<Either<u8, Instr>>) -> VM {
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
                        Instr::PUSHi
                        | Instr::PUSHA
                        | Instr::PUSHB
                        | Instr::PUSHX
                        | Instr::PUSHY => self.handle_push(),
                        Instr::POPA
                        | Instr::POPB
                        | Instr::POPX
                        | Instr::POPY  => self.handle_pop(),

                        Instr::HALT   => break,
                        _             => break,
                    }
                },
                None => { self.PC = None; break; },
                _ => (),
            }

        }
    }

    fn handle_push(&mut self) {

        let arg = match self.PC {
            Some(Instr::PUSHA) => self.A,
            Some(Instr::PUSHB) => self.B,
            Some(Instr::PUSHX) => self.X,
            Some(Instr::PUSHY) => self.Y,
            Some(Instr::PUSHi) => 
                match self.program.pop().unwrap(){
                    Left(x) => x,
                    _       => 0,
            }
            _ => 0,
        };

        if self.SP > 0 {
            self.mem[self.SP] = arg;
            self.SP -= 1;
        }
    }

    fn pop(&mut self) -> u8 {
        if self.SP == 255 {
            0
        } else {
            let arg = self.mem[self.SP + 1];
            self.SP += 1;
            arg
        }
    }

    fn handle_pop(&mut self) {
        match self.PC {
            Some(Instr::POPA) => {self.A = self.pop();},
            Some(Instr::POPB) => {self.B = self.pop();},
            Some(Instr::POPX) => {self.X = self.pop();},
            Some(Instr::POPY) => {self.Y = self.pop();},
            None | _ => (),
            
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
            if 255 - reg_value < arg{ 
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
        let overflow = vec![Left(255), Right(Instr::ADDA), Left(1), Right(Instr::ADDA)];
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

    #[test]
    fn setting_flags_on_subtraction() {
        let overflow = vec![Left(10), Right(Instr::SUBA)];
        let zero     = vec![Left(10), Right(Instr::SUBA), Left(10), Right(Instr::ADDA)];

        let mut vm = VM::new(overflow);
        vm.execute();
        assert_eq!(vm.CC, Flag::OVERFLOW);

        vm = VM::new(zero);
        vm.execute();
        assert_eq!(vm.CC, Flag::ZERO);
    }

    #[test]
    fn pushing_to_stack() {
        let push_immediate = vec![Left(10), Right(Instr::PUSHi)];
        let push_from_a = vec![Right(Instr::PUSHA), Left(10), Right(Instr::ADDA)];
        let push_from_b = vec![Right(Instr::PUSHB), Left(10), Right(Instr::ADDB)];
        let push_from_x = vec![Right(Instr::PUSHX), Left(10), Right(Instr::ADDX)];
        let push_from_y = vec![Right(Instr::PUSHY), Left(10), Right(Instr::ADDY)];

        let mut vm = VM::new(push_immediate);
        vm.execute();
        assert_eq!(vm.mem[vm.SP+1], 10);

        vm = VM::new(push_from_a);
        vm.execute();
        assert_eq!(vm.mem[vm.SP + 1], vm.A);

        vm = VM::new(push_from_b);
        vm.execute();
        assert_eq!(vm.mem[vm.SP + 1], vm.B);

        vm = VM::new(push_from_x);
        vm.execute();
        assert_eq!(vm.mem[vm.SP + 1], vm.X);

        vm = VM::new(push_from_y);
        vm.execute();
        assert_eq!(vm.mem[vm.SP + 1], vm.Y);
    }

    #[test]
    fn popping_from_stack() {
        let pop_to_a = vec![Right(Instr::POPA), Left(42), Right(Instr::PUSHi)];
        let pop_to_b = vec![Right(Instr::POPB), Left(32), Right(Instr::PUSHi)];
        let pop_to_x = vec![Right(Instr::POPX), Left(22), Right(Instr::PUSHi)];
        let pop_to_y = vec![Right(Instr::POPY), Left(12), Right(Instr::PUSHi)];

        let mut vm = VM::new(pop_to_a);
        vm.execute();
        assert_eq!(vm.A, 42);

        vm = VM::new(pop_to_b);
        vm.execute();
        assert_eq!(vm.B, 32);

        vm = VM::new(pop_to_x);
        vm.execute();
        assert_eq!(vm.X, 22);

        vm = VM::new(pop_to_y);
        vm.execute();
        assert_eq!(vm.Y, 12);
    }
}