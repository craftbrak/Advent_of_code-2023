use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

fn main() {
    println!("Hello, world! { }, part two : {}", part_one("input"),part_two());

}
#[derive(Debug)]
struct Race{
    time: u64,
    distance: u64
}

impl Display for Race {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}, {}", self.time,self.distance)
    }
}
impl PartialEq for Race {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.distance == other.distance
    }
}
fn part_one(filename :&str) -> u64{
    let mut product = 1;
    if let Ok(input) = read_file(filename){
        let races = parse_races(input);
        for race in races {
           product *= count_wins(race);
        }
    }
    return product;
}
fn part_two() -> u64 {
    return count_wins(Race{ time: 58996469, distance: 478223210191071})
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
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();
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
            time: *v ,
            distance: distances[i],
        })
    }

    return races
}
fn count_wins(race: Race)->u64{
    let mut wins = 0 ;
    for time in 1..race.time - 1 {
        if calculate_distance(race.time - time,time) > race.distance  {
            wins+=1;
        }
    }
    return wins
}
fn calculate_distance(time: u64, speed :u64) -> u64 {
    return time * speed;
}
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_parsing() {
        let input = "Time:        58     99     64     69
                            Distance:   478   2232   1019   1071
                            ";
        let result = parse_races(input.to_string());
        assert_eq!(result.len(),4);
        assert_eq!(result[0], Race {time: 58, distance: 478});
        assert_eq!(result[1], Race {time: 99, distance: 2232});
        assert_eq!(result[2], Race {time: 64, distance: 1019});
        assert_eq!(result[3], Race {time: 69, distance: 1071});

        let input_exemple = "Time:      7  15   30
                                Distance:  9  40  200
                            ";
        let result_exemple = parse_races(input_exemple.to_string());
        assert_eq!(result_exemple.len(),3);
        assert_eq!(result_exemple[0], Race {time: 7, distance: 9});
        assert_eq!(result_exemple[1], Race {time: 15, distance: 40});
        assert_eq!(result_exemple[2], Race {time: 30, distance: 200});
    }
    #[test]
    fn test_part_one_exemple() {
        let result = part_one("exemple");
        assert_eq!(result, 288);
    }
    #[test]
    fn test_part_one_input() {
        let result = part_one("input");
        assert_eq!(result, 128700);
    }
}