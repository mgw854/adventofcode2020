

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


pub fn run_rules2(input: &Vec<Seat>) -> (u64, Vec<Seat>) {
  let mut output = Vec::new();
  let mut adjustments = 0;

  for seat in input {
    let adjacents = get_all_nearest(seat, input);

  // If seat is empty, and no occupied seats adjacent, it becomes occupied
    // If seat is occupied, and four or more adjacent seats are occupied, it becomes empty
    // Otherwise, do nothing
    if seat.status == Status::Empty && !adjacents.iter().any(|s| s.status == Status::Occupied) {
      adjustments += 1;
      output.push(Seat { status: Status::Occupied, position: seat.position });
    } else if seat.status == Status::Occupied && adjacents.iter().filter(|s| s.status == Status::Occupied).count() >= 5 {
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


pub fn run_rules_until_stable2(input: Vec<Seat>) -> usize {
  let mut changes = 1000;

  let mut arrangement = input;

  while changes > 0 {
    let u = run_rules2(&arrangement);
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

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Direction {
  North,
  Northwest,
  West,
  Southwest,
  South,
  Southeast,
  East,
  Northeast
}

pub struct NearestIterator(Seat, Vec<Seat>, Option<Direction>);

fn get_all_nearest(seat: &Seat, search: &Vec<Seat>) -> Vec<Seat> {
  let mut direction = Some(Direction::North);

  let mut output: Vec<Seat> = Vec::new();

  while direction.is_some() {
    if let Some(x) = next_nearest(seat, search, direction) {
      output.push(x);
    }

    direction = direction.unwrap().next();
  }

  output
}

fn next_nearest(seat: &Seat, search: &Vec<Seat>, direction: Option<Direction>) -> Option<Seat> {
  if let Some(search_dir) = direction {
    let pos = seat.position;

    // Get only the points that we care to filter on for our direction
    // Unless we're going in a X direction instead of a + direction, then also make sure the slope is 1.
    // Of those, find the closet
    let mut care_about : Vec<Seat> = search.iter().filter_map(|s| {
      let slope = pos.slope_between(&s.position);

      if slope.dx == 0 && slope.dy == 0 {
        return None;
      }

      let direction = slope.get_direction();

      if direction == search_dir {
        if direction == Direction::North || direction == Direction::South || direction == Direction::East || direction == Direction::West {
          return Some(*s);
        }

        if slope.dx.abs() == slope.dy.abs() {
          return Some(*s);
        }
      }

      None
    }).collect::<Vec<Seat>>();
    
    if care_about.len() > 0 {
      care_about.sort_by(|a, b| {
        let a_slope = pos.slope_between(&a.position);
        let b_slope = pos.slope_between(&b.position);

        let magnitude = b_slope.dx.abs() + b_slope.dy.abs();

        (a_slope.dx.abs() + a_slope.dy.abs()).cmp(&magnitude)
      });

      return Some(care_about[0]);
    }
  }

  None
}

impl Direction {
  pub fn next(&self) -> Option<Self> {
    match self {
      Direction::North => Some(Direction::Northeast),
      Direction::Northeast => Some(Direction::East),
      Direction::East => Some(Direction::Southeast),
      Direction::Southeast => Some(Direction::South),
      Direction::South => Some(Direction::Southwest),
      Direction::Southwest => Some(Direction::West),
      Direction::West => Some(Direction::Northwest),
      Direction::Northwest => None
    }
  }
}

impl Slope2D {
  fn get_direction(&self) -> Direction {
    if self.dy == 0 {
      if self.dx > 0 {
        return Direction::East
      }

      return Direction::West
    } else if self.dy > 0 {
      if self.dx == 0 {
        return Direction::North;
      } else if self.dx > 0 {
        return Direction::Northeast
      } else {
        return Direction::Northwest
      }
    } else {
      if self.dx == 0 {
        return Direction::South;
      } else if self.dx > 0 {
        return Direction::Southeast
      } else {
        return Direction::Southwest
      }    
    }
  }
}

pub fn solve (input: &str) -> usize {
  run_rules_until_stable(convert_input_to_points(input.lines().collect()))
}

pub fn solve2 (input: &str) -> usize {
  run_rules_until_stable2(convert_input_to_points(input.lines().collect()))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day11_part1() {
    let input = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL\n";

    assert_eq!(37, run_rules_until_stable(convert_input_to_points(input.lines().collect())));
  }

  
  #[test]
  fn day11_part2() {
    let input = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL\n";

    assert_eq!(26, run_rules_until_stable2(convert_input_to_points(input.lines().collect())));
  }
}