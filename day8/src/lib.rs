pub mod part_one;
pub mod part_two;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point2D {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point2D {
    fn from(value: (i32, i32)) -> Self {
        Point2D {
            x: value.0,
            y: value.1,
        }
    }
}
impl Point2D {
    fn extrapolate(&self, other: &Point2D) -> Point2D {
        Point2D {
            x: self.x + 2 * (other.x - self.x),
            y: self.y + 2 * (other.y - self.y),
        }
    }
    fn extrapolate_n(
        &self,
        other: &Point2D,
        min_x: i32,
        max_x: i32,
        min_y: i32,
        max_y: i32,
    ) -> Vec<Point2D> {
        let mut res = Vec::<Point2D>::new();

        let offset_x = other.x - self.x;
        let offset_y = other.y - self.y;
        let mut i = 1;
        loop {
            let point = Point2D {
                x: self.x + i * offset_x,
                y: self.y + i * offset_y,
            };
            if point.x < max_x && point.x >= min_x && point.y < max_y && point.y >= min_y {
                res.push(point);
            } else {
                break;
            }
            i += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_test_part_one() {
        use crate::part_one::solve;
        assert_eq!(solve("input/part_one_sample.txt"), 14);
    }

    #[test]
    fn sanity_test_part_two() {
        use crate::part_two::solve;
        assert_eq!(solve("input/part_two_sample.txt"), 34);
    }
}
