pub struct Passport
{
  byr: bool,
  iyr: bool,
  eyr: bool,
  hgt: bool,
  hcl: bool,
  ecl: bool,
  pid: bool,
  cid: bool
}

fn parse_input(input: &str) -> Vec<Passport> {
  let inputs_collected = input.split("\n\n").collect::<Vec<&str>>();
  let mut passports : Vec<Passport> = Vec::new();

  for val in inputs_collected {
    passports.push(Passport {
      byr: val.contains("byr:"),
      iyr: val.contains("iyr:"),
      eyr: val.contains("eyr:"),
      hgt: val.contains("hgt:"),
      hcl: val.contains("hcl:"),
      ecl: val.contains("ecl:"),
      pid: val.contains("pid:"),
      cid: val.contains("cid:")
    });
  }

  passports
}

fn is_valid_passport(input: &Passport) -> bool {
  input.byr && input.iyr && input.eyr && input.hgt && input.hcl && input.ecl && input.pid
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day4_part1() {
    let input = r##"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"##;

    assert_eq!(2, parse_input(input).iter().map(|x| is_valid_passport(x)).filter(|x| *x).count());
    //assert_eq!(7, hit_trees(input.lines().collect(), 3, 1));
  }
}