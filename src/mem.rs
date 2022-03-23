pub mod memory {
    use std::io;
    use std::fs;
    use std::path::Path;

    pub mod video {
        use std::io;

        const SCREEN_WIDTH: usize = 64;
        const SCREEN_HEIGHT: usize = 32;
    
        pub struct SpriteSize { pub width: u8, pub height: u8 }
        pub struct SpritePos { pub x: u8, pub y: u8 }

        pub struct VideoRam {
            matrix: [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH],
        }

        impl VideoRam {
            pub fn new() -> VideoRam {
                VideoRam {
                    matrix: [[false; SCREEN_HEIGHT]; SCREEN_WIDTH],
                }
            }

            pub fn get_matrx(&self) -> &[[bool; SCREEN_HEIGHT]; SCREEN_WIDTH] {
                &self.matrix
            }

            pub fn clear(&mut self) {
                self.matrix = [[false; SCREEN_HEIGHT]; SCREEN_WIDTH];
            }

            pub fn draw(&mut self, pos: SpritePos, size: SpriteSize) -> Result<bool, io::Error> {
                let mut pixels_were_there = false;

                if pos.y + size.height > SCREEN_HEIGHT as u8 || 
                    pos.x + size.width > SCREEN_WIDTH as u8 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData, 
                        "Index out of bounds"
                    ));
                }

                for x in pos.x..(pos.x+size.width) {
                    for y in pos.y..(pos.y+size.height) {
                        let pixel = &mut self.matrix[x as usize][y as usize];

                        if *pixel {
                            pixels_were_there = true;
                        }

                        *pixel ^= true;
                    }
                }

                Ok(pixels_were_there)
            }

        }
    }

    pub struct Registers {
        pub variables: [u8; 15],
        pub instruction_pointer: u16,
        pub carry_flag: bool,
    }

    impl Registers {
        pub fn new() -> Registers {
            Registers { 
                variables: [0; 15], 
                carry_flag: 0, 
                instruction_pointer: 0
            }
        }

        pub fn carry_add(&mut self, lhs: u8, rhs: u8, state: &mut EmuState) -> u8 {
            let sum = (lhs as u16) + (rhs as u16);

            if sum > 255 {
                state.registers.carry_flag = true;
            }

            sum as u8
        }

        pub fn reset(&mut self) {
            for &mut reg in self.variables {
                reg = 0;
            }

            self.instruction_pointer = 0;
            self.carry_flag = 0;
        }
    }

    pub struct EmuState {
        pub screen: video::VideoRam,
        pub ram: Vec<u8>,
        pub stack: Vec<u16>,
        pub registers: Registers,

        pub loaded_rom: Vec<u8>,
    }

    impl EmuState {
        pub fn new(screen: video::VideoRam, ram: Vec<u8>, 
            stack: Vec<u8>, registers: Registers) -> EmuState {
            EmuState { 
                screen: video::VideoRam::new(), 
                ram: Vec::new(), 
                stack: Vec::new(), 
                registers: Registers::new(), 
                loaded_rom: vec![],
            }
        }

        pub fn from(screen: video::VideoRam, ram: Vec<u8>, 
            stack: Vec<u8>, registers: Registers, path: &Path) -> EmuState {
            let state = EmuState::new(screen, ram, stack, registers);
            state.load_rom(path).unwrap();

            state
        }

        pub fn load_rom(&mut self, path: &Path) -> Result<(), io::Error> {
            self.loaded_rom = fs::read(path)?;
            self.registers.instruction_pointer = 512;
            
            Ok(())
        }

        pub fn reset(&mut self) {
            self.registers.reset();
        }
    }
}