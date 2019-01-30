use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

mod bus;
mod cache;
mod cpu;
mod display;
mod instruction;
mod keypad;

use crate::bus::Bus;
use crate::cache::Cache;
use crate::cpu::CPU;
use crate::display::Display;
use crate::instruction::Instruction;
use crate::keypad::Keypad;

const GAME_FILE: &str = "HIDDEN";
const ROM_DIR: &str = "./roms";

const FRAME_TICK: Duration = Duration::from_millis(16);
const CPU_TICK: Duration = Duration::from_millis(2);

fn main() -> std::io::Result<()> {
  let mut file = File::open(&format!("{}/{}", ROM_DIR, GAME_FILE))?;
  let mut buf = Vec::new();

  file.read_to_end(&mut buf)?;

  let keypad = Keypad::new();
  let display = Display::new((640, 320));
  let mut events = display.context.event_pump().unwrap();

  let bus = Bus::new(display, keypad);

  let mut cpu = CPU::new(&buf, bus);

  let mut cpu_last = Instant::now();
  let mut frame_last = Instant::now();

  'game_loop: loop {
    for event in events.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'game_loop,
        _ => cpu.bus.keypad.on_key_event(event),
      }
    }

    if cpu.bus.keypad.keys[15] != 1 {
      if cpu_last.elapsed() >= CPU_TICK {
        cpu.cycle()?;
        cpu_last = Instant::now();
      }
    }

    if frame_last.elapsed() >= FRAME_TICK {
      cpu.bus.display.draw();

      frame_last = Instant::now();
    }
  }

  Ok(())
}
