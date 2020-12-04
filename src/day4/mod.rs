use regex::RegexBuilder;

#[derive(Debug)]
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

pub fn parse_input(input: &str) -> Vec<Passport> {
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

pub fn parse_and_validate_input(input: &str) -> Vec<Passport> {
  let inputs_collected = input.split("\n\n").collect::<Vec<&str>>();
  let mut passports : Vec<Passport> = Vec::new();

  let byr_regex = RegexBuilder::new(r#"^byr:(\d{4})$"#).multi_line(true).build().unwrap();
  let iyr_regex = RegexBuilder::new(r#"^iyr:(\d{4})$"#).multi_line(true).build().unwrap();
  let eyr_regex = RegexBuilder::new(r#"^eyr:(\d{4})$"#).multi_line(true).build().unwrap();
  let ecl_regex = RegexBuilder::new(r#"^ecl:(amb|blu|brn|gry|grn|hzl|oth)$"#).multi_line(true).build().unwrap();
  let hcl_regex = RegexBuilder::new(r#"^hcl:#[0-9a-f]{6}$"#).multi_line(true).build().unwrap();
  let pid_regex = RegexBuilder::new(r#"^pid:\d{9}$"#).multi_line(true).build().unwrap();
  let hgt_regex = RegexBuilder::new(r#"^hgt:(\d+)(cm|in)$"#).multi_line(true).build().unwrap();

  for val in inputs_collected {
    let v = val.replace(" ", "\n") + "\n";

    passports.push(Passport {
      byr: match byr_regex.captures(&v) {
        Some(x) => (1920..=2002).contains( &x.get(1).unwrap().as_str().parse::<i32>().unwrap()),
        None => false
      },
      iyr: match iyr_regex.captures(&v) {
        Some(x) => (2010..=2020).contains( &x.get(1).unwrap().as_str().parse::<i32>().unwrap()),
        None => false
      },
      eyr: match eyr_regex.captures(&v) {
        Some(x) => (2020..=2030).contains( &x.get(1).unwrap().as_str().parse::<i32>().unwrap()),
        None => false
      },
      hgt: match hgt_regex.captures(&v) {
        Some(x) => validate_height(x.get(1).unwrap().as_str().parse::<i32>().unwrap(), x.get(2).unwrap().as_str()),
        None => false
      },
      hcl: hcl_regex.is_match(&v),
      ecl: ecl_regex.is_match(&v),
      pid: pid_regex.is_match(&v),
      cid: val.contains("cid:")
    });
  }

  passports
}

pub fn is_valid_passport(input: &Passport) -> bool {
  input.byr && input.iyr && input.eyr && input.hgt && input.hcl && input.ecl && input.pid
}

fn validate_height(height: i32, unit: &str) -> bool {
  match unit {
    "in" => (59..=76).contains(&height),
    "cm" => (150..=193).contains(&height),
    _ => false
  }
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
  }

  #[test]
  fn day4_part2_valid() {
    let input = r##"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"##;

    assert_eq!(4, parse_and_validate_input(input).iter().map(|x| is_valid_passport(x)).filter(|x| *x).count());
  }
  #[test]
  fn day4_part2_invalid() {
    let input = r##"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"##;

    assert_eq!(0, parse_and_validate_input(input).iter().map(|x| is_valid_passport(x)).filter(|x| *x).count());
  }
}