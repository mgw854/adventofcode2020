use std::collections::HashMap;
use itertools::Itertools;

fn sort_jolts(adapters: &Vec<u64>) -> Vec<u64> {
  let mut remaining : Vec<u64> = adapters.clone();
  remaining.sort();
  dbg!(&remaining);
  // First, try to solve the end. Descend as far as we can 
  let mut end_rev : Vec<u64> = Vec::new();

  let max_jolt = remaining.last().copied().unwrap();

  end_rev.push(max_jolt + 3); // Our device always has an effective joltage of max+3
  end_rev.push(max_jolt);
  remaining.pop();

  let mut current_jolt = max_jolt;

  loop {
    let nearest = remaining.iter().filter(|x| (current_jolt - **x) <= 3).map(|x| *x).collect::<Vec<u64>>();

    if nearest.len() == 1 {
      current_jolt = nearest[0];
      end_rev.push(current_jolt);
      remaining.remove(remaining.binary_search(&current_jolt).unwrap());
    } else if nearest.len() == 0 {
      panic!();
    }else {
      // If we got here, that's as much back-tracking as we can do. Go forward now.
      break;
    }
  }

  end_rev.reverse();

  let mut tree : Vec<u64> = Vec::new();
  current_jolt = 0;
  tree.push(current_jolt);

  while remaining.len() > 0 {
    // Now, start at the top and descend
    let nearest = remaining.iter().filter(|x| **x > current_jolt && (**x - current_jolt) <= 3).map(|x| *x).collect::<Vec<u64>>();

    if nearest.len() == 1 {
      current_jolt = nearest[0];
      tree.push(current_jolt);
      remaining.remove(remaining.binary_search(&current_jolt).unwrap());
    } else if nearest.len() == 0 {
      panic!("0 choices");
    } else {
      // We have multiple choices; descend as far as we can go
      let mut found = false;
      for opt in nearest {
        if let Some(xlist) = sort_jolts_recursively_descend(opt, end_rev[0], remaining.clone()) {
          for x in xlist {
            tree.push(x);
            remaining.remove(remaining.binary_search(&x).unwrap());
          }

          found = true;

          break;
        }
      }

      if !found {
        panic!();
      }
    }
  }

  tree.append(&mut end_rev);

  tree
}

fn sort_jolts_recursively_descend(current_choice: u64, must_match_end: u64, mut remaining : Vec<u64>) -> Option<Vec<u64>> {
  let mut assumptions: Vec<u64> = Vec::new();

  assumptions.push(current_choice);
  remaining.remove(remaining.binary_search(&current_choice).unwrap());

  let mut current_jolt = current_choice;
  dbg!(&must_match_end);
  dbg!(current_choice);

  loop {
    if remaining.len() == 0 {
      return Some(assumptions);
    }

    let nearest = remaining.iter().filter(|x| **x > current_jolt && (**x - current_jolt) <= 3).map(|x| *x).collect::<Vec<u64>>();

    if nearest.len() == 1 {
      current_jolt = nearest[0];
      assumptions.push(current_jolt);
      remaining.remove(remaining.binary_search(&current_jolt).unwrap());
    } else if nearest.len() == 0 {
      return None;
    } else {
      // We have multiple choices; descend as far as we can go
      let mut found = false;
      for opt in nearest {
        if let Some(xlist) = sort_jolts_recursively_descend(opt, must_match_end, remaining.clone()) {
          for x in xlist {
            assumptions.push(x);
            remaining.remove(remaining.binary_search(&x).unwrap());
          }
          
          found = true;
          break;
        }
      }

      if !found {
        return None;
      }
    }
  }
}

fn count_diffs(sorted_jolts: Vec<u64>) -> u64 {
  let mut ones = 0;
  let mut threes = 0;

  for window in sorted_jolts.windows(2) {
    let diff = window[1] - window[0];

    match diff {
      1 => { ones += 1; },
      3 => { threes += 1; },
      _ => {}
    }
  }

  ones * threes
}

pub fn solve(inputs: &Vec<u64>) -> u64 {
  sort_jolts_all_opts(inputs)
}

fn sort_jolts_all_opts(adapters: &Vec<u64>) -> u64 {
  let mut remaining : Vec<u64> = adapters.clone();
  remaining.sort();
  //dbg!(&remaining);
  // First, try to solve the end. Descend as far as we can 
  let max_jolt = remaining.last().copied().unwrap();

  remaining.pop();

  let mut current_jolt = max_jolt;

  loop {
    let nearest = remaining.iter().filter(|x| (current_jolt - **x) <= 3).map(|x| *x).collect::<Vec<u64>>();

    if nearest.len() == 1 {
      current_jolt = nearest[0];
      remaining.remove(remaining.binary_search(&current_jolt).unwrap());
    } else if nearest.len() == 0 {
      panic!();
    }else {
      // If we got here, that's as much back-tracking as we can do. Go forward now.
      break;
    }
  }
  
  let mut cache : HashMap<u64, u64> = HashMap::new();

  jolt_descend_all(0, current_jolt, remaining, &mut cache)
}
use std::collections::hash_map::Entry;

fn jolt_descend_all(lower_bound: u64, upper_bound: u64, mut remaining: Vec<u64>, mut cache: &mut HashMap<u64, u64>) -> u64 {
  let mut current_jolt = lower_bound;
  let mut count = 1;

  loop {
    if remaining.len() == 0 {
      //dbg!("EOL");
      return count;
    }

    let nearest = remaining.iter().filter(|x| **x > current_jolt && (**x - current_jolt) <= 3).map(|x| *x).collect::<Vec<u64>>();

    if nearest.len() == 1 {
      //dbg!(current_jolt);
      if (upper_bound - current_jolt) <= 3 {
        count += 1;
      }

      current_jolt = nearest[0];
      remaining.remove(remaining.binary_search(&current_jolt).unwrap());
    } else if nearest.len() == 0 {
      //dbg!(current_jolt);
      if (upper_bound - current_jolt) <= 3 {
        //count += 1;
      }
      return count;
    } else {
      // We have multiple choices; descend as far as we can go
      //dbg!(remaining.iter().filter(|x| **x > current_jolt && (**x - current_jolt) <= 3).map(|x| *x).collect::<Vec<u64>>());
      //dbg!(current_jolt);
      let mut opt_count = 0;
      for opt in nearest {

        if let Entry::Occupied(e) = cache.entry(opt) {
          opt_count += e.get();
        }
        else {
          let mut potential = remaining.clone();
          potential.remove(potential.binary_search(&opt).unwrap());
  
          let c = jolt_descend_all(opt, upper_bound, potential, &mut cache);
          cache.insert(opt, c);
          opt_count += c;
        }
      }

      if (upper_bound - current_jolt) <= 3 {
        opt_count += 1;
      }

      //dbg!(opt_count);
      return count * opt_count;
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  //#[test]
  fn day10_part1() {
    let input = "16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4";

    let jolts : Vec<u64> = input.lines().map(|x| x.trim().parse().unwrap()).collect();

    assert_eq!(35, count_diffs(sort_jolts(&jolts)));
  }

  
 /// #[test]
  fn day10_part1b() {
    let input = "28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3";

    let jolts : Vec<u64> = input.lines().map(|x| x.trim().parse().unwrap()).collect();

    assert_eq!(220, count_diffs(sort_jolts(&jolts)));
  }

  
  #[test]
  fn day10_part2() {
    let input = "16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4";

    let jolts : Vec<u64> = input.lines().map(|x| x.trim().parse().unwrap()).collect();

    assert_eq!(8, sort_jolts_all_opts(&jolts));
  }

    
  #[test]
  fn day10_part2b() {
    let input = "28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3";

    let jolts : Vec<u64> = input.lines().map(|x| x.trim().parse().unwrap()).collect();

    assert_eq!(19208, sort_jolts_all_opts(&jolts));
  }
}