use std::error::Error;
mod day1;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 

  let input : Vec<i32> = helpers::inputhandling::parse_input_per_line(1, |s| s.parse::<i32>().map_err(|e| e.into()))?;

  dbg!(day1::find_2020_and_multiply_part1(input.clone()));
  dbg!(day1::find_2020_and_multiply_part2(input.clone()));

  Ok(())
}
