use std::error::Error;
mod day11;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 
  let mut input = helpers::inputhandling::get_string(11).unwrap();
  //input.sort();

  //dbg!(input);

  dbg!(day11::solve(&input));
  Ok(())
}
