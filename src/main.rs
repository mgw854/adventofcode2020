use std::error::Error;
mod day3;
mod helpers;

fn main()-> Result<(), Box<dyn Error>> { 
  let input = helpers::inputhandling::get_string(3).unwrap();
    
  let attempt1 = day3::hit_trees(input.lines().collect(), 1, 1);
  let attempt2 = day3::hit_trees(input.lines().collect(), 3, 1);
  let attempt3 = day3::hit_trees(input.lines().collect(), 5, 1);
  let attempt4 = day3::hit_trees(input.lines().collect(), 7, 1);
  let attempt5 = day3::hit_trees(input.lines().collect(), 1, 2);

  dbg!(attempt1 * attempt2 * attempt3 * attempt4 * attempt5);
  //dbg!(day2::get_valid_count_part2(input));

  Ok(())
}
