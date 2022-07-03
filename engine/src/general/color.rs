#[derive(Debug, Copy, Clone)]
/// Colors represented in normalized values between 1.0 and -0.0
/// Eg: Red would be r: 1.0, g:0.0, b: 0.0, a: 1.0
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl Color {
  pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color { r, g, b, a }
  }
}

/// Helper enum with predefined colors that translate to Color structs
pub enum COLORS {
  Red,
  White,
  Black,
  Green,
  Violet,
}

impl Into<Color> for COLORS {
  fn into(self) -> Color {
    match self {
      COLORS::Red => Color::new(1.0, 0.0, 0.0, 1.0),
      COLORS::Black => Color::new(0.0, 0.0, 0.0, 1.0),
      COLORS::White => Color::new(1.0, 1.0, 1.0, 1.0),
      COLORS::Green => Color::new(0.0, 1.0, 0.0, 1.0),
      COLORS::Violet => Color::new(1.0, 0.0, 1.0, 1.0),
    }
  }
}
