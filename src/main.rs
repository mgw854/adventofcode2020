use std::error::Error;
mod day9;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 
  let input = helpers::inputhandling::parse_input_per_line(9, |l| Ok(l.parse::<u64>().unwrap())).unwrap();

  let invalid = dbg!(day9::find_invalid_number(input.clone(), 25));
  dbg!(day9::find_contiguous_set(input, invalid));
  Ok(())
}
