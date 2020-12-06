enum Partition {
  Front,
  Back,
  Left,
  Right
}

impl Partition {
  fn parse_char(c: &char) -> Partition {
    match c {
      'F' => Partition::Front,
      'B' => Partition::Back,
      'L' => Partition::Left,
      'R' => Partition::Right,
      _ => panic!()
    }
  }
}

#[derive(Debug)]
struct SeatRange {
  min_row: u8,
  max_row: u8,
  min_col: u8,
  max_col: u8
}

impl SeatRange {
  fn new() -> SeatRange {
    SeatRange {
      min_row: 0,
      max_row: 127,
      min_col: 0,
      max_col: 7
    }
  }

  fn apply(&self, partition: Partition) -> SeatRange {
    match partition {
      Partition::Front => SeatRange { min_row: self.min_row, max_row: halve_down(self.min_row, self.max_row), min_col: self.min_col, max_col: self.max_col },
      Partition::Back => SeatRange { min_row: halve_up(self.min_row, self.max_row), max_row: self.max_row, min_col: self.min_col, max_col: self.max_col },
      Partition::Left => SeatRange { min_row: self.min_row, max_row: self.max_row, min_col: self.min_col, max_col: halve_down(self.min_col, self.max_col)},
      Partition::Right => SeatRange { min_row: self.min_row, max_row: self.max_row, min_col: halve_up(self.min_col, self.max_col), max_col: self.max_col }
    }
  }

  fn get_id(&self) -> u64 {
    if !(self.min_col == self.max_col && self.min_row == self.max_row) {
      panic!()
    }

    return (self.min_row as u64) * 8 + (self.min_col as u64);
  }
}

fn halve_up(min: u8, max: u8) -> u8 {
  let diff = max - min;

  if diff == 1 {
    return max;
  }

  return min + (diff / 2) + 1
}

fn halve_down(min: u8, max: u8) -> u8 {
  let diff = max - min;

  if diff == 1 {
    return min;
  }

  return max - (diff / 2) - 1
}

pub fn get_seat_id(input: &str) -> u64 {
  input.chars().map(|c| Partition::parse_char(&c)).fold(SeatRange::new(), |acc, x| acc.apply(x)).get_id()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day5_part1() {
    let input = "FBFBBFFRLR";

    assert_eq!(357, get_seat_id(input));
  }
  
  #[test]
  fn day5_part1a() {
    let input = "BFFFBBFRRR";

    assert_eq!(567, get_seat_id(input));
  }
  
  #[test]
  fn day5_part1b() {
    let input = "FFFBBBFRRR";

    assert_eq!(119, get_seat_id(input));
  }
  
  #[test]
  fn day5_part1c() {
    let input = "BBFFBBFRLL";

    assert_eq!(820, get_seat_id(input));
  }
}