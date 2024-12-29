use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::{HashMap, HashSet};

fn parse_connections(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(newline, separated_pair(alpha1, tag("-"), alpha1))(input)
}

fn parse_network(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut network: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (c1, c2) in parse_connections(input).unwrap().1 {
        network.entry(c1).or_default().insert(c2);
        network.entry(c2).or_default().insert(c1);
    }
    network
}

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
    let network = parse_network(input);
    network
        .keys()
        .combinations(3)
        .filter(|c| {
            network[c[0]].contains(c[1])
                && network[c[0]].contains(c[2])
                && network[c[1]].contains(c[2])
                && (c[0].starts_with('t') || c[1].starts_with('t') || c[2].starts_with('t'))
        })
        .count()
}

fn get_largest_component(
    network: &HashMap<&str, HashSet<&str>>,
    first: &str,
    component: HashSet<&str>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    let key = component.iter().sorted().join(",");
    if let Some(res) = cache.get(&key) {
        return *res;
    }
    let mut best_size = component.len();
    for candidate in network[first].difference(&component) {
        if network[candidate].is_superset(&component) {
            let mut new_component = component.clone();
            new_component.insert(candidate);
            best_size = best_size.max(get_largest_component(network, first, new_component, cache));
        }
    }
    cache.insert(key, best_size);
    best_size
}

#[aoc(day23, part2)]
fn part2(input: &str) -> String {
    let network = parse_network(input);
    let mut cache = HashMap::new();
    network.keys().for_each(|&p| {
        get_largest_component(&network, p, HashSet::from([p]), &mut cache);
    });
    cache.keys().max_by_key(|k| k.len()).unwrap().to_string()
}
