pub struct Memory {
    memory: [u16; 10000],
    loaded: usize             // how many words are loaded at the beginning of the memory
}

impl Default for Memory {
    fn default() -> Memory {
        Memory { 
            memory: [0; 10000],
            loaded: 0
        }
    }
}

impl Memory {
    pub fn load(&mut self, words: &[u16]) {
        let len = words.len();
        for n in self.loaded..(self.loaded+len) {
            self.memory[n] = words[n - self.loaded]; 
        }
        self.loaded += len;
    }

    pub fn get(&self, pos: usize) -> u16 {
        self.memory[pos]
    }

    pub fn is_readable(&self, pos: usize) -> bool {
        pos < self.loaded
    }
}

