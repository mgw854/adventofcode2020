use std::error::Error;
mod day4;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 
  let input = helpers::inputhandling::get_string(4).unwrap();
   
  dbg!(day4::parse_input(&input).iter().map(|x| day4::is_valid_passport(x)).filter(|x| *x).count());

  Ok(())
}
