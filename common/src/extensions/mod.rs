use std::ops::RangeInclusive;

pub mod vecvec;

pub trait RangeExt<T>
where
    T: PartialOrd,
{
    fn start(&self) -> &T;
    fn end(&self) -> &T;
    fn partially_contains(&self, other: &dyn RangeExt<T>) -> bool;
    fn fully_contains(&self, other: &dyn RangeExt<T>) -> bool;
}

impl<T: PartialOrd> RangeExt<T> for RangeInclusive<T> {
    fn fully_contains(&self, other: &dyn RangeExt<T>) -> bool {
        self.start() <= other.start() && other.end() <= self.end()
    }

    fn partially_contains(&self, other: &dyn RangeExt<T>) -> bool {
        let other_start_in_range = self.start() <= other.start() && other.start() <= self.end();
        let other_end_in_range = self.start() <= other.end() && other.end() <= self.end();

        other_start_in_range || other_end_in_range
    }

    fn start(&self) -> &T {
        self.start()
    }

    fn end(&self) -> &T {
        self.end()
    }
}

pub trait PointExt<T> {
    fn manhattan(&self, p: &(T, T)) -> T;
}

impl PointExt<i64> for (i64, i64) {
    fn manhattan(&self, (x2, y2): &(i64, i64)) -> i64 {
        let (x1, y1) = self;
        (x1.abs_diff(*x2) + y1.abs_diff(*y2)) as i64
    }
}

impl PointExt<i32> for (i32, i32) {
    fn manhattan(&self, (x2, y2): &(i32, i32)) -> i32 {
        let (x1, y1) = self;
        (x1.abs_diff(*x2) + y1.abs_diff(*y2)) as i32
    }
}

impl PointExt<usize> for (usize, usize) {
    fn manhattan(&self, (x2, y2): &(usize, usize)) -> usize {
        let (x1, y1) = self;
        x1.abs_diff(*x2) + y1.abs_diff(*y2)
    }
}
