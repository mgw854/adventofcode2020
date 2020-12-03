use std::error::Error;
mod day3;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 
  let input = helpers::inputhandling::get_string(3).unwrap();
    
  dbg!(day3::hit_trees(input.lines().collect(), 3, 1));
  //dbg!(day2::get_valid_count_part2(input));

  Ok(())
}
