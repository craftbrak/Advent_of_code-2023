
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::fmt::Display;
use crate::direction::Direction;
use crate::point::Point;
use crate::range::Range;
use itertools::Itertools;
#[derive(Debug, Clone)]
pub struct Grid<T> {
    cells: HashMap<Point, T>,
    columns_range: Range,
    rows_range: Range,
}

impl<T> Grid<T>
    where T: PartialEq
{
    pub fn new(cells: HashMap<Point, T>) -> Self {
        let columns_range = Self::calculate_columns_range(&cells);
        let rows_range = Self::calculate_rows_range(&cells);

        Self {
            cells,
            columns_range,
            rows_range,
        }
    }

    pub fn from_custom(input: &str, func: fn(char) -> T) -> Self {
        let cells: HashMap<Point, T> = input
            .lines()
            .enumerate()
            .map(|(y, line)| -> Vec<(Point, T)> {
                line
                    .chars()
                    .enumerate()
                    .map(|(x, c)| (Point::new(x as i32, y as i32), func(c)))
                    .collect()
            })
            .flatten()
            .collect();

        Self::new(cells)
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        self.cells.get(&Point::new(x, y))
    }

    pub fn get_for_point(&self, point: &Point) -> Option<&T> {
        self.cells.get(&point)
    }

    pub fn get_first_position(&self, element: &T) -> Option<Point> {
        self.cells
            .iter()
            .find_map(|(p, e)| {
                if element == e {
                    return Some(p.clone());
                }

                return None;
            })
    }

    pub fn get_all_positions(&self, element: &T) -> Vec<Point> {
        self.cells
            .iter()
            .filter(|(_, e)| element == *e)
            .map(|(p, _)| p.clone())
            .collect()
    }

    pub fn is_in(&self, point: &Point) -> bool {
        self.columns_range.is_in_range(point.x as i64)
            && self.rows_range.is_in_range(point.y as i64)
    }

    pub fn rows(&self) -> BTreeMap<i32, BTreeMap<&Point, &T>> {
        self.rows_range
            .iter()
            .map(|y| {
                let cells_in_row = self.cells
                    .iter()
                    .filter(|(&point, _)| point.y == y as i32)
                    .collect();

                (y as i32, cells_in_row)
            })
            .collect()
    }

    pub fn columns(&self) -> BTreeMap<i32, BTreeMap<&Point, &T>> {
        self.columns_range
            .iter()
            .map(|x| {
                let cells_in_column = self.cells
                    .iter()
                    .filter(|(&point, _)| point.x == x as i32)
                    .collect();

                (x as i32, cells_in_column)
            })
            .collect()
    }

    pub fn insert_rows(&mut self, rows: Vec<i32>, element: T)
        where T: Clone
    {
        for row in rows.iter().sorted().rev() {
            self.insert_row(row.clone(), element.clone());
        }
    }

    pub fn insert_row(&mut self, row: i32, element: T)
        where T: Clone
    {
        self.move_rows_to_south_from(row);
        self.add_row(row, element);
        self.recalculate_ranges();
    }

    pub fn insert_columns(&mut self, columns: Vec<i32>, element: T)
        where T: Clone
    {
        for column in columns.iter().sorted().rev() {
            self.insert_column(column.clone(), element.clone());
        }
    }

    pub fn insert_column(&mut self, column: i32, element: T)
        where T: Clone
    {
        self.move_columns_to_east_from(column);
        self.add_column(column, element);
        self.recalculate_ranges();
    }

    pub fn rows_range(&self) -> Range {
        self.rows_range
    }

    pub fn columns_range(&self) -> Range {
        self.columns_range
    }

    pub fn modify(&mut self, point: Point, new_value: T) {
        *self.cells.get_mut(&point).unwrap() = new_value;
    }
    pub fn columns_with_only(&self, value: T) -> Vec<i32> {
        let columns = self.columns();

        columns
            .iter()
            // Filter columns that are entirely Space::Void
            .filter_map(|(&column_id, column_data)| {
                if column_data.values().all(|&cell| *cell == value) {
                    Some(column_id) // Include this column ID
                } else {
                    None // Exclude this column
                }
            })
            // Collect the filtered column IDs into a vector
            .collect()
    }
    pub fn rows_all_filled_with(&self, value_to_check: T) -> Vec<i32> {
        let rows = self.rows();

        rows.iter()
            .filter_map(|(&row_id, row_data)| {
                if row_data.values().all(|&cell| *cell == value_to_check) {
                    Some(row_id) // Include this row ID
                } else {
                    None // Exclude this row
                }
            })
            .collect()
    }

    #[cfg(test)]
    pub fn modify_many(&mut self, points: Vec<Point>, new_value: T)
        where T: Clone
    {
        for point in points {
            self.modify(point, new_value.clone())
        }
    }

    fn move_rows_to_south_from(&mut self, from: i32) {
        for y in self.rows_range
            .iter()
            .skip(from as usize)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            for x in self.columns_range.iter() {
                let old = Point::new(x as i32, y as i32);
                let new = old.move_in(Direction::South);

                self.replace_position(&old, new);
            }
        }
    }

    fn replace_position(&mut self, old: &Point, new: Point) {
        if let Some(v) = self.cells.remove(&old) {
            self.cells.insert(new, v);
        }
    }

    fn move_columns_to_east_from(&mut self, from: i32) {
        for x in self.columns_range
            .iter()
            .skip(from as usize)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            for y in self.rows_range.iter() {
                let old = Point::new(x as i32, y as i32);
                let new = old.move_in(Direction::East);

                self.replace_position(&old, new);
            }
        }
    }

    fn add_row(&mut self, row: i32, element: T)
        where T: Clone
    {
        for x in self.columns_range.iter() {
            let new = Point::new(x as i32, row);

            self.cells.insert(new, element.clone());
        }
    }

    fn add_column(&mut self, column: i32, element: T)
        where T: Clone
    {
        for y in self.rows_range.iter() {
            let new = Point::new(column, y as i32);

            self.cells.insert(new, element.clone());
        }
    }

    fn calculate_rows_range(cells: &HashMap<Point, T>) -> Range {
        let y: Vec<i32> = cells
            .keys()
            .map(|k| k.y)
            .collect();

        Range::new(*y.iter().min().unwrap() as i64, *y.iter().max().unwrap() as i64).unwrap()
    }

    fn calculate_columns_range(cells: &HashMap<Point, T>) -> Range {
        let x: Vec<i32> = cells
            .keys()
            .map(|k| k.x)
            .collect();

        Range::new(*x.iter().min().unwrap() as i64, *x.iter().max().unwrap() as i64).unwrap()
    }

    fn recalculate_ranges(&mut self) {
        self.columns_range = Self::calculate_columns_range(&self.cells);
        self.rows_range = Self::calculate_rows_range(&self.cells)
    }

}

impl<T> Display for Grid<T>
    where T: Display + Ord
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printed_grid = String::new();
        for y in self.rows_range.iter() {
            for x in self.columns_range.iter() {
                let el = self.get(x as i32, y as i32).unwrap();

                printed_grid += el.to_string().as_str()
            }

            printed_grid += "\n";
        }

        write!(f, "{}", printed_grid)
    }
}
#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};
    use crate::grid::Grid;
    use crate::point::Point;

    #[test]
    fn get() {
        let grid: Grid<char> = grid();

        assert_eq!(Some(&'A'), grid.get(0, 0));
        assert_eq!(Some(&'B'), grid.get(1, 0));
        assert_eq!(Some(&'C'), grid.get(0, 1));
        assert_eq!(Some(&'D'), grid.get(1, 1));

        assert!(grid.get(1, 2).is_none())
    }

    #[test]
    fn get_first_position() {
        let mut hash_map: HashMap<Point, char> = HashMap::new();
        hash_map.insert(Point::new(2, 3), 'X');

        let grid = Grid::new(hash_map);

        assert_eq!(Point::new(2, 3), grid.get_first_position(&'X').unwrap());

        assert!(grid.get_first_position(&'A').is_none())
    }

    #[test]
    fn display() {
        let grid: Grid<char> = grid();

        assert_eq!("AB\nCD\n", grid.to_string());
    }

    #[test]
    fn rows() {
        let grid: Grid<char> = grid();
        let rows = grid.rows();

        let row_1: Vec<char> = get_chars(&rows, 0);
        assert_eq!(2, row_1.len());
        assert!(row_1.contains(&'A'));
        assert!(row_1.contains(&'B'));

        let row_2: Vec<char> = get_chars(&rows, 1);
        assert_eq!(2, row_2.len());
        assert!(row_2.contains(&'C'));
        assert!(row_2.contains(&'D'));
    }

    #[test]
    fn columns() {
        let grid: Grid<char> = grid();
        let columns = grid.columns();

        let column_1: Vec<char> = get_chars(&columns, 0);
        assert_eq!(2, column_1.len());
        assert!(column_1.contains(&'A'));
        assert!(column_1.contains(&'C'));

        let column_2: Vec<char> = get_chars(&columns, 1);
        assert_eq!(2, column_2.len());
        assert!(column_2.contains(&'B'));
        assert!(column_2.contains(&'D'));
    }

    #[test]
    fn insert_row() {
        let mut grid: Grid<char> = grid();

        grid.insert_row(1, '.');
        assert_eq!("AB\n..\nCD\n", grid.to_string());

        grid.insert_row(3, '.');
        assert_eq!("AB\n..\nCD\n..\n", grid.to_string());

        grid.insert_row(0, '.');
        assert_eq!("..\nAB\n..\nCD\n..\n", grid.to_string());
    }

    #[test]
    fn insert_column() {
        let mut grid: Grid<char> = grid();

        grid.insert_column(1, '.');
        assert_eq!("A.B\nC.D\n", grid.to_string());

        grid.insert_column(3, '.');
        assert_eq!("A.B.\nC.D.\n", grid.to_string());

        grid.insert_column(0, '.');
        assert_eq!(".A.B.\n.C.D.\n", grid.to_string());
    }

    #[test]
    fn insert_columns() {
        let mut grid: Grid<char> = grid();

        grid.insert_columns(vec!(1, 2, 0), '.');

        assert_eq!(".A.B.\n.C.D.\n", grid.to_string());
    }

    #[test]
    fn insert_rows() {
        let mut grid: Grid<char> = grid();

        grid.insert_rows(vec!(1, 2, 0), '.');

        assert_eq!("..\nAB\n..\nCD\n..\n", grid.to_string());
    }
    #[test]
    fn columns_all_filled_with_test() {
        // Create a grid with specific values
        let mut hash_map: HashMap<Point, char> = HashMap::new();
        hash_map.insert(Point::new(0, 0), 'X');
        hash_map.insert(Point::new(0, 1), 'X');
        hash_map.insert(Point::new(1, 0), 'O');
        hash_map.insert(Point::new(1, 1), 'X');
        hash_map.insert(Point::new(2, 0), 'X');
        hash_map.insert(Point::new(2, 1), 'X');
        let grid = Grid::new(hash_map);

        // Call the function with the value 'X'
        let columns_filled_with_x = grid.columns_with_only('X');

        // Assert that the function returns the correct column IDs
        // Here we expect columns 0 and 2 to be entirely filled with 'X'
        assert_eq!(columns_filled_with_x, vec![0, 2]);
    }
    #[test]
    fn rows_all_filled_with_test() {
        // Create a grid with specific values
        let mut hash_map: HashMap<Point, char> = HashMap::new();
        hash_map.insert(Point::new(0, 0), 'Y');
        hash_map.insert(Point::new(0, 1), 'O');
        hash_map.insert(Point::new(1, 0), 'Y');
        hash_map.insert(Point::new(1, 1), 'Y');
        hash_map.insert(Point::new(2, 0), 'Y');
        hash_map.insert(Point::new(2, 1), 'O');
        let grid = Grid::new(hash_map);

        // Call the function with the value 'Y'
        let rows_filled_with_y = grid.rows_all_filled_with('Y');

        // Assert that the function returns the correct row IDs
        // Here we expect row 0 to be entirely filled with 'Y'
        assert_eq!(rows_filled_with_y, vec![0]);
    }
    fn grid() -> Grid<char> {
        let mut hash_map: HashMap<Point, char> = HashMap::new();
        hash_map.insert(Point::new(0, 0), 'A');
        hash_map.insert(Point::new(0, 1), 'C');
        hash_map.insert(Point::new(1, 1), 'D');
        hash_map.insert(Point::new(1, 0), 'B');

        Grid::new(hash_map)
    }

    fn get_chars(data: &BTreeMap<i32, BTreeMap<&Point, &char>>, row_or_column: i32) -> Vec<char> {
        data
            .get(&row_or_column)
            .unwrap()
            .iter()
            .map(|(_, &&c)| c)
            .collect()
    }
}