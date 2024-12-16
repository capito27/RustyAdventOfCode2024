use grid2d::{Coordinate, Grid};
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::ops::Add;
use std::path::Path;

fn orthogonal_neighbors(start: &Coordinate) -> Vec<Option<Coordinate>> {
    [
        start.add(&(1, 0)),
        start.add(&(-1, 0)),
        start.add(&(0, 1)),
        start.add(&(0, -1)),
    ]
    .to_vec()
}

pub fn get_coords_in_region(grid: &Grid<char>, start: &Coordinate) -> Vec<Coordinate> {
    let mut res = vec![];
    let val = *grid.get(start).unwrap();

    let mut coordinate_queue = VecDeque::new();
    coordinate_queue.push_back(*start);
    while let Some(coordinate) = coordinate_queue.pop_front() {
        res.push(coordinate);

        // Find all direct neighbors that are in the grid and have a value of the next digit
        orthogonal_neighbors(&coordinate)
            .iter()
            .filter_map(|o| o.as_ref())
            .for_each(|neighbor| {
                if grid.encompasses(neighbor)
                    && grid.get(neighbor).is_some_and(|other| val == *other)
                    && !coordinate_queue.contains(&neighbor)
                    && !res.contains(&neighbor)
                {
                    coordinate_queue.push_back(neighbor.clone());
                }
            });
    }

    res
}

fn get_region_perimeter(coordinates: &[Coordinate]) -> u64 {
    coordinates
        .iter()
        .flat_map(|coordinate| {
            orthogonal_neighbors(coordinate)
                .into_iter()
                .filter(|neighbor| neighbor.is_none_or(|neighbor| !coordinates.contains(&neighbor)))
        })
        .count() as u64
}

pub fn solve<P>(input_file: P) -> u64
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
                .flat_map(|line| line.chars())
                .collect::<Vec<char>>(),
        )
    };

    (0..grid.size().get())
        .map(|linear_idx| Coordinate::from_width_and_index(width.try_into().unwrap(), linear_idx))
        .fold(
            (0u64, Vec::<Coordinate>::new()),
            |(acc, mut visted_coords), coord| {
                if !visted_coords.contains(&coord) {
                    let region = get_coords_in_region(&grid, &coord);
                    println!(
                        "region of {} plants with price {} * {} = {}",
                        grid.get(&coord).unwrap(),
                        region.len(),
                        get_region_perimeter(&region),
                        get_region_perimeter(&region) * region.len() as u64
                    );
                    visted_coords.extend(&region);
                    (
                        acc + get_region_perimeter(&region) * region.len() as u64,
                        visted_coords,
                    )
                } else {
                    (acc, visted_coords)
                }
            },
        )
        .0
}
