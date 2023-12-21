use std::num::ParseIntError;
use common_utils::read_file;

fn main() {
    println!("Hello, world!");
    let string = read_file("day1/input.txt").unwrap();
    let strings: Vec<&str> = string.lines().collect();
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
fn extract_code(source: &str)->Result<i32, Option<ParseIntError>>{
    let mut first_digit:Option<char>= None;
    let mut last_digit: Option<char>= None;

    for (i, c) in source.char_indices() {

        let digit=is_digit_or_digit_word(&source,&c,0,i+1);
        match digit {
            None => {}
            Some(d) => {first_digit= Some(d);break}
        }
    }
    for (i, c) in source.char_indices().rev() {

        let digit=is_digit_or_digit_word(&source,&c,i,source.len());
        match digit {
            None => {}
            Some(d) => {last_digit= Some(d);break}
        }
    }
    if first_digit.is_some() {
        println!("digit founds for this line {}: {}{} ",source,first_digit.unwrap(),last_digit.unwrap());
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
fn is_digit_or_digit_word(s:&str, c:&char, i_start:usize, i_end:usize)->Option<char>{
    let digit_pairs = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];
    if c.is_digit(10){
        return Some(*c);
    }
    else {
        for &(name, d) in &digit_pairs {
            if s[i_start..i_end].contains(name) {
                return Some(d);
            }
        }
    }
    None
}