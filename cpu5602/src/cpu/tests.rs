use std::ops::{Index, IndexMut};
use crate::consts::{Word, Byte};

struct MemoryMock {
    data: [u8; 512]
}

impl Default for MemoryMock {
    fn default() -> Self {
        let mut mock = MemoryMock { data: [0;512] };
        const DATA: [u8;5] = [0x44, 0x51, 0x88, 0x42, 0x99];
        mock.data[..DATA.len()].copy_from_slice(&DATA);

        return mock;
    }
}

impl Index<Word> for MemoryMock {
    type Output = Byte;

    fn index(&self, index: Word) -> &Self::Output {
        let addr: usize = index.into();
        return &self.data[addr];
    }
}

impl IndexMut<Word> for MemoryMock {
    fn index_mut(&mut self, index: Word) -> &mut Self::Output {
        let addr: usize = index.into();
        return &mut self.data[addr];
    }
}

#[cfg(test)]
mod new {
    use super::super::*;

    #[test]
    fn should_be_in_reset_state_after_creation() {
        let uut = CPU::new();

        assert_eq!(uut.accumulator, 0);
        assert_eq!(uut.cycle, 0);
        assert_eq!(uut.index_register_x, 0);
        assert_eq!(uut.index_register_y, 0);
        assert_eq!(uut.stack_pointer, 0);
        assert_eq!(uut.processor_status.flags, 0);
        assert_eq!(uut.program_counter, 0xFFFC);
    }
}

#[cfg(test)]
mod reset {
    use super::super::*;

    #[test]
    fn should_set_program_counter_to_fffc_after_reset() {
        let mut uut = CPU::new();
        uut.program_counter = 0xFFFF;

        uut.reset();

        assert_eq!(uut.program_counter, 0xFFFC);
    }

    #[test]
    fn should_set_negative_flag_in_processor_status_to_zero_after_reset() {
        let mut uut = CPU::new();
        uut.processor_status.flags = 0b11111111;

        uut.reset();

        assert_eq!(uut.processor_status.flags, 0b11110111);
    }
}

#[cfg(test)]
mod access_memory {
    use crate::cpu::CPU;
    use crate::consts::Word;
    use super::MemoryMock;

    const ADDR: Word = 0x0003;

    #[test]
    fn should_return_a_byte() {
        let memory: MemoryMock = MemoryMock::default();

        let mut uut = CPU::new();

        let result = uut.access_memory(ADDR, &memory);
        
        assert_eq!(result, 0x42);
    }

    #[test]
    fn should_increase_cycle_counter() {
        let memory: MemoryMock = MemoryMock::default();

        let mut uut = CPU::new();
        assert_eq!(uut.cycle, 0);

        uut.access_memory(ADDR, &memory);
        
        assert_eq!(uut.cycle, 1);
    }
}

#[cfg(test)]
mod fetch_byte {
    use crate::cpu::CPU;
    use super::MemoryMock;

    #[test]
    fn should_return_a_byte_pointed_by_a_program_counter() {
        let memory: MemoryMock = MemoryMock::default();

        let mut uut = CPU::new();
        uut.program_counter = 0x0001;

        let result = uut.fetch_byte(&memory);
        
        assert_eq!(result, 0x51);
    }

    #[test]
    fn should_increase_cycle_counter_and_a_program_counter() {
        let memory: MemoryMock = MemoryMock::default();

        let mut uut = CPU::new();
        uut.program_counter = 0x0001;

        assert_eq!(uut.cycle, 0);

        uut.fetch_byte(&memory);
        
        assert_eq!(uut.cycle, 1);
        assert_eq!(uut.program_counter, 0x0002);
    }
}

#[cfg(test)]
mod fetch_word {
    use crate::cpu::CPU;
    use super::MemoryMock;

    #[test]
    fn should_return_a_word_pointed_by_a_program_counter_in_little_endian() {
        let memory: MemoryMock = MemoryMock::default();

        let mut uut = CPU::new();
        uut.program_counter = 0x0001;

        let result = uut.fetch_word(&memory);
        
        assert_eq!(result, 0x8851);
    }

    #[test]
    fn should_increase_cycle_counter_and_a_program_counter_twice() {
        let memory: MemoryMock = MemoryMock::default();

        let mut uut = CPU::new();
        uut.program_counter = 0x0001;

        assert_eq!(uut.cycle, 0);

        uut.fetch_word(&memory);
        
        assert_eq!(uut.cycle, 2);
        assert_eq!(uut.program_counter, 0x0003);
    }
}

#[cfg(test)]
mod fetch_instruction {
    use crate::cpu::CPU;
    use super::MemoryMock;

    #[test]
    fn should_return_an_instruction_pointed_by_a_program_counter() {
        let memory: MemoryMock = MemoryMock::default();

        let mut uut = CPU::new();
        uut.program_counter = 0x0001;

        let result = uut.fetch_instruction(&memory);
        
        assert_eq!(result, 0x51);
    }

    #[test]
    fn should_increase_cycle_counter_and_a_program_counter() {
        let memory: MemoryMock = MemoryMock::default();

        let mut uut = CPU::new();
        uut.program_counter = 0x0001;

        assert_eq!(uut.cycle, 0);

        uut.fetch_instruction(&memory);
        
        assert_eq!(uut.cycle, 1);
        assert_eq!(uut.program_counter, 0x0002);
    }
}

#[cfg(test)]
mod push_byte_to_stack {
    use crate::cpu::CPU;
    use super::MemoryMock;

    #[test]
    fn should_push_a_byte_to_a_place_to_the_first_page_in_memory_pointed_by_a_stack_pointer() {
        let mut memory: MemoryMock = MemoryMock::default();

        let mut uut = CPU::new();
        uut.stack_pointer = 0x0002;

        let value: u8 = 0xDF;
        uut.push_byte_to_stack(value, &mut memory);
        
        assert_eq!(memory[0x0102], 0xDF);
    }

    #[test]
    fn should_increase_cycle_counter_and_stack_pointer_by_one() {
        let mut memory: MemoryMock = MemoryMock::default();

        let mut uut = CPU::new();
        uut.stack_pointer = 0x0002;

        assert_eq!(uut.cycle, 0);

        let value: u8 = 0xDF;
        uut.push_byte_to_stack(value, &mut memory);
        
        assert_eq!(uut.cycle, 1);
        assert_eq!(uut.stack_pointer, 0x0003);
    }
}

#[cfg(test)]
mod push_word_to_stack {
    use crate::cpu::CPU;
    use super::MemoryMock;

    #[test]
    fn should_push_a_byte_to_a_place_to_the_first_page_in_memory_pointed_by_a_stack_pointer() {
        let mut memory: MemoryMock = MemoryMock::default();

        let mut uut = CPU::new();
        uut.stack_pointer = 0x0002;

        let value: u16 = 0x56DF;
        uut.push_word_to_stack(value, &mut memory);
        
        assert_eq!(memory[0x0102], 0xDF);
        assert_eq!(memory[0x0103], 0x56);
    }

    #[test]
    fn should_increase_cycle_counter_and_stack_pointer_by_two() {
        let mut memory: MemoryMock = MemoryMock::default();

        let mut uut = CPU::new();
        uut.stack_pointer = 0x0002;

        assert_eq!(uut.cycle, 0);

        let value: u16 = 0x56DF;
        uut.push_word_to_stack(value, &mut memory);
        
        assert_eq!(uut.cycle, 2);
        assert_eq!(uut.stack_pointer, 0x0004);
    }
}