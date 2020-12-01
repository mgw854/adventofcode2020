use itertools::Itertools;

pub fn find_2020_and_multiply_part1(input: Vec<i32>) -> i32
{
  for x in input.iter().combinations(2) {
    if x[0] + x[1] == 2020 {
      return x[0] * x[1];
    }
  }

  panic!()
}

pub fn find_2020_and_multiply_part2(input: Vec<i32>) -> i32
{
  for x in input.iter().combinations(3) {
    if x[0] + x[1] + x[2] == 2020 {
      return x[0] * x[1] * x[2];
    }
  }

  panic!()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day1_part1() {
    let example = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(514579, find_2020_and_multiply(example));
  }

  
  #[test]
  fn day1_part2() {
    let example = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(241861950, find_2020_and_multiply_part2(example));
  }
}