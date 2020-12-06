use std::collections::HashMap;

pub fn count_any_answers(answers: Vec<Vec<Vec<char>>>) -> usize {
  let mut count = 0;
  for group in answers {
    count += group.iter().flatten().map(|x| (*x, ())).collect::<HashMap::<char, ()>>().len();
  }

  count
}

pub fn count_all_answers(answers: Vec<Vec<Vec<char>>>) -> usize {
  let mut count = 0;
  for group in answers {
    count += group.iter().flatten().fold(HashMap::<char, usize>::new(), |mut accum, x| { *accum.entry(*x).or_insert(0) += 1; accum }).iter().filter(|(c, l)| **l == group.len()).count();
  }

  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day6_part1() {
    let input = vec![vec![vec!['a', 'b', 'c']], vec![vec!['a'], vec!['b'], vec!['c']], vec![vec!['a', 'b'], vec!['a', 'c']], vec![vec!['a'], vec!['a'], vec!['a'], vec!['a']], vec![vec!['b']]];

    assert_eq!(11, count_any_answers(input));
  }

  #[test]
  fn day6_part2() {
    let input = vec![vec![vec!['a', 'b', 'c']], vec![vec!['a'], vec!['b'], vec!['c']], vec![vec!['a', 'b'], vec!['a', 'c']], vec![vec!['a'], vec!['a'], vec!['a'], vec!['a']], vec![vec!['b']]];

    assert_eq!(6, count_all_answers(input));
  }
}