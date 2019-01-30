use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Keypad {
  pub keys: [u8; 16],
}

impl Keypad {
  pub fn new() -> Keypad {
    Keypad { keys: [0; 16] }
  }

  pub fn on_key_event(&mut self, event: Event) {
    match event {
      Event::KeyDown {
        keycode: Some(Keycode::Space),
        ..
      } => self.keys[15] ^= 1,

      Event::KeyDown {
        keycode: Some(Keycode::Left),
        ..
      } => self.keys[4] = 1,
      Event::KeyUp {
        keycode: Some(Keycode::Left),
        ..
      } => self.keys[4] = 0,

      Event::KeyDown {
        keycode: Some(Keycode::Right),
        ..
      } => self.keys[1] = 1,
      Event::KeyUp {
        keycode: Some(Keycode::Right),
        ..
      } => self.keys[1] = 0,

      Event::KeyDown {
        keycode: Some(Keycode::Up),
        ..
      } => self.keys[12] = 1,
      Event::KeyUp {
        keycode: Some(Keycode::Up),
        ..
      } => self.keys[12] = 0,

      Event::KeyDown {
        keycode: Some(Keycode::Down),
        ..
      } => self.keys[13] = 1,
      Event::KeyUp {
        keycode: Some(Keycode::Down),
        ..
      } => self.keys[13] = 0,

      Event::KeyDown {
        keycode: Some(Keycode::Return),
        ..
      } => self.keys[0] = 1,
      Event::KeyUp {
        keycode: Some(Keycode::Return),
        ..
      } => self.keys[0] = 0,
      _ => {}
    }
  }
}
