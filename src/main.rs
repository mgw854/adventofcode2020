use std::error::Error;
mod day3;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 

  //let input : Vec<day2::PasswordPolicy> = helpers::inputhandling::parse_input_per_line(2, |s| s.parse::<day2::PasswordPolicy>().map_err(|e| e.into()))?;

  //dbg!(day2::get_valid_count(input));
  //dbg!(day2::get_valid_count_part2(input));

  Ok(())
}
