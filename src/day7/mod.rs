/*use petgraph::{Graph, graphmap::GraphMap, graphmap::DiGraphMap};

fn parse_rule(input: &str) -> Graph<String, u64> {
  let mut graph = Graph::<String, u64>::new();

  let sides = input.split(" bags contain ").collect::<Vec<&str>>();

  let main_bag = graph.add_node(sides[0].to_string());
  let other_bags = sides[1].split(", ").map(|s| s.replace("bags", "").replace("bag", "").replace(".", "").trim().to_string());

  for bag in other_bags {
    let rule = bag.splitn(2, " ").collect::<Vec<&str>>();
    dbg!(rule[0]);
    let count = rule[0].parse::<u64>().unwrap();
    let node = graph.add_node(rule[1].to_string());
    graph.add_edge(main_bag, node, count);
  }

  graph
}


fn parse_rule(input: &str) -> DiGraphMap<String, u64> {
  let mut graph = DiGraphMap::<String, u64>::new();

  let sides = input.split(" bags contain ").collect::<Vec<&str>>();

  let main_bag = graph.add_node(sides[0].to_string());
  let other_bags = sides[1].split(", ").map(|s| s.replace("bags", "").replace("bag", "").replace(".", "").trim().to_string());

  for bag in other_bags {
    if bag.contains("no other") {
      break;
    }

    let rule = bag.splitn(2, " ").collect::<Vec<&str>>();
    let count = rule[0].parse::<u64>().unwrap();
    let node = graph.add_node(rule[1].to_string());
    graph.add_edge(main_bag, node, count);
  }

  graph
}*/

use std::collections::HashMap;

pub fn get_answer(input: &str) -> u64 {
  let rules = merge_rules(input.lines().map(|l| parse_rule_map(l)).collect());

  find_eventual_holders("shiny gold", &rules)
}

fn parse_rule_map(input: &str) -> HashMap<String, Vec<Rule>> {
  let mut map = HashMap::<String, Vec<Rule>>::new();
  
  let sides = input.split(" bags contain ").collect::<Vec<&str>>();

  let mut rules = Vec::<Rule>::new();

  let other_bags = sides[1].split(", ").map(|s| s.replace("bags", "").replace("bag", "").replace(".", "").trim().to_string());

  for bag in other_bags {
    if bag.contains("no other") {
      break;
    }

    let rule = bag.splitn(2, " ").collect::<Vec<&str>>();
    let count = rule[0].parse::<u64>().unwrap();
    rules.push(Rule { count, color: rule[1].to_string() });
  }

  map.insert(sides[0].to_string(), rules);

  map
}

fn merge_rules(input: Vec<HashMap<String, Vec<Rule>>>) -> HashMap<String, Vec<Rule>> {
  let mut master_hash = HashMap::new();

  for map in input {
    for (k, mut v) in map {
      master_hash.entry(k).or_insert(Vec::new()).append(&mut v);
    }
  }

  master_hash
}

fn find_eventual_holders(color: &str, rules: &HashMap<String, Vec<Rule>>) -> u64 {
  let mut count = 0;

  let mut seen: HashMap<String, ()> = HashMap::new();

  for (k, v) in rules.iter() {
    if !seen.contains_key(k) && v.iter().any(|r| r.color == color) {
      seen.insert(k.to_string(), ());
      count += 1;
      count += find_eventual_holders_recursive(&k, &mut seen, rules);
    }
  }

  count
}

fn find_eventual_holders_recursive(color: &str, mut seen: &mut HashMap<String, ()>, rules: &HashMap<String, Vec<Rule>>) -> u64 {
  let mut count = 0;
  for (k, v) in rules.iter() {
    if !seen.contains_key(k) && v.iter().any(|r| r.color == color) {
      seen.insert(k.to_string(), ());
      count += 1;
      count += find_eventual_holders_recursive(&k, &mut seen, rules);
    }
  }

  count
}

#[derive(Debug)]
pub struct Rule {
  count: u64,
  color: String
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_rule_test() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
    let map = parse_rule_map(input);

    dbg!(map);
  }

  #[test]
  fn day7_part1() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    let rules = merge_rules(input.lines().map(|l| parse_rule_map(l)).collect());

    assert_eq!(4, find_eventual_holders("shiny gold", &rules));
  }
}