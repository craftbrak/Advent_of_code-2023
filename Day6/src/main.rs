use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

fn main() {
    println!("Hello, world! { }", part_one());

}
struct Race{
    time: i32,
    distance: i32
}
fn part_one() -> usize{
    let mut product = 1;
    if let Ok(input) = read_file("input"){
        let races = parse_races(input);
        for race in races {
           product *= count_wins(race);
        }
    }
    return product;
}
fn read_file(file_path: &str)-> io::Result<String>{
    let file = File::open(file_path)?;
    let mut bur_reader = BufReader::new(file);
    let mut contents = String::new();
    bur_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
fn parse_races(input: String)-> Vec<Race>{
    let mut races = Vec::new();
    let mut times: Vec<i32> = Vec::new();
    let mut distances: Vec<i32> = Vec::new();
    for (i, str) in input.lines().enumerate() {
        if i ==0  {
            for (i,s) in str.split_whitespace().enumerate(){
                if i != 0 {
                    times.push(s.parse().unwrap());
                }
            }
        }
        if i == 1 {
            for (i, s ) in str.split_whitespace().enumerate(){
                if i!= 0 {
                    distances.push(s.parse().unwrap())
                }

            }
        }
    }
    for (i,v) in times.iter().enumerate(){
        races.push(Race{
            time: *v,
            distance: distances[i],
        })
    }

    return races
}
fn count_wins(race: Race)->usize{
    return 5
}