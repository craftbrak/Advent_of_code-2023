use std::fmt;
use std::fmt::Display;
use std::ops::{Div, Sub};
use common_utils::direction::Direction;
use common_utils::grid::Grid;
use common_utils::point::Point;
use common_utils::{read_file, shoelace_formula};


fn main() {
    println!("Hello, world!");
    part_one();
    part_two();
}
#[derive(Clone,PartialEq)]
enum Pipe {
    Vertical,    // For '|'
    Horizontal,  // For '-'
    BendNE,      // For 'L', a bend connecting north and east
    BendNW,      // For 'J', a bend connecting north and west
    BendSW,      // For '7', a bend connecting south and west
    BendSE,      // For 'F', a bend connecting south and east
    Ground,      // For '.', representing no pipe
    Start,       // For 'S', the starting position
}

impl Pipe {
    fn directions(&self) -> Vec<Direction> {
        match self {
            Pipe::Vertical => { vec![Direction::North, Direction::South] }
            Pipe::Horizontal => { vec![Direction::East, Direction::West] }
            Pipe::BendNE => { vec![Direction::North, Direction::East] }
            Pipe::BendNW => { vec![Direction::North, Direction::West] }
            Pipe::BendSW => { vec![Direction::South, Direction::West] }
            Pipe::BendSE => { vec![Direction::South, Direction::East] }
            Pipe::Ground => { vec![] }
            Pipe::Start => { vec![Direction::South, Direction::East, Direction::West, Direction::North] }
        }
    }
}

impl From<char> for Pipe{
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::BendNE,
            'J' => Self::BendNW,
            '7' => Self::BendSW,
            'F' => Self::BendSE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("Unknown pipe character: {}", value),
        }
    }
}
impl Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char: char = match self {
            Self::Vertical => '|',
            Self::Horizontal => '-',
            Self::BendNE => 'L',
            Self::BendNW => 'J',
            Self::BendSW => '7',
            Self::BendSE => 'F',
            Self::Start => 'S',
            Self::Ground => '.',
        };
        write!(f, "{}", char)
    }
}
fn walk(grid:Grid<Pipe>) -> Vec<Point> {
    let start = grid.get_first_position(&Pipe::Start).expect("No Start");
    let mut visited : Vec<Point> = vec![start];
    loop {
        let current = *visited.last().unwrap();
        let current_pipe = grid.get_for_point(&current).expect("no pipe ??");

        let adjacent: Vec<Point> = match current_pipe {
            Pipe::Start => { current.adjacent().into_iter().filter(|point| {grid.is_in(point)}).filter(|adjacent| {
                let pipe =grid.get_for_point(&adjacent).unwrap();
                adjacent.adjacent_in_directions(pipe.directions()).contains(&current)
            }).collect()
            }
            _ => {current.adjacent_in_directions(current_pipe.directions())}
        };
        let next_moves :Vec<Point> = adjacent.into_iter().filter(|point: &Point| {grid.is_in(point) && !visited.contains(&&point)}).filter(|p| { let pipe = grid.get_for_point(&p).unwrap(); *pipe != Pipe::Ground}).collect();
        if visited.len()> 1 && next_moves.is_empty(){
            break;
        }
        let next_move = next_moves.first().expect("No NextMoves").clone();
        visited.push(next_move)
    }
    visited
}


fn part_one(){
    let grid = Grid::from_custom(read_file("day10/input").unwrap().as_str(), |c| {Pipe::from(c)});
    let path = walk(grid);
    println!("p1 : {}", path.len().div(2).to_string())
}
fn part_two(){
    let grid = Grid::from_custom(read_file("day10/input").unwrap().as_str(), |c| {Pipe::from(c)});
    let path = walk(grid);
    println!("p1 : {}", shoelace_formula(&path)
        .sub(path.len() as isize)
        .to_string())
}