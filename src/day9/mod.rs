use itertools::Itertools;

pub fn find_invalid_number(input: Vec<u64>, preamble: usize) -> u64
{
  let mut start = preamble;

  while start < input.len() {
    let preamble_sums : Vec<u64> = input[(start-preamble)..start].iter().map(|x| *x).collect::<Vec<u64>>().iter().combinations(2).map(|x| x[0] + x[1]).collect();

    if !preamble_sums.contains(&input[start]) {
      return input[start];
    }

    start += 1;
  }

  panic!()
}

pub fn find_contiguous_set(input: Vec<u64>, sum: u64) -> u64
{
  let mut window_size = input.len();

  while window_size > 2 {
    for window in input.windows(window_size).filter(|w| !w.iter().any(|x| *x > sum)) {
      if window.iter().sum::<u64>() == sum {
        return window.iter().min().unwrap() + window.iter().max().unwrap();
      }
    }

    window_size -= 1;
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

  #[test]
  fn day9_part2() {
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

    assert_eq!(62, find_contiguous_set(input.lines().map(|x| x.trim().parse().unwrap()).collect(), 127));
  }
}