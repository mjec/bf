use std::iter;
use std::usize;

#[derive(Debug, Clone)]
pub struct HalfTape {
    memory: Vec<u8>,
    pointer: usize,
}

impl HalfTape {
    pub fn new() -> HalfTape {
        HalfTape {
            memory: vec![0],
            pointer: 0,
        }
    }

    pub fn get_pointer(&self) -> usize {
        self.pointer
    }

    pub fn move_pointer_to(&mut self, to: usize) {
        self.pointer = to;
        self.expand_memory_if_required();
    }

    pub fn move_pointer_by(&mut self, by: i128) {
        if self.pointer as i128 + by < 0 {
            panic!("Can't move half tape past zero! This indicates a logic bug. Pointer: {}, move: {}.", self.pointer, by)
        }

        if self.pointer as i128 + by > usize::MAX as i128 {
            panic!(
                "Out of memory! Tried to allocate more than {} bytes to a half tape.",
                usize::MAX
            )
        }

        self.pointer = (self.pointer as i128 + by) as usize;
        self.expand_memory_if_required();
    }

    pub fn get_value(&mut self) -> u8 {
        *self.get_ref()
    }

    pub fn set_value(&mut self, value: u8) {
        *self.get_mut_ref() = value;
    }

    pub fn increment_value(&mut self, by: u8) {
        *self.get_mut_ref() = self.get_value().wrapping_add(by)
    }

    pub fn decrement_value(&mut self, by: u8) {
        *self.get_mut_ref() = self.get_value().wrapping_sub(by)
    }

    fn get_ref(&mut self) -> &u8 {
        &self.memory[self.pointer]
    }

    fn get_mut_ref(&mut self) -> &mut u8 {
        &mut self.memory[self.pointer]
    }

    fn expand_memory_if_required(&mut self) {
        let bytes_required = self.pointer + 1;
        if let Some(missing_bytes) = bytes_required.checked_sub(self.memory.len()) {
            self.memory.extend(iter::repeat(0).take(missing_bytes))
        }
    }
}
