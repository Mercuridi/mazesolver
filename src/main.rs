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
// define ordering so that we can use Cells in a BinaryHeap
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
    // define a constructor for a Cell
    fn new(coordinate: Coordinate, cell_type: CellType) -> Cell {
        Cell {
            cell_type,
            coordinate,
            parent_coord: Coordinate{x: 0, y: 0}, // set a default coordinate; (0, 0) is nearly always a wall, so we know if something goes wrong
            manhattan_from_exit: 0,    
            cost: 0,    // we leave cost at 0 so that if something goes wrong, the cost is still an underestimate and therefore
                        // an admissible heuristic for A*
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
    // Grid constructor
    fn new(path_to_maze: &Path) -> Result<Grid<Cell>, std::io::Error> {
        // Remove spaces from the maze we just read in
        let maze_as_string = read_to_string(path_to_maze)?.replace(' ', "");
        // Convert the maze to a vector of strings, one string per row
        let maze_as_vec: Vec<&str> = maze_as_string.trim().lines().collect();
        // Get the width and height of the maze
        let width = maze_as_vec[0].len();
        let height = maze_as_vec.len();
        // Declare vectors to hold all the cells and a vector of coordinates to hold where the exits are
        let mut cells = Vec::with_capacity(width * height);
        let mut exit_coordinates = Vec::new();
        // Boolean to check if we've found the entrance yet
        let mut entrance_found = false;
        for (row, chars) in maze_as_vec.iter().enumerate() {
            for (column, char) in chars.chars().enumerate() {
                match char {
                    '-' => {
                        if row == 0 || row == height - 1 || column == 0 || column == width - 1 {
                            // Only the first '-' we find is the entrance, the rest are exits
                            if !entrance_found {
                                cells.push(Cell::new(Coordinate{x: column, y: row}, CellType::Entrance));
                                entrance_found = true;
                            }
                            else {
                                cells.push(Cell::new(Coordinate{x: column, y: row}, CellType::Exit));
                            }
                            exit_coordinates.push(Coordinate{x: column, y: row});
                        }
                        // If it's a '-' that's not on the edge, it's a path
                        else {
                            cells.push(Cell::new(Coordinate{x: column, y: row}, CellType::Path));
                        }
                    },
                    // Any '#' is a wall
                    '#' => {
                        cells.push(Cell::new(Coordinate{x: column, y: row}, CellType::Wall));
                    },
                    _ => (),
                }
            }
        };
        exit_coordinates.reverse();
        //print!("exit coordinates: {:?} \n", exit_coordinates);
        // Get the entrance and exit coordinates
        // We only pop the first two to simulate a "perfect maze"; a further expansion would be to allow for imperfect mazes
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

    // Declare all our collections to store our working data
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut current_cell = maze.cells[maze.entrance_location.y * maze.width + maze.entrance_location.x];

    //println!("current_cell: {:?} ", current_cell);
    open_set.push(current_cell);
    while !open_set.is_empty() {
        // Get the lowest cost item from the open set
        // The open set will always pop the lowest cost item due to our custom definition of Ord on the Cells in open_set
        current_cell = open_set.pop().unwrap();
        if current_cell.coordinate == maze.exit_location {
            // If the popped cell is the exit, we're done, so break the loop
            break;
        }
        // If the popped cell is not the exit, add it to the closed set and get its neighbours
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

        // Loop across the neighbours we just found
        for neighbour in neighbours {
            // If a neighbour is in the closed set, skip it
            if closed_set.contains(&neighbour) {
                //print!("skipping neighbour found in closed set \n");
                continue;
            }
            //print!("neighbour.x: {}, neighbour.y: {}, width: {}, height: {} \n", neighbour.x, neighbour.y, maze.width, maze.height);

            // Get the neighbour cell itself from the maze using its coordinates
            let neighbour_cell = &mut maze.cells[neighbour.y * maze.width + neighbour.x];
            if neighbour_cell.cell_type == CellType::Wall || neighbour_cell.cell_type == CellType::Entrance {
                // If the neighbour is a wall or an entrance, we can safely skip it
                // (walls are irrelevant, the entrance is already in the closed set even on the first iteration)
                //print!("skipping wall or entrance: {:?} at {:?} \n", neighbour_cell.cell_type, neighbour_cell.coordinate);
                continue;
            }

            // A neighbour cell's cost is the cost of the current cell plus 1
            let tentative_cost = current_cell.cost + 1;
            // If the neighbour cell is not in the open set, or if the tentative cost is less than the neighbour cell's cost, update the neighbour cell
            // We update on the basis of the tentative cost being less than the neighbour cell's cost because we want to find the shortest path, 
            // and a neighbour may have already been found in another exploration of the maze, but with a higher cost
            // We only ever care about the lower cost; if we found a path to a cell with a lower cost, great!
            if !open_set.iter().any(|heap_item| heap_item == neighbour_cell) || tentative_cost < neighbour_cell.cost {
                neighbour_cell.parent_coord = current_cell.coordinate;
                neighbour_cell.cost = tentative_cost;
                neighbour_cell.manhattan_from_exit = (neighbour_cell.coordinate.x as isize - maze.exit_location.x as isize).unsigned_abs() + (neighbour_cell.coordinate.y as isize - maze.exit_location.y as isize).unsigned_abs();
                // Now that we've updated the neighbour, if it's not in the open set, add it
                // On top of that, if it's in the closed set, remove it from the closed set so we don't skip over it later when we shouldn't
                if !open_set.iter().any(|heap_item| heap_item == neighbour_cell) {
                    open_set.push(*neighbour_cell);
                    closed_set.remove(&neighbour_cell.coordinate);
                }
            }
        }
    }
    println!("Solution found. ");
    let mut path = Vec::new();
    let mut current_cell = maze.cells[maze.exit_location.y * maze.width + maze.exit_location.x];
    // Loop to backtrack through the complete path and reconstruct it.
    while current_cell.coordinate != maze.entrance_location {
        path.push(current_cell.coordinate);
        current_cell = maze.cells[current_cell.parent_coord.y * maze.width + current_cell.parent_coord.x];
    }
    path.push(maze.entrance_location);
    path.reverse();
    println!("Path length: {} ", path.len());
    //print!("path: {:?} \n", path);
    //print!("maze: {:?} \n", maze);
}