use std::collections::HashSet;

pub struct UnionFind(Vec<usize>);

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind((0..size).collect())
    }

    pub fn find(&mut self, x: usize) -> usize {
        let mut y = self.0[x];
        if y != x {
            y = self.find(y);
        }
        y
    }

    pub fn union(&mut self, idx: usize, idy: usize) {
        let x = self.find(idx);
        let y = self.find(idy);
        self.0[y] = x;
    }

    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }

    pub fn len(&mut self) -> usize {
        let mut s: HashSet<usize> = HashSet::new();
        for i in 0..self.0.len() {
            s.insert(self.find(i));
        }
        s.len()
    }
}
