use std::ops::RangeInclusive;

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
