use grid2d::{Coordinate, Grid};
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::ops::Add;
use std::path::Path;

fn get_trail_score(grid: &Grid<u32>, start: &Coordinate) -> u32 {
    let mut score = 0;
    if grid.get(start) != Some(&0) {
        return score;
    }
    let mut coordinate_queue = VecDeque::new();
    coordinate_queue.push_back(*start);
    for i in 0..10u32 {
        // Keep popping the queue while it contains the value we search
        while coordinate_queue
            .front()
            .is_some_and(|coord| grid.get(coord).is_some_and(|val| *val == i))
        {
            let coord = coordinate_queue.pop_front().unwrap();
            // If we're processing cells at 9, we just increase the score and continue
            if i == 9 {
                score += 1;
                continue;
            }
            // Find all direct neighbors that are in the grid and have a value of the next digit
            for neighbor in [
                coord.add(&(1, 0)),
                coord.add(&(-1, 0)),
                coord.add(&(0, 1)),
                coord.add(&(0, -1)),
            ]
            .iter()
            .filter_map(|a| *a)
            {
                // If neighbor is in the grind, has the right value, and is not already present in the queue, push it
                if grid.encompasses(neighbor)
                    && grid.get(neighbor).is_some_and(|val| *val == i + 1)
                    && !coordinate_queue.contains(&neighbor)
                {
                    coordinate_queue.push_back(neighbor);
                }
            }
        }
    }

    score
}

pub fn solve<P>(input_file: P) -> u32
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap();
    let width = input.lines().next().unwrap().len();
    let grid = unsafe {
        Grid::new_unchecked(
            width.try_into().unwrap(),
            input
                .lines()
                .flat_map(|line| line.chars().filter_map(|c| c.to_digit(10)))
                .collect::<Vec<u32>>(),
        )
    };

    (0..grid.size().get())
        .map(|linear_idx| Coordinate::from_width_and_index(width.try_into().unwrap(), linear_idx))
        .map(|coord| get_trail_score(&grid, &coord))
        .sum()
}
