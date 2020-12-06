use std::error::Error;
mod day5;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 
  let input = helpers::inputhandling::parse_input_per_line(5, |x| Ok(day5::get_seat_id(x))).unwrap();
   
  dbg!(input.iter().max());
  //dbg!(day4::parse_and_validate_input(&input).iter().map(|x| day4::is_valid_passport(x)).filter(|x| *x).count());

  Ok(())
}
