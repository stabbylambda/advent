use std::collections::BTreeSet;

use super::Coord;

#[derive(Debug)]
pub struct Path {
    pub segments: Vec<Coord>,
}

impl Path {
    pub fn new(segments: Vec<Coord>) -> Self {
        Self { segments }
    }

    pub fn from_point_range(
        (point_axis, point_value): (char, u32),
        (range_axis, range_start, range_end): (char, u32, u32),
    ) -> Path {
        let point_value = point_value as usize;
        let range_start = range_start as usize;
        let range_end = range_end as usize;

        Path::new(match (point_axis, range_axis) {
            ('x', 'y') => vec![(point_value, range_start), (point_value, range_end)],
            ('y', 'x') => vec![(range_start, point_value), (range_end, point_value)],
            _ => unreachable!(),
        })
    }

    pub fn all_points(&self) -> BTreeSet<Coord> {
        let mut points = BTreeSet::new();
        for pair in self.segments.windows(2) {
            let &[(x1, y1), (x2, y2)] = pair else {
                panic!()
            };

            let start_x = x1.min(x2);
            let end_x = x1.max(x2);

            let start_y = y1.min(y2);
            let end_y = y1.max(y2);

            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    points.insert((x, y));
                }
            }
        }

        points
    }
}
