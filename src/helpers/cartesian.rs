use std::ops::Add;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Point2D {
  pub x: usize,
  pub y: usize
}

#[derive(Debug)]
pub struct Slope2D {
  pub dx: i64,
  pub dy: i64
}

impl Add<Slope2D> for Point2D {
  fn add(self, other: Slope2D) -> Self {
    Self {
      x: ((self.x as i64) + (other.dx as i64)) as usize,
      y: ((self.y as i64) + (other.dy as i64)) as usize
    }
  }
  type Output = Point2D;
}

impl Point2D {
  pub fn slope_between(&self, other: &Point2D) -> Slope2D {
    Slope2D { dy: ((self.y as i64) - (other.y as i64)), dx: ((self.x as i64) - (other.x as i64)) }
  }
}