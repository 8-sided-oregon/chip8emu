mod instructions {
    use crate::memory::EmuState;

    pub trait Instruction<E> {
        fn new() -> Self;
        fn extract_data(&mut self, ins: u16);
        fn execute(&self, state: &mut EmuState) -> Result<(), E>;
    }

    pub struct ClearScreen {
    }

    impl Instruction<()> for ClearScreen {
        fn new() -> Self {
            ClearScreen {  }
        }

        fn extract_data(&mut self, ins: u16) {}

        fn execute(&self, state: &mut EmuState) -> Result<(), ()> {
            state.screen.clear();

            Ok(())
        }
    }

    pub struct Return {
    }

    impl Instruction<&'static str> for Return {
        fn new() -> Return {
            Return { }
        }

        fn extract_data(&mut self, ins: u16) {}

        fn execute(&self, state: &mut EmuState) -> Result<(), &'static str> {
            let return_address = match state.stack.pop() {
                Some(v) => v,
                None => return Err("Stack is empty"),
            };

            state.registers.instruction_pointer  = return_address;

            Ok(())
        }
    }

    pub struct Jump {
        address: u16,
    }

    impl Instruction<()> for Jump {
        fn new() -> Jump {
            Jump { address: 0 }
        }

        fn extract_data(&mut self, ins: u16) {
            self.address = ins & 0x0fff;
        }

        fn execute(&self, state: &mut EmuState) -> Result<(), ()> {
            state.registers.instruction_pointer = self.address;

            Ok(())
        }
    }

    pub struct Call {
        address: u16,
    }

    impl Instruction<()> for Call {
        fn new() -> Call {
            Call { address: 0 }
        }

        fn extract_data(&mut self, ins: u16) {
            self.address = ins & 0x0fff;
        }

        fn execute(&self, state: &mut EmuState) -> Result<(), ()> {
            state.stack.push(self.address);
            state.registers.instruction_pointer = self.address;

            Ok(())
        }
    }

    pub struct CondEq {
        register_num: u8,
        compare: u8,
    }

    impl Instruction<&'static str> for CondEq {
        fn new() -> CondEq {
            CondEq { register_num: 0, compare: 0 }
        }

        fn extract_data(&mut self, ins: u16) {
            self.register_num = ((ins & 0x0f00) >> 8) as u8;
            self.compare = (ins & 0xff) as u8;
        }

        fn execute(&self, state: &mut EmuState) -> Result<(), &'static str> {
            if state.registers.variables[self.compare as usize] == self.compare {
                state.registers.instruction_pointer += 1;
            }

            Ok(())
        }
    }

    pub struct CondNeq {
        register_num: u8,
        compare: u8,
    }

    impl Instruction<()> for CondNeq {
        fn new() -> CondNeq {
            CondNeq { register_num: 0, compare: 0 }
        }

        fn extract_data(&mut self, ins: u16) {
            self.register_num = ((ins & 0x0f00) >> 8) as u8;
            self.compare = (ins & 0xff) as u8;
        }

        fn execute(&self, state: &mut EmuState) -> Result<(), ()> {
            if state.registers.variables[self.compare as usize] != self.compare {
                state.registers.instruction_pointer += 1;
            }

            Ok(())
        }
    }

    pub struct CondVEq {
        register_num1: u8,
        register_num2: u8,
    }

    impl Instruction<()> for CondVEq {
        fn new() -> CondVEq {
            CondVEq { register_num1: 0, register_num2: 0 }
        }

        fn extract_data(&mut self, ins: u16) {
            self.register_num1 = ((ins & 0x0f00) >> 8) as u8;
            self.register_num2 = ((ins & 0xf0) >> 4) as u8;
        }

        fn execute(&self, state: &mut EmuState) -> Result<(), ()> {
            let variables = &state.registers.variables;
            
            if variables[self.register_num1 as usize] == variables[self.register_num2 as usize] {
                state.registers.instruction_pointer += 1;
            }

            Ok(())
        }
    }

    pub struct SetV {
        register_num: u8,
        value: u8,
    }

    impl Instruction<()> for SetV {
        fn new() -> SetV {
            SetV { register_num: 0, value: 0 }
        }

        fn extract_data(&mut self, ins: u16) {
            self.register_num = ((ins & 0x0f00) >> 8) as u8;
            self.value = ins as u8;
        }

        fn execute(&self, state: &mut EmuState) -> Result<(), ()> {
            state.registers.variables[self.register_num as usize] = self.value;

            Ok(())
        }
    }

    pub struct AddV {
        register_num: u8,
        value: u8,
    }

    impl Instruction<()> for AddV {
        fn new() -> AddV {
            AddV { register_num: 0, value: 0 }
        }

        fn extract_data(&mut self, ins: u16) {
            self.register_num = ((ins & 0x0f00) >> 8) as u8;
            self.value = ins as u8;
        }

        fn execute(&self, state: &mut EmuState) -> Result<(), ()> {
            state.registers.variables[self.register_num as usize] += self.value;

            Ok(())
        }
    }

    fn extract_reg_nums(ins: u16) -> (u8, u8) {
        (((ins & 0x0f00) >> 8) as u8, ((ins & 0xf0) >> 4) as u8)
    }

    pub struct VSetV {
        register_num1: u8,
        register_num2: u8,
    }

    impl Instruction<()> for VSetV {
        fn new() -> VSetV {
            VSetV { register_num1: 0, register_num2: 0 }
        }

        fn extract_data(&mut self, ins: u16) {
            (self.register_num1, self.register_num2) = extract_reg_nums(ins)
        }

        fn execute(&self, state: &mut EmuState) -> Result<(), ()> {
            state.registers.variables[self.register_num1 as usize] =
                state.registers.variables[self.register_num2 as usize];

            Ok(())
        }
    }

    pub struct VOrV {
        register_num1: u8,
        register_num2: u8,
    }

    impl Instruction<()> for VOrV {
        fn new() -> VOrV {
            VOrV { register_num1: 0, register_num2: 0 }
        }

        fn extract_data(&mut self, ins: u16) {
            (self.register_num1, self.register_num2) = extract_reg_nums(ins)
        }

        fn execute(&self, state: &mut EmuState) -> Result<(), ()> {
            state.registers.variables[self.register_num1 as usize] |=
                state.registers.variables[self.register_num2 as usize];

            Ok(())
        }
    }

    pub struct VAndV {
        register_num1: u8,
        register_num2: u8,
    }

    impl Instruction<()> for VAndV {
        fn new() -> VAndV {
            VAndV { register_num1: 0, register_num2: 0 }
        }

        fn extract_data(&mut self, ins: u16) {
            (self.register_num1, self.register_num2) = extract_reg_nums(ins)
        }

        fn execute(&self, state: &mut EmuState) -> Result<(), ()> {
            state.registers.variables[self.register_num1 as usize] &=
                state.registers.variables[self.register_num2 as usize];

            Ok(())
        }
    }

    pub struct VXorV {
        register_num1: u8,
        register_num2: u8,
    }

    impl Instruction<()> for VXorV {
        fn new() -> VXorV {
            VXorV { register_num1: 0, register_num2: 0 }
        }

        fn extract_data(&mut self, ins: u16) {
            (self.register_num1, self.register_num2) = extract_reg_nums(ins)
        }

        fn execute(&self, state: &mut EmuState) -> Result<(), ()> {
            state.registers.variables[self.register_num1 as usize] ^=
                state.registers.variables[self.register_num2 as usize];

            Ok(())
        }
    }

    pub struct VAddV {
        register_num1: u8,
        register_num2: u8,
    }

    impl Instruction<()> for VAddV {
        fn new() -> VAddV {
            VAddV { register_num1: 0, register_num2: 0 }
        }

        fn extract_data(&mut self, ins: u16) {
            (self.register_num1, self.register_num2) = extract_reg_nums(ins)
        }

        fn execute(&self, state: &mut EmuState) -> Result<(), ()> {
            if state.registers.variables[self.reg]

            state.registers.variables[self.register_num1 as usize] +=
                state.registers.variables[self.register_num2 as usize];
            
            Ok(())
        }
    }

    pub struct VMinV {
        register_num1: u8,
        register_num2: u8,
    }

    impl Instruction<()> for VAndV {
        fn new() -> VAndV {
            VAndV { register_num1: 0, register_num2: 0 }
        }

        fn extract_data(&mut self, ins: u16) {
            (self.register_num1, self.register_num2) = extract_reg_nums(ins)
        }

        fn execute(&self, state: &mut EmuState) -> Result<(), ()> {
            state.registers.variables[self.register_num1 as usize] &=
                state.registers.variables[self.register_num2 as usize];

            Ok(())
        }
    }
}