use std::collections::HashMap;

pub fn execute(program: Vec<AsmInstruction>) -> i64 {
  let mut instruction_pointer = 0;
  let mut accum = 0;
  let mut visited : HashMap<usize, ()> = HashMap::new();

  while !visited.contains_key(&instruction_pointer) {
    visited.insert(instruction_pointer, ());

    instruction_pointer = (instruction_pointer as i64 + match program[instruction_pointer] {
      AsmInstruction::NoOp(_) => 1,
      AsmInstruction::Accumulate(x) => { accum += x; 1 },
      AsmInstruction::Jump(x) => x
    }) as usize;
  }

  accum
}

pub fn execute_mutate(program: Vec<AsmInstruction>) -> i64 {
  let mut instruction_pointer = 0;

  loop
  {   
    let mut program2 = program.clone();
    // find the first instruction that is a jmp or nop, and flip it
    for ip in instruction_pointer..program2.len() {
      match program2[ip] {
        AsmInstruction::Accumulate(_) => {},
        AsmInstruction::NoOp(x) => { instruction_pointer = ip + 1; program2[ip] = AsmInstruction::Jump(x); break; },
        AsmInstruction::Jump(x) => { instruction_pointer = ip + 1; program2[ip] = AsmInstruction::NoOp(x); break; }
      }
    }
    
    match execute_mutate_internal(program2) {
      Ok(i) => return i,
      _ => {}
    }
  }
}

fn execute_mutate_internal(program: Vec<AsmInstruction>) -> Result<i64, ()> {
  let mut instruction_pointer = 0;
  let mut accum = 0;
  let mut visited : HashMap<usize, ()> = HashMap::new();

  while !visited.contains_key(&instruction_pointer) && instruction_pointer < program.len() {
    visited.insert(instruction_pointer, ());

    instruction_pointer = (instruction_pointer as i64 + match program[instruction_pointer] {
      AsmInstruction::NoOp(_) => 1,
      AsmInstruction::Accumulate(x) => { accum += x; 1 },
      AsmInstruction::Jump(x) => x
    }) as usize;
  }

  if instruction_pointer >= program.len() {
    return Ok(accum);
  }

  Err(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AsmInstruction {
  NoOp(i64),
  Accumulate(i64),
  Jump(i64)
}

impl AsmInstruction {
  pub fn parse(input: &str) -> AsmInstruction {
    let split = input.split(" ").collect::<Vec<&str>>();
    let arg : i64 = split[1].parse().unwrap();

    match split[0] {
      "nop" => AsmInstruction::NoOp(arg),
      "acc" => AsmInstruction::Accumulate(arg),
      "jmp" => AsmInstruction::Jump(arg),
      _ => panic!()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_nop() {
    assert_eq!(AsmInstruction::NoOp(0), AsmInstruction::parse("nop +0"));
  }

  #[test]
  fn parse_acc() {
    assert_eq!(AsmInstruction::Accumulate(5), AsmInstruction::parse("acc +5"));
  }
  
  #[test]
  fn parse_jmp() {
    assert_eq!(AsmInstruction::Jump(-7), AsmInstruction::parse("jmp -7"));
  }
  
  #[test]
  fn day8_part1() {
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    assert_eq!(5, execute(input.lines().map(|x| AsmInstruction::parse(x)).collect()));
  }

  #[test]
  fn day8_part2() {
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    assert_eq!(8, execute_mutate(input.lines().map(|x| AsmInstruction::parse(x)).collect()));
  }
}