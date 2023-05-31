use std::{path::Path, fs::read_to_string, collections::BinaryHeap};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Cell {
    coordinate: Coordinate,
    parent_index: usize,
    manhattan_from_exit: usize,
    cost: usize,
}
impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}
impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

impl Cell {
    fn new(coordinate: Coordinate) -> Cell {
        Cell {
            coordinate,
            parent_index: 0,
            manhattan_from_exit: 0,
            cost: 0,
        }
    }
    fn get_manhattan(&mut self, exit: &Coordinate) {
        self.manhattan_from_exit = (self.coordinate.x as isize - exit.x as isize).abs() as usize + (self.coordinate.y as isize - exit.y as isize).abs() as usize;
    }
    fn get_cost(mut self, cells: Vec<Cell>) {
        self.cost = self.manhattan_from_exit + cells[self.parent_index].cost + 1;
    }

}

#[derive(Debug)]
struct Grid<Cell> {
    width: usize,
    height: usize,
    cells: BinaryHeap<Cell>,
}

impl Grid<Cell> {
    fn new(path_to_maze: &Path) -> Result<Grid<Cell>, std::io::Error> {
        let maze_as_string = read_to_string(path_to_maze)?.replace(" ", "");
        let maze_as_vec: Vec<&str> = maze_as_string.trim().lines().collect();
        let width = maze_as_vec[0].len();
        let height = maze_as_vec.len();
        let mut cells = Vec::with_capacity(width * height);
        let mut exits = Vec::new();
        let mut index: usize = 0;
        for (row, chars) in maze_as_vec.iter().enumerate() {
            for (column, char) in chars.chars().enumerate() {
                match char {
                    '-' => {
                        cells.push(Cell::new(Coordinate{x: row, y: column}));
                        if row == 0 || row == height - 1 || column == 0 || column == width - 1 {
                            exits.push(index);
                        }
                        index += 1;
                    },
                    _ => (),
                }
            }
        };
        exits.reverse();
        let entrance_index = exits.pop().unwrap();
        let exit_index = exits.pop().unwrap();
        let exit_cell_coordinate = cells[exit_index].coordinate;
        for cell in &mut cells {
            cell.get_manhattan(&exit_cell_coordinate);
        }
        Ok(Grid {
            width,
            height,
            cells: BinaryHeap::from(cells),
        })
    }
}


fn main() {
    let maze_path = Path::new("mazes/maze-Easy.txt");
    let grid = Grid::new(maze_path);
    print!("{:?}", grid);
}
