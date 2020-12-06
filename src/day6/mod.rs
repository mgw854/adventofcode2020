use std::collections::HashMap;

pub fn count_answers(answers: Vec<Vec<Vec<char>>>) -> usize {
  let mut count = 0;
  for group in answers {
    count += group.iter().flatten().map(|x| (*x, ())).collect::<HashMap::<char, ()>>().len();
  }

  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day6_part1() {
    let input = vec![vec![vec!['a', 'b', 'c']], vec![vec!['a'], vec!['b'], vec!['c']], vec![vec!['a', 'b'], vec!['a', 'c']], vec![vec!['a'], vec!['a'], vec!['a'], vec!['a']], vec![vec!['b']]];

    assert_eq!(11, count_answers(input));
  }
}