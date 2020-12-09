use std::error::Error;
mod day8;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 
  let mut input = helpers::inputhandling::get_string(8).unwrap();

  //dbg!(day7::get_answer(&input));
  Ok(())
}
