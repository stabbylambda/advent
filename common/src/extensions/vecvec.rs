pub trait VecVec<T: Clone + Copy> {
    fn rotate(&self) -> Vec<Vec<T>>;
    fn transpose(&self) -> Vec<Vec<T>>;
}

impl<T: Clone + Copy> VecVec<T> for Vec<Vec<T>> {
    /** Rotate 90 degrees clockwise */
    fn rotate(&self) -> Vec<Vec<T>> {
        assert!(self.len() == self[0].len());
        let mut v = self.clone();
        v.reverse();
        for i in 0..v.len() {
            for j in i..v.len() {
                // I'm sure we can use mem::swap here, but that probably involves slicing and isn't as clear
                let x = v[i][j];
                let y = v[j][i];
                v[j][i] = x;
                v[i][j] = y;
            }
        }
        v
    }

    fn transpose(&self) -> Vec<Vec<T>> {
        let width = self[0].len();
        // transpose the nested vec so we can examine each char index
        let mut i_t: Vec<Vec<T>> = vec![vec![]; width];
        (0..width).for_each(|x| {
            (0..self.len()).for_each(|y| i_t[x].push(self[y][x]));
        });

        i_t
    }
}
