use std::error::Error;
use itertools::{Itertools, diff_with, Diff};
mod day5;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 
  let mut input = helpers::inputhandling::parse_input_per_line(5, |x| Ok(day5::get_seat_id(x))).unwrap();
   
  input.sort();
  let min = *input.iter().min().unwrap();
  let max = *input.iter().max().unwrap();

  if let Some(d) = diff_with(min..=max, input.iter(), |i, j| i == *j) {
    match d {
      Diff::FirstMismatch(_, i, _) => { dbg!(i); },
      Diff::Shorter(_, i) => { dbg!(i); },
      Diff::Longer(_, j) => { dbg!(j); }
    }
  }

  Ok(())
}
