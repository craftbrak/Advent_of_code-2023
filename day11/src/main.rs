use std::cmp::{max,min};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use itertools::{Itertools};
use common_utils::grid::Grid;
use common_utils::point::Point;
use common_utils::read_file;


fn main() {
    println!("{},{}",part1("day11/input").0,part2("day11/input",1000000));
}
#[derive(Eq, PartialEq,Clone,Debug, Ord, PartialOrd)]
enum Pixel {
    Galaxy,
    Void,
}
impl From<char> for Pixel {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Galaxy,
            _ => Self::Void
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char: char = match self {
            Self::Galaxy => '#',
            Self::Void => '.',
        };
        write!(f, "{}", char)
    }
}
fn part1(str:&str) -> (i32,Grid<Pixel>){
    let mut grid:Grid<Pixel> = Grid::from_custom(read_file(str).unwrap().as_str(), |c|{ Pixel::from(c) });
    let rows_to_expand = grid.rows_all_filled_with(Pixel::Void);
    rows_to_expand.iter().sorted().rev().for_each(|&row_id| {
        grid.insert_row(row_id, Pixel::Void);
    });
    let cols_to_expand = grid.columns_with_only(Pixel::Void);
    cols_to_expand.iter().sorted().rev().for_each(|&col_id| {
        grid.insert_column(col_id, Pixel::Void);
    });
    let galaxies :HashSet<(Point,Point)>= grid.get_all_positions(&Pixel::Galaxy).iter().combinations(2).map(|x| {
        let mut iter = x.into_iter();
        let a = *iter.next().unwrap();
        let b = *iter.next().unwrap();
        (a,b)
    }).collect();
    println!("{}",grid.rows_range());
    println!("{}",grid.columns_range());
    (galaxies.iter().map(|pair| pair.0.manhattan_distance(&pair.1)).sum::<i32>(),grid)

}
fn part2(str: &str, expention_factor: i32)-> i64{
    let mut grid:Grid<Pixel> = Grid::from_custom(read_file(str).unwrap().as_str(), |c|{ Pixel::from(c) });
    let rows_to_expand = grid.rows_all_filled_with(Pixel::Void);

    let cols_to_expand = grid.columns_with_only(Pixel::Void);
    let galaxies :HashSet<(Point,Point)>= grid.get_all_positions(&Pixel::Galaxy).iter().combinations(2).map(|x| {
        let mut iter = x.into_iter();
        let a = *iter.next().unwrap();
        let b = *iter.next().unwrap();
        (a,b)
    }).collect();

    galaxies.iter().map(|(a,b)| {
        let start_x = min(a.x,b.x);
        let end_x = max(a.x,b.x);

        let n_col_between_x = cols_to_expand.iter().filter(|x| {(start_x..end_x).contains(x)}).collect::<Vec<&i32>>().len() as i32;

        let start_y = min(a.y,b.y);
        let end_y = max(a.y,b.y);
        let n_rows_between_y= rows_to_expand.iter().filter(|y| {(start_y..end_y).contains(y)}).collect::<Vec<&i32>>().len() as i32;

        (a.manhattan_distance(&b)+ n_col_between_x*(expention_factor-1) + n_rows_between_y*(expention_factor-1))as i64
    }).sum::<i64>()
}
#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn exemple(){
        assert_eq!(part1("exemple").0,374);
    }#[test]
    fn exemplep2(){
        assert_eq!(part2("exemple",10),1030);
    }

}