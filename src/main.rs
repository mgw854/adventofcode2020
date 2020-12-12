use std::error::Error;
mod day10;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 
  let mut input = helpers::inputhandling::parse_input_per_line(10, |l| Ok(l.parse::<u64>().unwrap())).unwrap();
  input.sort();

  //dbg!(input);

  dbg!(day10::solve(&input));
  Ok(())
}
