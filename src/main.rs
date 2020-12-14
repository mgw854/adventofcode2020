use std::error::Error;
mod day12;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 
  let input = helpers::inputhandling::parse_input_per_line(12, day12::NavigationInstruction::parse).unwrap();
  //input.sort();

  //dbg!(input);

  dbg!(day12::generate_position(&input));
  Ok(())
}
