mod memory;
mod Instruction;

pub struct Dcpu {
    memory: memory::Memory
}

impl Default for Dcpu {
    fn default() -> Dcpu {
        Dcpu {
            memory: Default::default()
        }
    }
}

impl Dcpu {
    fn process(&self, word: u16) {
    }
}

