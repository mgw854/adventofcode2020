use std::error::Error;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct NavigationInstruction
{
  direction: Direction,
  run: u64
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
  North,
  South,
  East,
  West,
  Left,
  Right,
  Forward
}

impl NavigationInstruction {
  pub fn parse(input: &str) -> Result<NavigationInstruction, Box<dyn Error>> {
    let dir = match input.chars().nth(0) {
      Some('L') => Direction::Left,
      Some('R') => Direction::Right,
      Some('F') => Direction::Forward,
      Some('N') => Direction::North,
      Some('S') => Direction::South,
      Some('E') => Direction::East,
      Some('W') => Direction::West,
      _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Bad direction")))
    };

    let run : u64 = input[1..].parse()?;

    Ok(NavigationInstruction { direction : dir, run : run })
  }
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
pub struct Point {
  x: i64,
  y: i64
}


#[derive(Eq, PartialEq, Debug, Hash)]
pub struct ShipPosition {
  point: Point,
  length: u32
}

pub fn generate_position(instructions: &Vec<NavigationInstruction>) -> i64 {
  let mut x : i64 = 0;
  let mut y : i64 = 0;
  let mut orientation = Direction::East;
  
  for instr in instructions {
    let orientation = match instr.direction {
      Direction::Forward => orientation,
      Direction::Right => {
        let mut rot = instr.run;
        while rot > 0 {
          orientation = match orientation {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            _ => panic!()
          };
          rot -= 90;
        }

        continue;
      },
      Direction::Left => {
        let mut rot = instr.run;
        while rot > 0 {
          orientation = match orientation {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            _ => panic!()
          };
          rot -= 90;
        }

        continue;
      },
      x => x
    };

    match orientation {
      Direction::North => { x += (instr.run as i64); },
      Direction::South => { x -= (instr.run as i64); },
      Direction::East => { y += (instr.run as i64); },
      Direction::West => { y -= (instr.run as i64); },
      _ => panic!()
    };
  };

  x.abs() + y.abs()
}


pub fn generate_waypoint_position(instructions: &Vec<NavigationInstruction>) -> i64 {
  let mut x : i64 = 0;
  let mut y : i64 = 0;

  let mut waypoint_x: i64 = 10;
  let mut waypoint_y: i64 = 1;
  
  for instr in instructions {
    match instr.direction {
      Direction::Forward => {
        x += waypoint_x * instr.run as i64;
        y += waypoint_y * instr.run as i64;
      },
      Direction::North => { waypoint_y += instr.run as i64 },
      Direction::South => { waypoint_y -= instr.run as i64 },
      Direction::East => { waypoint_x += instr.run as i64 },
      Direction::West => { waypoint_x -= instr.run as i64 },
      Direction::Right => {
        let mut rot = instr.run;
        while rot > 0 {
          let new_y = waypoint_x * -1;
          waypoint_x = waypoint_y;
          waypoint_y = new_y;   
          rot -= 90;
        };
      },
      Direction::Left => {
        let mut rot = instr.run;
        while rot > 0 {
          let new_x = waypoint_y * -1;
          waypoint_y = waypoint_x;
          waypoint_x = new_x;
          rot -= 90;
        };
      }
    };
  }

  x.abs() + y.abs()
}

/*
pub fn generate_positions(instructions: &Vec<WiringInstruction>) -> Vec<WirePosition> {
  let mut vec : Vec<WirePosition> = Vec::new();

  let mut originX = 0;
  let mut originY = 0;
  let mut totalLen = 0;
  
  for instr in instructions {
    match instr.direction {
      Direction::Left => {
        for len in 0..instr.run
        {
          originX -= 1;
          totalLen += 1;
          vec.push(WirePosition { point: Point { x: originX, y: originY }, length: totalLen });
        }
      },
      Direction::Right => {
        for len in 0..instr.run
        {
          originX += 1;
          totalLen += 1;
          vec.push(WirePosition { point: Point { x: originX, y: originY }, length: totalLen });
        }
      },
      Direction::Up => {
        for len in 0..instr.run
        {
          originY += 1;
          totalLen += 1;
          vec.push(WirePosition { point: Point { x: originX, y: originY }, length: totalLen });
        }
      },
      Direction::Down => {
        for len in 0..instr.run
        {
          originY -= 1;
          totalLen += 1;
          vec.push(WirePosition { point: Point { x: originX, y: originY }, length: totalLen });
        }
      }
    }
  };

  vec
}*/

/*
pub fn generate_shortest_path(one: &Vec<WirePosition>, two: &Vec<WirePosition>) -> u32 {
  let oneHash : HashSet<Point> = one.iter().map(|p| p.point).collect();
  let twoHash : HashSet<Point> = two.iter().map(|p| p.point).collect();

  let mut minPath = 2000000000;

  for pt in oneHash.intersection(&twoHash)
  {
    let oneDistance = one.iter().filter(|o| o.point == *pt).min_by(|wp, y| wp.length.cmp(&y.length));
    let twoDistance = two.iter().filter(|o| o.point == *pt).min_by(|wp, y| wp.length.cmp(&y.length));

    if let Some(oneD) = oneDistance {
      if let Some(twoD) = twoDistance {
        if oneD.length + twoD.length < minPath {
          minPath = oneD.length + twoD.length;
        }
      }
    }
  };

  minPath
}*/

#[cfg(test)]
mod tests {
    use super::*;
/*
    #[test]
    fn test_parsing_instruction() {
      assert_eq!(WiringInstruction::parse("D958").expect("Value wasn't parsable"), WiringInstruction { direction: Direction::Down, run: 958});
      assert_eq!(WiringInstruction::parse("U1").expect("Value wasn't parsable"), WiringInstruction { direction: Direction::Up, run: 1});
      assert_eq!(WiringInstruction::parse("L48").expect("Value wasn't parsable"), WiringInstruction { direction: Direction::Left, run: 48});
      assert_eq!(WiringInstruction::parse("R37").expect("Value wasn't parsable"), WiringInstruction { direction: Direction::Right, run: 37});
    }
*/

    #[test]
    fn day12_part1() {
      let input = "F10\nN3\nF7\nR90\nF11";

      let instructions = input.lines().map(|x| NavigationInstruction::parse(x).unwrap()).collect();
      assert_eq!(25, generate_position(&instructions));
    }

    #[test]
    fn day12_part2() {
      let input = "F10\nN3\nF7\nR90\nF11";

      let instructions = input.lines().map(|x| NavigationInstruction::parse(x).unwrap()).collect();
      assert_eq!(286, generate_waypoint_position(&instructions));
    }
  }