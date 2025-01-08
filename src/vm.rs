use crate::interpreter::Opcode;

pub struct VM {
  registers: [i32; 32],
  pc: usize,
  program: Vec<u8>
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
        }
    }
    
    pub fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc+=1; 
        return result;
    }
    
    pub fn next_16_bits(&mut self) -> u16 {
        let result = (self.program[self.pc] as u16) << 8 | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;

    }

    pub fn run(&mut self) {
        let mut to_continue = true;
        while to_continue {
            to_continue = self.run_opcode();
        }
    }
    pub fn execute_once(&mut self) {
        self.run_opcode();
    }

    fn run_opcode(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16; 
                self.registers[register] = number as i32;
            },
            Opcode::ADD => {
                let register = self.next_8_bits() as usize;
                let register1 = self.next_8_bits() as usize;
                let register2 = self.next_8_bits() as usize;
                let number = (self.registers[register1] + self.registers[register2]) as u16; 
                self.registers[register] = number as i32;
            },
            Opcode::SUB => {
                let register = self.next_8_bits() as usize;
                let register1 = self.next_8_bits() as usize;
                let register2 = self.next_8_bits() as usize;
                let number = (self.registers[register1] - self.registers[register2]) as u16; 
                self.registers[register] = number as i32;
            },
            Opcode::MUL => {
                let register = self.next_8_bits() as usize;
                let register1 = self.next_8_bits() as usize;
                let register2 = self.next_8_bits() as usize;
                let number = (self.registers[register1] * self.registers[register2]) as u16; 
                self.registers[register] = number as i32;
            },
            Opcode::DIV => {
                let register = self.next_8_bits() as usize;
                let register1 = self.next_8_bits() as usize;
                let register2 = self.next_8_bits() as usize;
                let number = (self.registers[register1] / self.registers[register2]) as u16; 
                self.registers[register] = number as i32;
            },
            Opcode::HLT => {
                println!("HLT encountered");
                return false
            },
            _ => {
                println!("Unknown command");
                return false
            }

        }
        return true
    }


    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }
        
}

mod tests {
    use super::*;
    
    #[test]
    fn create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }
    
    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
    
    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        let test_bytes = vec![1,1,0,10,1,2,0,15,2,3,1,2];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[3], 25);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::new();
        let test_bytes = vec![1,1,0,15,1,2,0,10,3,3,1,2];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[3], 5);
    }


    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::new();
        let test_bytes = vec![1,1,0,10,1,2,0,15,4,3,1,2];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[3], 150);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::new();
        let test_bytes = vec![1,1,0,20,1,2,0,10,5,3,1,2];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[3], 2);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test] 
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1,0,1,244];
        test_vm.run();
        assert_eq!(test_vm.registers[0],500)
    }
}