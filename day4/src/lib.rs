pub mod part_one;
pub mod part_two;

struct _2DMask {
    relative_pos: Vec<isize>,
    left_offset: isize,
    right_offset: isize,
}

impl _2DMask {
    pub const fn new(relative_pos: Vec<isize>, left_offset: isize, right_offset: isize) -> _2DMask {
        _2DMask {
            relative_pos,
            left_offset,
            right_offset,
        }
    }

    pub fn apply<'a, T>(&self, pos: isize, width: isize, vals: &'a [T]) -> Option<Vec<&'a T>> {
        if pos % width < self.left_offset || (pos % width) + self.right_offset >= width {
            return None;
        }

        self.relative_pos
            .iter()
            .map(|&i| vals.get((i + pos) as usize))
            .collect::<Option<Vec<_>>>()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_test_part_one() {
        use crate::part_one::solve;
        assert_eq!(solve("input/part_one_sample.txt"), 18);
    }

    #[test]
    fn sanity_test_part_two() {
        use crate::part_two::solve;
        assert_eq!(solve("input/part_two_sample.txt"), 9);
    }
}
