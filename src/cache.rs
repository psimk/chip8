use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Error, Write};

const SPRITE_CHARACTERS: [u8; 80] = [
  0xf0, 0x90, 0x90, 0x90, 0xf0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
  0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
  0x90, 0x90, 0xf0, 0x10, 0x10, // 4
  0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
  0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
  0xf0, 0x10, 0x20, 0x40, 0x40, // 7
  0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
  0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
  0xf0, 0x90, 0xf0, 0x90, 0x90, // A
  0xe0, 0x90, 0xe0, 0x90, 0xe0, // B
  0xf0, 0x80, 0x80, 0x80, 0x80, // C
  0xe0, 0x90, 0x90, 0x90, 0xe0, // D
  0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
  0xf0, 0x80, 0xf0, 0x80, 0x80, // F
];

const MEMORY_SIZE: usize = 0x1000;
const INTERPRETER_BOUNDS: (usize, usize) = (0x0, 0x200);

pub struct Cache {
  memory: Cursor<Vec<u8>>,
}

impl Cache {
  pub fn new(data: &Vec<u8>) -> Cache {
    let mut memory = vec![0; MEMORY_SIZE];

    for i in 0..SPRITE_CHARACTERS.len() {
      memory[INTERPRETER_BOUNDS.0 + i] = SPRITE_CHARACTERS[i];
    }

    for i in 0..data.len() {
      memory[INTERPRETER_BOUNDS.1 + i] = data[i];
    }

    Cache {
      memory: Cursor::new(memory),
    }
  }

  pub fn next_opcode(&mut self, pos: u64) -> Result<u16, Error> {
    self.memory.set_position(pos);
    self.memory.read_u16::<BigEndian>()
  }

  pub fn read_from(&mut self, pos: u64) -> Result<u8, Error> {
    self.memory.set_position(pos);
    self.memory.read_u8()
  }

  pub fn write_to(&mut self, pos: u64, data: &[u8]) -> Result<(), Error> {
    self.memory.set_position(pos);
    self.memory.write(data)?;
    Ok(())
  }
}
