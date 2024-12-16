use crate::Direction::{East, North, South, West};
use std::fmt::{Debug, Formatter, Pointer};
use std::str::FromStr;
use strum_macros::{AsRefStr, EnumString, IntoStaticStr};

pub mod part_one;
pub mod part_two;

#[derive(Clone, Copy, EnumString, AsRefStr, IntoStaticStr)]
enum Direction {
    #[strum(serialize = "^")]
    North,
    #[strum(serialize = ">")]
    East,
    #[strum(serialize = "v")]
    South,
    #[strum(serialize = "<")]
    West,
}
impl Direction {
    fn next_pos(&self, pos: &(usize, usize)) -> (usize, usize) {
        match *self {
            North => (pos.0 - 1, pos.1),
            East => (pos.0, pos.1 + 1),
            South => (pos.0 + 1, pos.1),
            West => (pos.0, pos.1 - 1),
        }
    }
    fn next_direction(&self) -> Direction {
        match *self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

#[derive(Clone, Copy, EnumString, AsRefStr, IntoStaticStr)]
enum GridElement {
    #[strum(
        serialize = "^",
        serialize = "v",
        serialize = "<",
        serialize = ">",
        serialize = "."
    )]
    Clear,
    #[strum(serialize = "#")]
    Obstacle,
    #[strum(serialize = "E")]
    Edge,
    #[strum(serialize = "X")]
    Visited([bool; 4]),
}

impl GridElement {
    fn from_row(row: &str) -> Vec<GridElement> {
        let mut elems = row
            .chars()
            .flat_map(|c| Self::from_str(&c.to_string()).ok())
            .collect::<Vec<_>>();
        if matches!(elems.first(), Some(GridElement::Clear)) {
            *elems.first_mut().unwrap() = GridElement::Edge;
        }

        if matches!(elems.last(), Some(GridElement::Clear)) {
            *elems.last_mut().unwrap() = GridElement::Edge;
        }

        elems
    }
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<GridElement>>,
    guard_position: (usize, usize),
    guard_direction: Direction,
    loops: bool,
}
impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let grid = &self
            .grid
            .iter()
            .enumerate()
            .map(|(x, v)| {
                v.iter()
                    .enumerate()
                    .map(|(y, g)| {
                        if (x, y) == self.guard_position {
                            self.guard_direction.as_ref()
                        } else {
                            g.as_ref()
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "\nguard pos : {:?}\n{grid}\n", self.guard_position)
    }
}

impl Map {
    fn new(grid: &str) -> Map {
        // Extract the guard and its direction
        let (guard_position, guard_direction) = grid
            .lines()
            .enumerate()
            .flat_map(|(x, row)| {
                row.chars().enumerate().filter_map(move |(y, elem)| {
                    Direction::from_str(&elem.to_string())
                        .ok()
                        .map(move |direction| ((x, y), direction))
                })
            })
            .next()
            .unwrap();

        let mut grid: Vec<Vec<GridElement>> = grid.lines().map(GridElement::from_row).collect();
        grid.first_mut().unwrap().iter_mut().for_each(|g| {
            if matches!(g, GridElement::Clear) {
                *g = GridElement::Edge
            }
        });
        grid.last_mut().unwrap().iter_mut().for_each(|g| {
            if matches!(g, GridElement::Clear) {
                *g = GridElement::Edge
            }
        });

        Self {
            grid,
            guard_position,
            guard_direction,
            loops: false,
        }
    }
    // Returns if we can keep iterating, MUST NEVER ITERATE ONCE DONE
    fn iterate(&mut self) -> bool {
        let (x, y) = self.guard_position;

        let mut visited = [false; 4];
        if let Some(GridElement::Visited(vis)) = self.grid.get(x).and_then(|row| row.get(y)) {
            visited = *vis;
        }

        // If we already have the current visited as flagged in this direction, we're done
        if visited[self.guard_direction as usize] {
            self.loops = true;
            return false;
        }

        visited[self.guard_direction as usize] = true;
        // If we're visiting an edge, flag it and we're done
        if matches!(self.grid[x][y], GridElement::Edge) {
            self.grid[x][y] = GridElement::Visited(visited);
            return false;
        }
        self.grid[x][y] = GridElement::Visited(visited);

        let (mut next_x, mut next_y) = self.guard_direction.next_pos(&self.guard_position);

        // Sanity check, if the next position is outside the grid, we're done
        if next_x >= self.grid.len() || next_y >= self.grid[0].len() {
            return false;
        }

        // If we're about to hit an obstacle, turn instead
        if matches!(self.grid[next_x][next_y], GridElement::Obstacle) {
            self.guard_direction = self.guard_direction.next_direction();
        } else {
            // update guard
            self.guard_position = (next_x, next_y);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_test_part_one() {
        use crate::part_one::solve;
        assert_eq!(solve("input/part_one_sample.txt"), 41);
    }

    #[test]
    fn sanity_test_part_two() {
        use crate::part_two::solve;
        assert_eq!(solve("input/part_two_sample.txt"), 6);
    }
}
