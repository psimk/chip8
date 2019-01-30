use crate::Bus;
use crate::Cache;
use crate::Display;
use crate::Instruction;
use byteorder::ReadBytesExt;
use std::io::{Error, Write};

pub struct CPU {
  pc: usize,
  registers: [u8; 16],
  address: u16,
  call_stack: Vec<usize>,
  cache: Cache,
  delay_timer: u8,
  sound_timer: u8,
  pub bus: Bus,
}

const COUNTER_START: usize = 0x200;

impl CPU {
  pub fn new(buf: &Vec<u8>, bus: Bus) -> CPU {
    CPU {
      cache: Cache::new(&buf),
      pc: COUNTER_START,
      call_stack: vec![],
      registers: [0; 16],
      address: 0,
      delay_timer: 0,
      sound_timer: 0,
      bus,
    }
  }

  pub fn cycle(&mut self) -> Result<(), Error> {
    let raw_opcode = self.cache.next_opcode(self.pc as u64)?;

    if let Some(instruction) = Instruction::from_u16(&raw_opcode) {
      // println!("{:X?}: {:X?}", raw_opcode, instruction);
      self.do_instruction(&instruction)?;
    }

    Ok(())
  }

  fn timer_tick(&mut self) {
    if self.delay_timer > 0 {
      self.delay_timer -= 1;
    }

    if self.sound_timer > 0 {
      self.sound_timer -= 1;
    }
  }

  fn do_instruction(&mut self, instruction: &Instruction) -> Result<(), Error> {
    let mut should_increment_pc = true;

    self.timer_tick();

    match instruction {
      Instruction::Clear => self.bus.display.clear(),
      Instruction::Return => {
        if let Some(ret) = self.call_stack.pop() {
          self.pc = ret;
          should_increment_pc = true;
        }
      }
      Instruction::JumpToRoutine(addr) => {
        self.pc = *addr as usize;
        should_increment_pc = false;
      }
      Instruction::CallRoutine(addr) => {
        self.call_stack.push(self.pc);
        self.pc = *addr as usize;
        should_increment_pc = false;
      }
      Instruction::SkipIfEqual(vx, data) => {
        if self.registers[*vx as usize] == *data {
          self.inc_pc();
        }
      }
      Instruction::SkipIfNotEqual(vx, data) => {
        if self.registers[*vx as usize] != *data {
          self.inc_pc();
        }
      }
      Instruction::SkipIfEqualRegister(vx, vy) => {
        if self.registers[*vx as usize] == self.registers[*vy as usize] {
          self.inc_pc();
        }
      }
      Instruction::SetConst(vx, data) => {
        self.registers[*vx as usize] = *data;
      }
      Instruction::AddConst(vx, data) => {
        self.registers[*vx as usize] = self.registers[*vx as usize].wrapping_add(*data);
      }
      Instruction::Set(vx, vy) => {
        self.registers[*vx as usize] = self.registers[*vy as usize];
      }
      Instruction::SetOR(vx, vy) => {
        self.registers[*vx as usize] |= self.registers[*vy as usize];
      }
      Instruction::SetAND(vx, vy) => {
        self.registers[*vx as usize] &= self.registers[*vy as usize];
      }
      Instruction::SetXOR(vx, vy) => {
        self.registers[*vx as usize] ^= self.registers[*vy as usize];
      }
      Instruction::Add(vx, vy) => {
        let x = self.registers[*vx as usize];
        let y = self.registers[*vy as usize];

        let ret = x as u16 + y as u16;
        self.registers[*vx as usize] = x.wrapping_add(y);
        self.registers[0xF] = (ret > 255) as u8;
      }
      Instruction::Subtract(vx, vy) => {
        let x = self.registers[*vx as usize];
        let y = self.registers[*vy as usize];

        self.registers[0xF] = (x > y) as u8;
        self.registers[*vx as usize] = x.wrapping_sub(y);
      }
      Instruction::ShiftRight(vx, vy) => {
        self.registers[0xF] = self.registers[*vy as usize] & 1;
        self.registers[*vx as usize] = self.registers[*vy as usize] >> 1;
      }
      Instruction::SetSubtracted(vx, vy) => {
        let x = self.registers[*vx as usize];
        let y = self.registers[*vy as usize];

        self.registers[*vx as usize] = y.wrapping_sub(x);
        self.registers[0xF] = (x > y) as u8;
      }
      Instruction::ShiftLeft(vx, vy) => {
        self.registers[0xF] = self.registers[*vy as usize] >> 7;
        self.registers[*vx as usize] = self.registers[*vy as usize] << 1;
      }
      Instruction::SkipIfNotEqualRegister(vx, vy) => {
        if self.registers[*vx as usize] != self.registers[*vy as usize] {
          self.inc_pc()
        }
      }
      Instruction::SetAddress(addr) => {
        self.address = *addr;
      }
      Instruction::JumpToAddress(addr) => {
        self.pc = ((self.registers[0] as u16) + *addr) as usize;
        should_increment_pc = false
      }
      Instruction::SetRandom(vx, data) => {
        self.registers[*vx as usize] = rand::random::<u8>() & *data;
;
      }
      Instruction::Display(vx, vy, data) => {
        let mut start_x = self.registers[*vx as usize];
        let start_y = self.registers[*vy as usize];
        let height = *data;

        self.registers[0xF] = 0;

        if start_x > (Display::REAL_RESOLUTION.0 - 1) as u8 {
          start_x = 0;
        }

        for y in 0..height {
          let row = self.cache.read_from((self.address + y as u16) as u64)?;
          let final_y = y as u16 + start_y as u16;

          for x in 0..8 {
            let final_x = x as u16 + start_x as u16;

            if final_x > (Display::REAL_RESOLUTION.0 - 1) as u16 {
              continue;
            }

            let coord = ((final_y * Display::REAL_RESOLUTION.0 as u16) + final_x) as usize;

            if coord >= Display::REAL_RESOLUTION.0 * Display::REAL_RESOLUTION.1 {
              continue;
            }
            if (row >> 7 - x) & 1 != 0 {
              self.registers[0xf] = (self.bus.display.matrix[coord] == 1) as u8;
              self.bus.display.matrix[coord] ^= 1;
            }
          }
        }
      }
      Instruction::SkipIfKeyPressed(vx) => {
        let key = self.registers[*vx as usize] as usize;
        println!("PRESSED: {:?} {:?}", self.bus.keypad.keys[key], key);
        if self.bus.keypad.keys[key] == 1 {
          self.inc_pc();
        }
      }
      Instruction::SkipIfKeyNotPressed(vx) => {
        let key = self.registers[*vx as usize] as usize;
        println!("NOT: {:?} {:?}", self.bus.keypad.keys[key], key);
        if self.bus.keypad.keys[key] != 1 {
          self.inc_pc();
        }
      }
      Instruction::LoadDelay(vx) => {
        self.registers[*vx as usize] = self.delay_timer;
      }
      Instruction::WaitForKeyPress(vx) => {
        let key = self.registers[*vx as usize] as usize;
        println!("WAIT: {:?} {:?}", self.bus.keypad.keys[key], key);
        if self.bus.keypad.keys[key] != 1 {
          should_increment_pc = false;
        }
      }
      Instruction::SetDelayTimer(vx) => {
        self.delay_timer = self.registers[*vx as usize];
      }
      Instruction::SetSoundTimer(vx) => {
        self.sound_timer = self.registers[*vx as usize];
      }
      Instruction::AddToMem(vx) => {
        self.address += self.registers[*vx as usize] as u16;
      }
      Instruction::SetSpriteLocation(vx) => {
        self.address = (self.registers[*vx as usize] * 5) as u16;
      }
      Instruction::SetBCD(vx) => {
        let val = self.registers[*vx as usize];

        let h = val / 100;
        let t = (val / 10) % 10;
        let d = (val % 100) % 10;

        self.cache.write_to(self.address as u64, &[h, t, d])?;
      }
      Instruction::DumpMem(vx) => {
        for idx in 0..*vx + 1 {
          self
            .cache
            .write_to(self.address as u64, &[self.registers[idx as usize]])?;
          self.address += 1;
        }
      }
      Instruction::LoadMem(vx) => {
        for idx in 0..*vx + 1 {
          self.registers[idx as usize] = self.cache.read_from((self.address as u16) as u64)?;
          self.address += 1;
        }
      }
    };

    if should_increment_pc {
      self.inc_pc();
    }

    Ok(())
  }

  fn inc_pc(&mut self) {
    self.pc += 2;
  }
}
