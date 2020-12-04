use std::error::Error;
mod day4;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 
  let input = helpers::inputhandling::get_string(3).unwrap();
   
  //dbg!(day2::get_valid_count_part2(input));

  Ok(())
}
