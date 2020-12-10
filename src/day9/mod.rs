use itertools::Itertools;

pub fn find_invalid_number(input: Vec<i32>, preamble: usize) -> i32
{
  let mut start = preamble;

  while start < input.len() {
    let preamble_sums : Vec<i32> = input[(start-preamble)..start].iter().map(|x| *x).collect::<Vec<i32>>().iter().combinations(2).map(|x| x[0] + x[1]).collect();

    if !preamble_sums.contains(&input[start]) {
      return input[start];
    }

    start += 1;
  }

  panic!()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day9_part1() {
    let input = "35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576";

    assert_eq!(127, find_invalid_number(input.lines().map(|x| x.trim().parse().unwrap()).collect(), 5));
  }
}