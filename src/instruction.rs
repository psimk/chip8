type Addr = u16;
type Byte = u8;
type Vx = u8;
type Vy = u8;

#[derive(Debug)]
pub enum Instruction {
  Clear,                          // 0x00E0
  Return,                         // 0x00EE
  JumpToRoutine(Addr),            // 0x1NNN
  CallRoutine(Addr),              // 0x2NNN
  SkipIfEqual(Vx, Byte),          // 0x3XNN
  SkipIfNotEqual(Vx, Byte),       // 0x4XNN
  SkipIfEqualRegister(Vx, Vy),    // 0x5XY_
  SetConst(Vx, Byte),             // 0x6XNN
  AddConst(Vx, Byte),             // 0x7XNN
  Set(Vx, Vy),                    // 0x8XY0
  SetOR(Vx, Vy),                  // 0x8XY1
  SetAND(Vx, Vy),                 // 0x8XY2
  SetXOR(Vx, Vy),                 // 0x8XY3
  Add(Vx, Vy),                    // 0x8XY4
  Subtract(Vx, Vy),               // 0x8XY5
  ShiftRight(Vx, Vy),             // 0x8XY6
  SetSubtracted(Vx, Vy),          // 0x8XY7
  ShiftLeft(Vx, Vy),              // 0x8XYE
  SkipIfNotEqualRegister(Vx, Vy), // 0x9XY_
  SetAddress(Addr),               // 0xANNN
  JumpToAddress(Addr),            // 0xBNNN
  SetRandom(Vx, Byte),            // 0xCXNN
  Display(Vx, Vy, Byte),          // 0xDXYN
  SkipIfKeyPressed(Vx),           // 0xEX9E
  SkipIfKeyNotPressed(Vx),        // 0xEXA1
  LoadDelay(Vx),                  // 0xFX07
  WaitForKeyPress(Vx),            // 0xFX0A
  SetDelayTimer(Vx),              // 0xFX15
  SetSoundTimer(Vx),              // 0xFX18
  AddToMem(Vx),                   // 0xFX1E
  SetSpriteLocation(Vx),          // 0xFX29
  SetBCD(Vx),                     // 0xFX33
  DumpMem(Vx),                    // 0xFX55
  LoadMem(Vx),                    // 0xFX65
}

fn first(encoded: &u16) -> u8 {
  ((encoded & 0xF000) >> 12) as u8
}

fn second(encoded: &u16) -> u8 {
  ((encoded & 0x0F00) >> 8) as u8
}

fn third(encoded: &u16) -> u8 {
  ((encoded & 0x00F0) >> 4) as u8
}

fn last_one(encoded: &u16) -> u8 {
  (encoded & 0x000F) as u8
}

fn last_two(encoded: &u16) -> u8 {
  (encoded & 0x00FF) as u8
}

fn last_three(encoded: &u16) -> u16 {
  (encoded & 0x0FFF)
}

impl Instruction {
  pub fn from_u16(encoded: &u16) -> Option<Instruction> {
    match first(encoded) {
      0x0 => match last_two(encoded) {
        0xE0 => Some(Instruction::Clear),
        0xEE => Some(Instruction::Return),
        _ => None,
      },
      0x1 => Some(Instruction::JumpToRoutine(last_three(encoded))),
      0x2 => Some(Instruction::CallRoutine(last_three(encoded))),
      0x3 => Some(Instruction::SkipIfEqual(third(encoded), last_two(encoded))),
      0x4 => Some(Instruction::SkipIfNotEqual(
        second(encoded),
        last_two(encoded),
      )),
      0x5 => Some(Instruction::SkipIfEqualRegister(
        second(encoded),
        third(encoded),
      )),
      0x6 => Some(Instruction::SetConst(second(encoded), last_two(encoded))),
      0x7 => Some(Instruction::AddConst(second(encoded), last_two(encoded))),
      0x8 => match last_one(encoded) {
        0x0 => Some(Instruction::Set(second(encoded), third(encoded))),
        0x1 => Some(Instruction::SetOR(second(encoded), third(encoded))),
        0x2 => Some(Instruction::SetAND(second(encoded), third(encoded))),
        0x3 => Some(Instruction::SetXOR(second(encoded), third(encoded))),
        0x4 => Some(Instruction::Add(second(encoded), third(encoded))),
        0x5 => Some(Instruction::Subtract(second(encoded), third(encoded))),
        0x6 => Some(Instruction::ShiftRight(second(encoded), third(encoded))),
        0x7 => Some(Instruction::SetSubtracted(second(encoded), third(encoded))),
        0xE => Some(Instruction::ShiftLeft(second(encoded), third(encoded))),
        _ => None,
      },
      0x9 => Some(Instruction::SkipIfNotEqualRegister(
        second(encoded),
        third(encoded),
      )),
      0xA => Some(Instruction::SetAddress(last_three(encoded))),
      0xB => Some(Instruction::JumpToAddress(last_three(encoded))),
      0xC => Some(Instruction::SetRandom(second(encoded), last_two(encoded))),
      0xD => Some(Instruction::Display(
        second(encoded),
        third(encoded),
        last_one(encoded),
      )),
      0xE => match last_two(encoded) {
        0x9E => Some(Instruction::SkipIfKeyPressed(second(encoded))),
        0xA1 => Some(Instruction::SkipIfKeyNotPressed(second(encoded))),
        _ => None,
      },
      0xF => match last_two(encoded) {
        0x07 => Some(Instruction::LoadDelay(second(encoded))),
        0x0A => Some(Instruction::WaitForKeyPress(second(encoded))),
        0x15 => Some(Instruction::SetDelayTimer(second(encoded))),
        0x18 => Some(Instruction::SetSoundTimer(second(encoded))),
        0x1E => Some(Instruction::AddToMem(second(encoded))),
        0x29 => Some(Instruction::SetSpriteLocation(second(encoded))),
        0x33 => Some(Instruction::SetBCD(second(encoded))),
        0x55 => Some(Instruction::DumpMem(second(encoded))),
        0x65 => Some(Instruction::LoadMem(second(encoded))),
        _ => None,
      },
      _ => {
        println!("{:X?} is not defined", encoded);
        None
      }
    }
  }
}
