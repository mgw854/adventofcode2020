use std::error::Error;
mod day6;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 
  let mut input = helpers::inputhandling::parse_input_between_blank_lines(6, |x| Ok(x.chars().collect::<Vec<char>>())).unwrap();

  dbg!(day6::count_all_answers(input));
  Ok(())
}
