

use super::helpers::cartesian::*;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Status {
  Empty,
  Occupied
}

#[derive(Debug, Clone, Copy)]
pub struct Seat {
  status: Status,
  position: Point2D
}

fn convert_input_to_points(input: Vec<&str>) -> Vec<Seat> {
  let mut seats : Vec<Seat> = Vec::new();

  let mut y = 0;

  for line in input {
    for (x, c) in line.char_indices() {
      let status = match c {
        '#' => Status::Occupied,
        'L' => Status::Empty,
        _ => continue
      };

      seats.push(Seat { status: status, position: Point2D { x: x, y: y }});
    };

    y = y + 1;
  }

  seats
}

pub fn run_rules(input: &Vec<Seat>) -> (u64, Vec<Seat>) {
  let mut output = Vec::new();
  let mut adjustments = 0;

  for seat in input {
    let adjacents = input.clone().into_iter().find_adjacents(*seat).collect::<Vec<Seat>>();

  // If seat is empty, and no occupied seats adjacent, it becomes occupied
    // If seat is occupied, and four or more adjacent seats are occupied, it becomes empty
    // Otherwise, do nothing
    if seat.status == Status::Empty && !adjacents.iter().any(|s| s.status == Status::Occupied) {
      adjustments += 1;
      output.push(Seat { status: Status::Occupied, position: seat.position });
    } else if seat.status == Status::Occupied && adjacents.iter().filter(|s| s.status == Status::Occupied).count() >= 4 {
      adjustments += 1;
      output.push(Seat { status: Status::Empty, position: seat.position });
    } else {
      output.push(seat.clone());
    }
  }

  (adjustments, output)
}

pub fn run_rules_until_stable(input: Vec<Seat>) -> usize {
  let mut changes = 1000;

  let mut arrangement = input;

  while changes > 0 {
    let u = run_rules(&arrangement);
    changes = u.0;
    arrangement = u.1;

    dbg!(changes);
  }

  arrangement.iter().filter(|s| s.status == Status::Occupied).count()
}

pub struct AdjacencyIterator<I>(Seat, I)
  where I: Iterator<Item = Seat>;

impl<I> Iterator for AdjacencyIterator<I>
  where I: Iterator<Item = Seat>
{
  type Item = Seat;

  fn next(&mut self) -> Option<Self::Item> {
    let pos = self.0.position;

    while let Some(n) = self.1.next() {
      let dx = (n.position.x as i64 - pos.x as i64).abs();
      let dy = (n.position.y as i64 - pos.y as i64).abs();

      if dy == 0 && dx == 0 {
        continue;
      } else if dy == 1 && (dx == 0 || dx == 1) {
        return Some(n);
      } else if dx == 1 && dy == 0 {
        return Some(n);
      }
    }
    
    None
  }
}

pub trait FindAdjacent : Iterator<Item = Seat> + Sized {
  fn find_adjacents(self, point: Seat) -> AdjacencyIterator<Self>;
}

impl<I> FindAdjacent for I where I: Iterator<Item = Seat>
{
  fn find_adjacents(self, point: Seat) -> AdjacencyIterator<Self> {
    AdjacencyIterator(point, self)
  }
}

pub fn solve (input: &str) -> usize {
  run_rules_until_stable(convert_input_to_points(input.lines().collect()))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day11_part1() {
    let input = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL\n";

    assert_eq!(37, run_rules_until_stable(convert_input_to_points(input.lines().collect())));
  }
}