use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::num::ParseIntError;

fn main() {
    let p_1 = part_one("input.txt").unwrap();
    let p_2 = part_two("input.txt").unwrap();
    println!("part 1 {}", p_1);
    println!("part 2 {}",p_2);
}

fn part_one(file_path: &str) -> Option<i32> {
    let mut sum :i32= 0;
    let mut number_with_neighbors:HashMap<i32, HashSet<(usize, usize)>> = HashMap::new();
    let schema :Vec<Vec<char>>=read_file(file_path).unwrap().lines().map(|line| line.chars().collect()).collect();
    for (i, row) in schema.iter().enumerate() {

        for (j, val) in row.iter().enumerate() {
            if val.is_digit(10){
                if let Some((number, coords)) =get_number_neighbors(i,j,&schema){
                    number_with_neighbors.entry(number).or_default().extend(coords);
                }

            }
            // if val.is_digit(10) && !is_adjecent_to_symbol(i, j, &schema).is_empty(){
            //     if let Ok(num) =digit_to_number(i,j,&schema){
            //         if processed_numbers.insert(num){
            //             sum+= num;
            //         }
            //         // if last_valid_n != num {
            //         //     last_valid_n = num;
            //         //     sum = sum.add(num);
            //         // }
            //     }
            // }
        }
    }
    for (number, neighbors) in number_with_neighbors {
        sum+= number * neighbors.len() as i32;
    }
    return Some(sum);
}

fn part_two(file_path: &str) -> Option<i32>{
    let mut sum :i32= 0;
    let schema :Vec<Vec<char>>=read_file(file_path).unwrap().lines().map(|line| line.chars().collect()).collect();
    for (i, row) in schema.iter().enumerate() {
        for (j, _val) in row.iter().enumerate() {
            if let Some(ratio)  =get_gear_ratio(i,j,&schema)  {
                sum+=ratio
            }
        }
    }
    return Some(sum)
}
fn get_gear_ratio(cell_x:usize,cell_y:usize, schema: &Vec<Vec<char>>)->Option<i32>{
    if schema[cell_x][cell_y] !='*' {
        return None
    }
    let n_row = schema.len() as i32 ;
    let n_col = schema[cell_x].len() as i32;
    let mut values:Vec<i32> = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            if i==0 && j==0 {continue}
            let n_x = i + cell_x as i32;
            let n_y = j+ cell_y as i32;
            if (0 <= n_x )&& n_x < n_row && 0 <= n_y && n_y < n_col{
                if schema[n_x as usize][n_y as usize].is_digit(10){
                    if let Ok(num)= digit_to_number(n_x as usize,n_y as usize,&schema) {
                        println!("a gear has neighbor gear : {} {}, neighbor {}",cell_x,cell_y,num);
                        if !values.contains(&num){
                            values.push(num);
                        }

                    }
                }
            }
        }
    }
    if  values.len() == 2 {
        let ratio =values[0] * values[1];
        return Some(ratio);
    }
    None
}

fn read_file(file_path: &str)-> io::Result<String>{
    let file = File::open(file_path)?;
    let mut bur_reader = BufReader::new(file);
    let mut contents = String::new();
    bur_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn is_adjecent_to_symbol(cell_x:usize,cell_y:usize, schema: &Vec<Vec<char>>)->Vec<(usize,usize)>{
    let n_row = schema.len() as i32;
    let n_col = schema[cell_x].len() as i32;
    let mut neighbors_coodrs: Vec<(usize,usize)> = Vec::new();

    for delta_x  in [-1, 0, 1] {
        for delta_y in -1..=1 {
            if delta_x == 0 && delta_y == 0  { continue }
            let neibor_x = cell_x as i32+ delta_x ;
            let neibor_y = cell_y as i32 + delta_y;
            if (0 <= neibor_x )&& neibor_x < n_row && 0 <= neibor_y && neibor_y < n_col { 
                if !&schema[neibor_x as usize][neibor_y as usize].is_digit(10) && schema[neibor_x as usize][neibor_y as usize]!= '.' {
                    neighbors_coodrs.push((neibor_x as usize, neibor_y as usize))
                }
            }
        }
    }
    return neighbors_coodrs
}
fn get_start_index(cell_x : usize, cell_y: usize , schema : &Vec<Vec<char>>)-> usize{
    let i = if cell_y > 0 && schema[cell_x][cell_y - 1].is_numeric() {
        let mut temp_i = cell_y;
        while temp_i > 0 && schema[cell_x][temp_i - 1].is_numeric() {
            temp_i -= 1;
        }
        temp_i
    } else {
        cell_y
    };
    return i
}
fn get_end_index(cell_x : usize, cell_y: usize , schema : &Vec<Vec<char>>)-> usize{
    let n_col = schema[cell_x].len();
    let mut  i = cell_y;
    while i < n_col && schema[cell_x][i].is_numeric() {
        i+=1;
    }
    return i-1
}
fn digit_to_number(cell_x:usize, cell_y:usize, schema: &Vec<Vec<char>>) -> Result<i32, ParseIntError> {
    let n_col = schema[cell_x].len();
    let mut number_str = String::new();
    let mut i = get_start_index(cell_x,cell_y,&schema);
    while i < n_col && schema[cell_x][i].is_numeric() {
        let val = schema[cell_x][i];
        number_str.push(val);
        i+=1;
    }
    return  number_str.parse::<i32>()
}

fn get_number_neighbors(cell_x:usize,cell_y:usize, schema: &Vec<Vec<char>>)-> Option<(i32, HashSet<(usize,usize)>)>{
    let mut coords = HashSet::new();
    let start = get_start_index(cell_x, cell_y, &schema);
    let end = get_end_index(cell_x,cell_y,&schema);
    for _ in start..=end {
        coords.extend(is_adjecent_to_symbol(cell_x,cell_y,&schema))
    }
    if !coords.is_empty(){
        let number = digit_to_number(cell_x,cell_y,&schema);
        if let Ok(n) = number{ 
            return Some((n,coords))
        }
    }
    None
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part_one_complete() {
        let result = part_one("input.txt");
        assert_eq!(result, Some(498559));
    }
    #[test]
    fn test_part_one_exemple() {
        let result = part_one("exemple");
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_one_multi_digit_numbers() {
        let result = part_one("multi-digit");
        assert_eq!(result, Some(3192)); // Update with the correct sum
    }

    #[test]
    fn test_part_one_numbers_start_end_rows() {
        let result = part_one("12*...\n...*34\n56*78*\n");
        assert_eq!(result, Some(180)); // Update with the correct sum
    }

    #[test]
    fn test_part_one_adjacent_symbols() {
        let result = part_one("..*2*..\n..*345*\n..*6*..\n");
        assert_eq!(result, Some(353)); // Update with the correct sum
    }

    #[test]
    fn test_part_one_diagonal_adjacency() {
        let result = part_one("1*...\n.*2..\n..*3.\n...*4\n");
        assert_eq!(result, Some(10)); // Update with the correct sum
    }

    #[test]
    fn test_part_one_no_adjacent_symbols() {
        let result = part_one("123\n456\n789\n");
        assert_eq!(result, Some(0)); // Expected result should be 0
    }

    #[test]
    fn test_part_one_symbols_without_numbers() {
        let result = part_one("..*..\n.*.*.\n..*..\n");
        assert_eq!(result, Some(0)); // Expected result should be 0
    }

    #[test]
    fn test_part_one_complex_scenario() {
        let result = part_one("12*3.4$5\n67#89*0.\n.*@*..%.\n$123#456\n");
        assert_eq!(result, Some(1234)); // Update with the correct sum
    }


}