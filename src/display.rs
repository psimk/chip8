// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Display {
  pub context: sdl2::Sdl,
  pub canvas: Canvas<Window>,
  pub matrix: Vec<usize>,
}

const WINDOW_NAME: &str = "Chip 8";
impl Display {
  pub const REAL_RESOLUTION: (usize, usize) = (64, 32);

  pub fn new(resolution: (usize, usize)) -> Display {
    let context = sdl2::init().unwrap();
    let subsystem = context.video().unwrap();

    let window = subsystem
      .window(WINDOW_NAME, resolution.0 as u32, resolution.1 as u32)
      .position_centered()
      .build()
      .unwrap();

    let canvas = window.into_canvas().build().unwrap();

    Display {
      context,
      canvas,
      matrix: vec![0; Display::REAL_RESOLUTION.0 * Display::REAL_RESOLUTION.1],
    }
  }

  pub fn clear(&mut self) {
    self.matrix = self.matrix.iter_mut().map(|_| 0).collect::<Vec<usize>>();
  }

  pub fn draw(&mut self) {
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.canvas.clear();

    self.canvas.set_draw_color(Color::RGB(255, 255, 255));

    for y in 0..Display::REAL_RESOLUTION.1 {
      for x in 0..Display::REAL_RESOLUTION.0 {
        if self.matrix[(y * Display::REAL_RESOLUTION.0) + x] == 1 {
          self
            .canvas
            .fill_rect(Rect::new(
              (x as u8) as i32 * 10,
              (y as u8) as i32 * 10,
              10,
              10,
            ))
            .unwrap();
        }
      }
    }
    self.canvas.present();
  }
}
