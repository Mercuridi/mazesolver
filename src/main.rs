use std::{path::Path, fs::read_to_string, collections::{BinaryHeap, HashSet}};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum CellType {
    Entrance,
    Exit,
    Wall,
    Path,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Cell {
    cell_type: CellType,
    coordinate: Coordinate,
    parent_coord: Coordinate,
    manhattan_from_exit: usize,
    cost: usize,
}
impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}
impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost).reverse())
    }
}

impl Cell {
    fn new(coordinate: Coordinate, cell_type: CellType) -> Cell {
        Cell {
            cell_type,
            coordinate,
            parent_coord: Coordinate{x: 0, y: 0},
            manhattan_from_exit: usize::MAX,
            cost: 0,
        }
    }
}

#[derive(Debug)]
struct Grid<Cell> {
    width: usize,
    height: usize,
    entrance_location: Coordinate,
    exit_location: Coordinate,
    cells: Vec<Cell>,
}
impl Grid<Cell> {
    fn new(path_to_maze: &Path) -> Result<Grid<Cell>, std::io::Error> {
        let maze_as_string = read_to_string(path_to_maze)?.replace(' ', "");
        let maze_as_vec: Vec<&str> = maze_as_string.trim().lines().collect();
        let width = maze_as_vec[0].len();
        let height = maze_as_vec.len();
        let mut cells = Vec::with_capacity(width * height);
        let mut exit_coordinates = Vec::new();
        let mut entrance_found = false;
        for (row, chars) in maze_as_vec.iter().enumerate() {
            for (column, char) in chars.chars().enumerate() {
                match char {
                    '-' => {
                        if row == 0 || row == height - 1 || column == 0 || column == width - 1 {
                            if !entrance_found {
                                cells.push(Cell::new(Coordinate{x: column, y: row}, CellType::Entrance));
                                entrance_found = true;
                            }
                            else {
                                cells.push(Cell::new(Coordinate{x: column, y: row}, CellType::Exit));
                            }
                            exit_coordinates.push(Coordinate{x: column, y: row});
                        }
                        else {
                            cells.push(Cell::new(Coordinate{x: column, y: row}, CellType::Path));
                        }
                    },
                    '#' => {
                        cells.push(Cell::new(Coordinate{x: column, y: row}, CellType::Wall));
                    },
                    _ => (),
                }
            }
        };
        exit_coordinates.reverse();
        //print!("exit coordinates: {:?} \n", exit_coordinates);
        let entrance_location = exit_coordinates.pop().unwrap();
        let exit_location = exit_coordinates.pop().unwrap();
        println!("Grid constructed. ");
        Ok(Grid {
            width,
            height,
            entrance_location,
            exit_location,
            cells,
        })
    }
}

fn main() {
    let mut maze = Grid::new(Path::new("mazes/maze-VLarge.txt")).unwrap();
    //println!("maze: {:?} ", maze);
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut current_cell = maze.cells[maze.entrance_location.y * maze.width + maze.entrance_location.x];
    //println!("current_cell: {:?} ", current_cell);
    open_set.push(current_cell);
    while !open_set.is_empty() {
        current_cell = open_set.pop().unwrap();
        if current_cell.coordinate == maze.exit_location {
            break;
        }
        closed_set.insert(current_cell.coordinate);
        let mut neighbours = Vec::new();
        if current_cell.coordinate.x > 0 {
            neighbours.push(Coordinate{x: current_cell.coordinate.x - 1, y: current_cell.coordinate.y});
        }
        if current_cell.coordinate.x < maze.width - 1 {
            neighbours.push(Coordinate{x: current_cell.coordinate.x + 1, y: current_cell.coordinate.y});
        }
        if current_cell.coordinate.y > 0 {
            neighbours.push(Coordinate{x: current_cell.coordinate.x, y: current_cell.coordinate.y - 1});
        }
        if current_cell.coordinate.y < maze.height - 1 {
            neighbours.push(Coordinate{x: current_cell.coordinate.x, y: current_cell.coordinate.y + 1});
        }
        for neighbour in neighbours {
            if closed_set.contains(&neighbour) {
                //print!("skipping neighbour found in closed set \n");
                continue;
            }
            //print!("neighbour.x: {}, neighbour.y: {}, width: {}, height: {} \n", neighbour.x, neighbour.y, maze.width, maze.height);
            let neighbour_cell = &mut maze.cells[neighbour.y * maze.width + neighbour.x];
            if neighbour_cell.cell_type == CellType::Wall || neighbour_cell.cell_type == CellType::Entrance {
                //print!("skipping wall or entrance: {:?} at {:?} \n", neighbour_cell.cell_type, neighbour_cell.coordinate);
                continue;
            }
            let tentative_cost = current_cell.cost + 1;
            if !open_set.iter().any(|heap_item| heap_item == neighbour_cell) || tentative_cost < neighbour_cell.cost {
                neighbour_cell.parent_coord = current_cell.coordinate;
                neighbour_cell.cost = tentative_cost;
                neighbour_cell.manhattan_from_exit = (neighbour_cell.coordinate.x as isize - maze.exit_location.x as isize).unsigned_abs() + (neighbour_cell.coordinate.y as isize - maze.exit_location.y as isize).unsigned_abs();
                if !open_set.iter().any(|heap_item| heap_item == neighbour_cell) {
                    open_set.push(*neighbour_cell);
                }
            }
        }
    }
    println!("Solution found. ");
    let mut path = Vec::new();
    let mut current_cell = maze.cells[maze.exit_location.y * maze.width + maze.exit_location.x];
    while current_cell.coordinate != maze.entrance_location {
        //print!("current_cell in backtrack: {:?} \n", current_cell);
        path.push(current_cell.coordinate);
        current_cell = maze.cells[current_cell.parent_coord.y * maze.width + current_cell.parent_coord.x];
    }
    path.push(maze.entrance_location);
    path.reverse();
    println!("Path length: {} ", path.len());
    //print!("path: {:?} \n", path);
    //print!("maze: {:?} \n", maze);
}