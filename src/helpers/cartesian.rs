use std::ops::Add;
pub struct Point2D {
  x: usize,
  y: usize
}

pub struct Slope2D {
  dx: i32,
  dy: i32
}

impl<Add<Point>> for Point2D {
  fn add(self, other: Slope2D) -> Self {
    Self {
      x: self.x + other.dx,
      y: self.y + other.dy
    }
  }
}