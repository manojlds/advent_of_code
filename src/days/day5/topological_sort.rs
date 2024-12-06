use std::collections::HashMap;
use std::collections::HashSet;

pub fn topological_sort(orderings: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut in_degree = HashMap::new();
    let mut graph = HashMap::new();
    let mut sorted = Vec::new();

    for (&key, values) in orderings.iter() {
        graph.entry(key).or_insert(Vec::new()).extend(values.clone());
        in_degree.entry(key).or_insert(0);
        for &value in values {
            *in_degree.entry(value).or_insert(0) += 1;
        }
    }

    let mut queue: Vec<i32> = in_degree.iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&node, _)| node)
        .collect();

    while let Some(node) = queue.pop() {
        sorted.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push(neighbor);
                    }
                }
            }
        }
    }

    if sorted.len() != in_degree.len() {
        println!("Cycle detected in the orderings!");
        let sorted_set: HashSet<_> = sorted.iter().cloned().collect();
        let all_nodes: HashSet<_> = in_degree.keys().cloned().collect();

        let missing_nodes: Vec<_> = all_nodes.difference(&sorted_set).cloned().collect();
        println!("Nodes missing from sorted order (in cycle): {:?}", missing_nodes);

        panic!("Cannot complete topological sort due to cycle.");
    }

    sorted
}