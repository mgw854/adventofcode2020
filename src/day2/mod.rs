use std::str::FromStr;

pub fn get_valid_count(input: Vec<PasswordPolicy>) -> usize
{
  input.iter().map(|x| x.validate()).filter(|x| *x).count()
}

pub fn get_valid_count_part2(input: Vec<PasswordPolicy>) -> usize
{
  input.iter().map(|x| x.validate2()).filter(|x| *x).count()
}

pub struct PasswordPolicy {
  range: std::ops::RangeInclusive<usize>,
  c: char,
  password: String
}

impl FromStr for PasswordPolicy {
  type Err = Box<dyn std::error::Error>;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    // 1-2 a: abcde

    let parts = input.split(" ").collect::<Vec<&str>>();
    let range = parts[0].split("-").collect::<Vec<&str>>();

    let range_start : usize  = range[0].parse::<usize>()?;
    let range_end : usize = range[1].parse::<usize>()?;

    let c = parts[1].chars().next().unwrap();

    Ok(PasswordPolicy {
      range: std::ops::RangeInclusive::new(range_start, range_end),
      c: c,
      password: parts[2].to_owned()
    })
  }
}

impl PasswordPolicy {
  pub fn validate(&self) -> bool {
    let hits = self.password.chars().filter(|&x| x == self.c).count();
    self.range.contains(&hits)
  }

  pub fn validate2(&self) -> bool {
    let cs : Vec<char> = self.password.chars().collect();
    let first = cs[*self.range.start() - 1];
    let second : char = cs[*self.range.end() - 1];

    (first == self.c && second != self.c) ||
    (first != self.c && second == self.c)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day2_part1() {
    let example = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
    assert_eq!(2, get_valid_count(example.iter().map(|x| PasswordPolicy::from_str(x).unwrap()).collect()));
  }

  #[test]
  fn day2_part2() {
    let example = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
    assert_eq!(1, get_valid_count_part2(example.iter().map(|x| PasswordPolicy::from_str(x).unwrap()).collect()));
  }
}