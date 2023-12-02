use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::num::ParseIntError;

fn main() {
    println!("Hello, world!");
    let string = read_file("./input.txt").unwrap();
    let strings: Vec<&str> = string.split("\n").collect();
    let mut codes: Vec<i32>= Vec::new();
    for s in strings {
        match extract_code(s) {
            Ok(code) => {codes.push(code)}
            Err(e) => { match e {
                None => {}
                Some(err) => println!("{}",err.to_string())
            }
                }
        };
    }
    let awnser:i32 /* Type */ =codes.iter().sum();
    println!("The awnser is {}",awnser)
}

fn read_file(file_path: &str)-> io::Result<String>{
    let file = File::open(file_path)?;
    let mut bur_reader = BufReader::new(file);
    let mut contents = String::new();
    bur_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
fn extract_code(source: &str)->Result<i32, Option<ParseIntError>>{
    let mut first_digit:Option<char>= None;
    let mut last_digit: Option<char>= None;
    for c in source.chars() {
        if c.is_numeric() {
            if first_digit.is_none() {
               first_digit = Some(c);
            }
            last_digit = Some(c);
        }
    }
   return  match (first_digit, last_digit) {
        (Some(c1), Some(c2)) => {
            let combined_string = c1.to_string() + &c2.to_string();
            match combined_string.parse::<i32>() {
                Ok(num)=> Ok(num),
                Err(e)=> Err(Some(e))
            }
        }
        _ =>{Err(None)}}
}