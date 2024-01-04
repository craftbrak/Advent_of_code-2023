use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::ops::{Add, Div};
use crate::point::Point;

pub mod grid;
pub mod point;
pub mod direction;
pub mod range;

pub fn read_file(file_path: &str)-> io::Result<String>{
    let file = File::open(file_path)?;
    let mut bur_reader = BufReader::new(file);
    let mut contents = String::new();
    bur_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
pub fn shoelace_formula(points: &Vec<Point>) -> isize {
    let len = points.len();

    let (area, perimeter) = points
        .iter()
        .enumerate()
        .fold((0isize, 0isize), |(sum, perimeter), (i, p1)| {
            let l = (i + 1) % len;
            let p2 = points[l];

            let new_perimeter = perimeter + p1.manhattan_distance(&p2) as isize;
            let new_area = sum + (p1.y as isize * p2.x as isize) - (p1.x as isize * p2.y as isize);

            (new_area, new_perimeter)
        });

    area
        .abs()
        .add(perimeter)
        .div(2)
        .add(1)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = read_file("test");
        match result {
            Ok(st) => {
                assert_eq!(st, "test");
            }
            Err(_) => {}
        }

    }
}


#[cfg(test)]
mod tests_shoelace_fromula {
    use crate::point::Point;
    use crate::shoelace_formula;
    #[test]
    fn shoelace_formula_test() {
        assert_eq!(9, shoelace_formula(&square_2()));
        assert_eq!(25, shoelace_formula(&square_4()));
    }

    fn square_2() -> Vec<Point> {
        vec![
            Point::new(0, 0),
            Point::new(0, 2),
            Point::new(2, 2),
            Point::new(2, 0),
        ]
    }

    fn square_4() -> Vec<Point> {
        vec![
            Point::new(0, 0),
            Point::new(0, 4),
            Point::new(4, 4),
            Point::new(4, 0),
        ]
    }
}