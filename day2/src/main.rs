use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::num::ParseIntError;

struct Game{
   id: i32,
   max_red: i32,
   max_green: i32,
   max_bleu: i32
}

fn main() {
    println!("Hello, world!");
    let source = read_file("./input.txt").unwrap();
    let games =parse_game(&source).unwrap();
    let mut sum = 0;
    for game in games {
        // println!("games {} , max red {}, max green {}, max_blue {}",game.id, game.max_red,game.max_green , game.max_bleu);
        if game.max_red > 12 || game.max_green > 13 || game.max_bleu >14 {
            continue;
        }
        else {
            println!("game : {} is possible (max red {}, max green {}, max_blue {})", game.id,game.max_red,game.max_green , game.max_bleu);
            sum+=game.id;
        }
    }
    println!("the awnser is : {}",sum);
}

fn read_file(file_path: &str)-> io::Result<String>{
    let file = File::open(file_path)?;
    let mut bur_reader = BufReader::new(file);
    let mut contents = String::new();
    bur_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
fn parse_game(input: &str)->Option<Vec<Game>> {
    let mut games :Vec<Game>=Vec::new();
    for line in input.lines() {
        let max_colors = extract_game_max_colors(&line).unwrap();
        let game = Game{
            id: extract_game_id(&line).unwrap(),
            max_red:max_colors.0,
            max_green:max_colors.1,
            max_bleu:max_colors.2
        };
        games.push(game)
    }

    return Some(games);
}
fn extract_game_id(game_string: &str)->Result<i32, Option<ParseIntError>>{
    let parts : Vec<&str> = game_string.split(": ").collect();
    let game_id_str= parts[0].split_whitespace().nth(1);
    match game_id_str {
        None => {Err(None)}
        Some(s) => {
            let id= s.parse::<i32>();
            return match id {
                Ok(i) => { Ok(i) }
                Err(e) => { Err(Some(e)) }
            }
        }
    }

}
fn extract_game_max_colors(game_string: &str)-> Result<(i32,i32,i32),Option<ParseIntError>>{
    let game:Vec<&str>=game_string.split(";").collect();
    let mut max_red = 0;
    let mut max_blue = 0;
    let mut max_green = 0;

    for pull in game {
        let colors: Vec<&str> = pull.split(", ").collect();
        for mut color in colors {
            if let Some(colon_index) = color.find(':'){
                color = &color[colon_index+1..].trim();
            }
            let parts: Vec<&str> = color.split_whitespace().collect();
            if let (Some(number_str), Some(color_str)) = (parts.get(0), parts.get(1) ){
                // println!("{} {}",color_str, number_str );
                match *color_str {
                    "red" => max_red = max_red.max(number_str.parse::<i32>().unwrap_or(0)),
                    "blue" => max_blue = max_blue.max(number_str.parse::<i32>().unwrap_or(0)),
                    "green" => max_green = max_green.max(number_str.parse::<i32>().unwrap_or(0)),
                    _ => {}
                }
            }
        }
    }
    return Ok((max_red,max_green,max_blue));
}
