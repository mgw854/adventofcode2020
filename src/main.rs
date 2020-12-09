use std::error::Error;
mod day8;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 
  let input = helpers::inputhandling::parse_input_per_line(8, |l| Ok(day8::AsmInstruction::parse(l))).unwrap();

  dbg!(day8::execute_mutate(input));
  Ok(())
}
