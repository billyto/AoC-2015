use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn parse_input(input_path: String) -> Result<Vec<String>> {
    let input_contents: String = read_to_string(input_path).context("Could not read input file")?;
    let strings: Vec<String> = input_contents.lines().map(String::from).collect();

    Ok(strings)
}

fn parse_distances(strings: &[String]) -> Result<(HashMap<(String, String), usize>, Vec<String>)> {
    let mut distances = HashMap::new();
    let mut cities = HashSet::new();

    for line in strings {
        let parts: Vec<&str> = line.split(" = ").collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid line format: {}", line));
        }

        let distance: usize = parts[1]
            .parse()
            .with_context(|| format!("Failed to parse distance: {}", parts[1]))?;

        let route_parts: Vec<&str> = parts[0].split(" to ").collect();
        if route_parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid route format: {}", parts[0]));
        }

        let city_a = route_parts[0].to_string();
        let city_b = route_parts[1].to_string();

        // Store both directions since TSP is undirected
        distances.insert((city_a.clone(), city_b.clone()), distance);
        distances.insert((city_b.clone(), city_a.clone()), distance);

        cities.insert(city_a);
        cities.insert(city_b);
    }

    let city_list: Vec<String> = cities.into_iter().collect();
    Ok((distances, city_list))
}

fn find_optimal_route(
    distances: &HashMap<(String, String), usize>,
    cities: &[String],
    find_shortest: bool,
) -> (usize, Vec<String>) {
    let routes = cities.iter().permutations(cities.len()).map(|route| {
        let distance = calculate_route_distance(&route, distances);
        let route_owned: Vec<String> = route.into_iter().cloned().collect();
        (distance, route_owned)
    });

    if find_shortest {
        routes
            .min_by_key(|(distance, _)| *distance)
            .unwrap_or((0, Vec::new()))
    } else {
        routes
            .max_by_key(|(distance, _)| *distance)
            .unwrap_or((0, Vec::new()))
    }
}

fn calculate_route_distance(
    route: &[&String],
    distances: &HashMap<(String, String), usize>,
) -> usize {
    route
        .windows(2)
        .map(|pair| {
            distances
                .get(&(pair[0].clone(), pair[1].clone()))
                .copied()
                .unwrap_or(0)
        })
        .sum()
}

pub fn solve_part1(strings: &[String]) -> usize {
    let (distances, cities) = parse_distances(strings).unwrap();
    // let (shortest_distance, _route) = find_shortest_route(&distances, &cities);
    let (shortest_distance, _route) = find_optimal_route(&distances, &cities, true);

    shortest_distance
}

pub fn solve_part2(strings: &[String]) -> usize {
    let (distances, cities) = parse_distances(strings).unwrap();
    let (longest_distance, _route) = find_optimal_route(&distances, &cities, false);
    longest_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_distances() {
        let input = vec![
            "London to Dublin = 464".to_string(),
            "London to Belfast = 518".to_string(),
            "Dublin to Belfast = 141".to_string(),
        ];

        let (distances, cities) = parse_distances(&input).unwrap();

        assert_eq!(distances.len(), 6); // 3 routes × 2 directions
        assert_eq!(
            *distances
                .get(&("London".to_string(), "Dublin".to_string()))
                .unwrap(),
            464
        );
        assert_eq!(
            *distances
                .get(&("Dublin".to_string(), "London".to_string()))
                .unwrap(),
            464
        );
        assert_eq!(cities.len(), 3);
        assert!(cities.contains(&"London".to_string()));
        assert!(cities.contains(&"Dublin".to_string()));
        assert!(cities.contains(&"Belfast".to_string()));
    }

    #[test]
    fn test_parse_distances_invalid_format() {
        let input = vec!["Invalid line".to_string()];
        assert!(parse_distances(&input).is_err());
    }

    #[test]
    fn test_calculate_route_distance() {
        let mut distances = HashMap::new();
        distances.insert(("A".to_string(), "B".to_string()), 10);
        distances.insert(("B".to_string(), "C".to_string()), 5);

        let city_a = "A".to_string();
        let city_b = "B".to_string();
        let city_c = "C".to_string();
        let route = vec![&city_a, &city_b, &city_c];
        let distance = calculate_route_distance(&route, &distances);

        assert_eq!(distance, 15);
    }

    #[test]
    fn test_find_optimal_route() {
        let mut distances = HashMap::new();
        distances.insert(("A".to_string(), "B".to_string()), 10);
        distances.insert(("B".to_string(), "A".to_string()), 10);
        distances.insert(("A".to_string(), "C".to_string()), 15);
        distances.insert(("C".to_string(), "A".to_string()), 15);
        distances.insert(("B".to_string(), "C".to_string()), 5);
        distances.insert(("C".to_string(), "B".to_string()), 5);

        let cities = vec!["A".to_string(), "B".to_string(), "C".to_string()];

        // Test shortest route
        let (shortest_distance, shortest_route) = find_optimal_route(&distances, &cities, true);
        assert_eq!(shortest_distance, 15); // A->B->C or C->B->A
        assert_eq!(shortest_route.len(), 3);

        // Test longest route
        let (longest_distance, longest_route) = find_optimal_route(&distances, &cities, false);
        assert_eq!(longest_distance, 25); // A->C->B or B->C->A
        assert_eq!(longest_route.len(), 3);
    }
}
