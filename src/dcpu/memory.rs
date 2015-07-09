pub struct Memory {
    memory: [u16; 10000]
}

impl Default for Memory {
    fn default() -> Memory { 
        Memory { memory: [0; 10000] }
    }
}

