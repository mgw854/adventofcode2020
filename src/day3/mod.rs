fn hit_trees(input: Vec<&str>, slope_x: usize, slope_y: usize) -> usize {
  let y_max = input.len();
  let trees = convert_trees_to_points(input);

  let mut x = 0;
  let mut y = 0;
  let mut hits = 0;

  while y <= y_max {
    y += slope_y;
    x += slope_x;

    if trees.iter().any(|t| t.y_coordinate == y && (x % t.x_pattern_size) == t.x_coordinate) {
      hits += 1;
    }
  }

  hits
}

fn convert_trees_to_points(input: Vec<&str>) -> Vec<ChristmasTree> {
  let mut trees : Vec<ChristmasTree> = Vec::new();
  let pattern_size = input[0].trim().len();


  let mut y = 0;

  for line in input {
    for (x, c) in line.char_indices() {
      if c == '#' {
        trees.push(ChristmasTree { x_coordinate: x, y_coordinate: y, x_pattern_size: pattern_size });
      }
    }

    y = y + 1;
  }

  trees
}

struct ChristmasTree {
  x_coordinate: usize,
  y_coordinate: usize,
  x_pattern_size: usize
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day3_part1() {
    let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";

    assert_eq!(7, hit_trees(input.lines().collect(), 3, 1));
  }
}