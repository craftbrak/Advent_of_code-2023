use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::{io, thread};
use std::io::{BufReader, Read};

fn main() {
    println!("part one results: {}", part_one("input"));
    println!("part two result: {}",part_two_recursive("input"))

}
struct Card{
    id: i32,
    winning: Vec<i32>,
    values: Vec<i32>,
    points: i32,
    wins: i32,
}
fn part_one(file_path: &str)->i32{
    let mut sum = 0;
    if let Ok(pileofcard) = read_file(file_path){
        let cards :Vec<Card> =pileofcard.lines().map(|line| parse_card(line)).collect();
        for mut card in cards {
            calculate_cart_points(&mut card);
            // println!("card {} has {} points",card.id,card.points);
            sum += card.points;
        }
    }

    return sum
}
fn part_two_recursive(file_path: &str)->i32{
    let mut sum = 0;
    if let Ok(pileofcard) = read_file(file_path){
        let mut  cards :Vec<Card> =pileofcard.lines().map(|line| parse_card(line)).collect();
        for card in &mut cards {
            calculate_cart_points(card);
        }
        for card in &cards {
           sum += resolve_card(&card,&mut 0, &cards);
            sum+=1;
        }

    }
    return sum
}
fn part_two(file_path: &str) -> i32 {
    if let Ok(pileofcard) = read_file(file_path){
        let mut card_counts = HashMap::new();
        let mut cards :Vec<Card> =pileofcard.lines().map(|line| parse_card(line)).collect();
        // Initialize the map with each card having a count of 1
        for card in &mut cards {
            *card_counts.entry(card.id).or_insert(0) += 1;
            calculate_cart_points(card)
        }

        let mut processed = HashSet::new();
        while let Some((&card_id, &count)) = card_counts.iter().find(|&(&id, _)| !processed.contains(&id)) {
            processed.insert(card_id);

            if let Some( card) = cards.iter().find(|&c| c.id == card_id) {

                // Add subsequent cards based on the number of wins
                for i in 1..=card.wins {
                    let next_card_id = card_id + i;
                    *card_counts.entry(next_card_id).or_insert(0) += count;
                }
            }
        }

        // Sum up all counts
        return card_counts.values().sum()
    }
    return 0


}
fn calculate_cart_points(card: &mut Card){
    let mut values: HashSet<&i32> = HashSet::from_iter(&(card.values));
    for value in &card.winning {
        if !values.insert(&value) {
            if card.points == 0 { card.points= 1 }
            else { card.points*=2};
            card.wins+=1;
        }
    }
}
fn resolve_card(card: &Card, sum: &mut i32, cards : &Vec<Card>) -> i32{
    if card.wins ==0  { return *sum }
    let subcards :Vec<&Card>= cards.iter().skip((card.id) as usize).take(card.wins as usize).collect();
    *sum+=subcards.len()as i32;
    for subcard in subcards {
        resolve_card(subcard,sum,cards);
    }

    return *sum
}
fn parse_card(line: &str)->Card{
    let parts: Vec<&str> = line.split(": ").collect();
    let id_str = *parts.get(0).unwrap();
    let numbers_str = *parts.get(1).unwrap();
    let id :i32=id_str.split_whitespace().last().unwrap().parse().unwrap();

    let number_parts: Vec<&str> = numbers_str.split('|').collect();
    let winning: Vec<i32> = number_parts[0].split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    let values: Vec<i32> = number_parts[1].split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    return Card {
        id ,
        winning,
        values,
        points: 0,
        wins:0
    }
}

fn read_file(file_path: &str)-> io::Result<String>{
    let file = File::open(file_path)?;
    let mut bur_reader = BufReader::new(file);
    let mut contents = String::new();
    bur_reader.read_to_string(&mut contents)?;
    Ok(contents)
}