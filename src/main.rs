use std::error::Error;
mod day10;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 
  let input = helpers::inputhandling::parse_input_per_line(10, |l| Ok(l.parse::<u64>().unwrap())).unwrap();

  dbg!(day10::solve(&input));
  Ok(())
}
