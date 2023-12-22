use std::collections::HashMap;
use num::integer::lcm;
use common_utils::read_file;

struct Map{
    instructions:Vec<Direction>,
    nodes: HashMap< String ,(String,String)>,
    steps:u32
}
enum Direction {
    R,
    L
}
fn main() {
    let str = read_file("day8/input").unwrap();
    println!("Part 1 : {}",part_1(str.clone()));
    println!("Part 2 : {}",part2_lcm(str));
}
fn part_1 (string: String) -> u32 {
    let mut map = parse_input(string.as_str());
    let mut cur = map.nodes.get_key_value("AAA").unwrap();
    while cur.0 != "ZZZ" {
        cur =match map.instructions.get((map.steps % map.instructions.len() as u32) as usize ).unwrap() {
            Direction::L => {map.nodes.get_key_value(&cur.1.0).unwrap()}
            Direction::R => {map.nodes.get_key_value(&cur.1.1).unwrap()}
        };
        map.steps +=1;
    }
    return map.steps;
}
fn part2_lcm(string: String) -> u64 {
    let map = parse_input(string.as_str());
    let cur = map.nodes.iter().filter(|x| {
        x.0.chars().last().unwrap() == 'A'
    }).collect::<Vec<(&String ,&(String,String))>>();
    let mut  loop_len : Vec<u64>= Vec::new();
    for (i, (key, val)) in cur.iter().enumerate() {
        loop_len.insert(i, 0);
        let mut current = map.nodes.get_key_value(*key).unwrap(); // Get the initial node

        while !current.0.chars().last().eq(&Some('Z')) {
            // Calculate the next step based on current instruction and update the current node
            current = match map.instructions.get((loop_len[i] % map.instructions.len() as u64) as usize).unwrap() {
                Direction::L => map.nodes.get_key_value(&current.1.0).unwrap(),
                Direction::R => map.nodes.get_key_value(&current.1.1).unwrap(),
            };

            loop_len[i] += 1; // Increment loop length for this path
        }
    }
    let mut all_lcm = loop_len[0];
    for &len in &loop_len {
        all_lcm = lcm(all_lcm, len )
    }
    all_lcm
}
// fn gcd(a: usize, b: usize) -> usize {
//     if b == 0 {
//         a
//     } else {
//         gcd(b, a % b)
//     }
// }
//
// fn lcm(a: usize, b: usize) -> usize {
//     a / gcd(a, b) * b // Ensuring no overflow with integer division
// }
// fn lcm_of_array(numbers: Vec<usize>) -> usize {
//     numbers.iter().cloned().reduce(lcm).unwrap_or(1)
// }

fn part_2(string: String) -> u32{
    let mut map = parse_input(string.as_str());
    let mut cur = map.nodes.iter().filter(|x| {
        x.0.chars().last().unwrap() == 'A'
    }).collect::<Vec<(&String ,&(String,String))>>();
    let mut all= false;
    while !all {
        let mut new_cur = Vec::new();

        for (_node, (left, right)) in &cur {
            let next_node = match map.instructions.get(map.steps as usize % map.instructions.len()).unwrap() {
                Direction::R => map.nodes.get_key_value(right).unwrap(),
                Direction::L => map.nodes.get_key_value(left).unwrap(),
            };
            new_cur.push(next_node);
        }

        cur = new_cur; // Replace old cur with the new one
        map.steps += 1;
        all = all_z(&cur); // Check if all nodes end with 'Z'
    }
    map.steps
}
fn all_z( cur: &[(&String, &(String, String))] )-> bool{
    cur.iter().all(|x| x.0.chars().last() == Some('Z'))
}
fn parse_input(str:&str) -> Map {
    let parts = str.split("\n\n").collect::<Vec<&str>>();
    let instructions = parts.get(0).unwrap().chars().map(|x| {if x =='R' { Direction::R } else { Direction::L }}).collect();
    let nodes :HashMap<String, (String, String)> = parts.get(1).unwrap().lines().filter_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect(); // Split the line by whitespace
        if parts.len() == 4 {
            // Extract key and values, removing parentheses
            let key = parts[0].to_string();
            let value1 = parts[2].trim_start_matches('(').trim_end_matches(',').to_string(); // Remove the comma at the end
            let value2 = parts[3].trim_start_matches('(').trim_end_matches(')').to_string(); // Remove the parentheses

            Some((key, (value1, value2)))
        } else {
            None // Skip lines that don't fit the expected format
        }
    }).collect();
    Map{
        instructions, nodes, steps: 0
    }
}

#[cfg(test)]
mod test{
    use common_utils::read_file;
    use crate::{part2_lcm, part_1, part_2};

    #[test]
    fn  test_part1_exemple1(){
        let input = read_file("exemple1").unwrap();
        let res = part_1(input);
        assert_eq!(res,2);
    }
    #[test]
    fn  test_part1_exemple2(){
        let input = read_file("exemple2").unwrap();
        let res = part_1(input);
        assert_eq!(res,6);
    }
    #[test]
    fn  test_part2_exemple1(){
        let input = read_file("exemplep2").unwrap();
        let res = part2_lcm(input);
        assert_eq!(res,6);
    }
}