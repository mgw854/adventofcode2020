use petgraph::Graph;

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_rule_test() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
    let graph = parse_rule(input);

    assert_eq!(3, graph.node_count());
    assert_eq!(2, graph.edge_count());
  }
}