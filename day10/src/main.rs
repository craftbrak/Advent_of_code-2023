use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}
struct Coordinate {
    x: usize,
    y: usize
}
struct Node {
    prev: Coordinate,
    next: Coordinate,
    visited:bool
}
struct Graph {
    nodes: HashMap<Coordinate,Node>,
    distance: i64
}
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

fn get_pipe(c: char) -> Pipe {
    match c {
        '|' => Pipe::Vertical,
        '-' => Pipe::Horizontal,
        'L' => Pipe::BendNE,
        'J' => Pipe::BendNW,
        '7' => Pipe::BendSW,
        'F' => Pipe::BendSE,
        '.' => Pipe::Ground,
        'S' => Pipe::Start,
        _ => panic!("Unknown pipe character: {}", c), // Or handle this case differently if needed
    }
}
fn can_connect(current: Pipe, neighbor: Pipe) -> bool {
    match current {
        Pipe::Vertical => matches!(neighbor, Pipe::Vertical | Pipe::BendNE | Pipe::BendNW | Pipe::Start),
        Pipe::Horizontal => matches!(neighbor, Pipe::Horizontal | Pipe::BendSE | Pipe::BendSW | Pipe::Start),
        Pipe::BendNE => matches!(neighbor, Pipe::Vertical | Pipe::Horizontal | Pipe::Start),
        Pipe::BendNW => matches!(neighbor, Pipe::Vertical | Pipe::Horizontal | Pipe::Start),
        Pipe::BendSW => matches!(neighbor, Pipe::Vertical | Pipe::Horizontal | Pipe::Start),
        Pipe::BendSE => matches!(neighbor, Pipe::Vertical | Pipe::Horizontal | Pipe::Start),
        Pipe::Start => true, // Start can connect to any pipe, but this is further determined by the neighbor's type
        Pipe::Ground => false, // Ground doesn't connect to anything
    }
}
fn get_neighbours(matrix :&Vec<Vec<char>>, coordinate: Coordinate)-> (Option<Coordinate>,Option<Coordinate>){
    let pipe = get_pipe(matrix.get(coordinate.x).unwrap().get(coordinate.y).unwrap().clone());
    match pipe {
        Pipe::Vertical => {
            let left = if coordinate.y > 0 { Some(Coordinate { x: coordinate.x, y: coordinate.y - 1 }) } else { None };
            let right = if coordinate.y < matrix[coordinate.x].len() - 1 { Some(Coordinate { x: coordinate.x, y: coordinate.y + 1 }) } else { None };
            (left, right)
        },
        Pipe::Horizontal => {
            let up = if coordinate.x > 0 { Some(Coordinate { x: coordinate.x - 1, y: coordinate.y }) } else { None };
            let down = if coordinate.x < matrix.len() - 1 { Some(Coordinate { x: coordinate.x + 1, y: coordinate.y }) } else { None };
            (up, down)
        },
        Pipe::BendNE => {
            let up = if coordinate.x > 0 { Some(Coordinate { x: coordinate.x - 1, y: coordinate.y }) } else { None };
            let right = if coordinate.y < matrix[coordinate.x].len() - 1 { Some(Coordinate { x: coordinate.x, y: coordinate.y + 1 }) } else { None };
            (up, right)
        },
        Pipe::BendNW => {
            let up = if coordinate.x > 0 { Some(Coordinate { x: coordinate.x - 1, y: coordinate.y }) } else { None };
            let left = if coordinate.y > 0 { Some(Coordinate { x: coordinate.x, y: coordinate.y - 1 }) } else { None };
            (up, left)
        },
        Pipe::BendSW => {
            let down = if coordinate.x < matrix.len() - 1 { Some(Coordinate { x: coordinate.x + 1, y: coordinate.y }) } else { None };
            let left = if coordinate.y > 0 { Some(Coordinate { x: coordinate.x, y: coordinate.y - 1 }) } else { None };
            (down, left)
        },
        Pipe::BendSE => {
            let down = if coordinate.x < matrix.len() - 1 { Some(Coordinate { x: coordinate.x + 1, y: coordinate.y }) } else { None };
            let right = if coordinate.y < matrix[coordinate.x].len() - 1 { Some(Coordinate { x: coordinate.x, y: coordinate.y + 1 }) } else { None };
            (down, right)
        },

        Pipe::Start => {
            let mut connections: Vec<Option<Coordinate>> = Vec::new();

            // Check Up
            if coordinate.x > 0 && can_connect(Pipe::Start, get_pipe(matrix[coordinate.x - 1][coordinate.y])) {
                connections.push(Some(Coordinate { x: coordinate.x - 1, y: coordinate.y }));
            }

            // Check Down
            if coordinate.x < matrix.len() - 1 && can_connect(Pipe::Start, get_pipe(matrix[coordinate.x + 1][coordinate.y])) {
                connections.push(Some(Coordinate { x: coordinate.x + 1, y: coordinate.y }));
            }

            // Check Left
            if coordinate.y > 0 && can_connect(Pipe::Start, get_pipe(matrix[coordinate.x][coordinate.y - 1])) {
                connections.push(Some(Coordinate { x: coordinate.x, y: coordinate.y - 1 }));
            }

            // Check Right
            if coordinate.y < matrix[coordinate.x].len() - 1 && can_connect(Pipe::Start, get_pipe(matrix[coordinate.x][coordinate.y + 1])) {
                connections.push(Some(Coordinate { x: coordinate.x, y: coordinate.y + 1 }));
            }

            // Return the first two connections found (or None if fewer)
            (connections.get(0).cloned().flatten(), connections.get(1).cloned().flatten())
        }
        Pipe::Ground => (None, None),
    }
}
fn deph_fisrt_search(){
    //Todo: implmement DFS
}
fn parse_matrix(string: String)-> Vec<Vec<char>>{
    string.lines().map(|x |{ x.chars().collect::<Vec<char>>()}).collect::<Vec<Vec<char>>>()
}
fn find_start(matrix :&Vec<Vec<char>>)-> Option<Coordinate>{
    for (i,row) in matrix.iter().enumerate() {
        for (j, c)  in row.iter().enumerate() {
            if c == 'S' { return Some(Coordinate{ x: i , y: j  }) }
        }
    }
    return None
}