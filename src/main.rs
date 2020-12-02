use std::error::Error;
mod day2;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 

  let input : Vec<day2::PasswordPolicy> = helpers::inputhandling::parse_input_per_line(2, |s| s.parse::<day2::PasswordPolicy>().map_err(|e| e.into()))?;

  dbg!(day2::get_valid_count(input));
  //dbg!(day1::find_2020_and_multiply_part2(input.clone()));

  Ok(())
}
