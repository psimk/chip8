use crate::Display;
use crate::Keypad;

pub struct Bus {
  pub display: Display,
  pub keypad: Keypad,
}

impl Bus {
  pub fn new(display: Display, keypad: Keypad) -> Bus {
    Bus { display, keypad }
  }
}
