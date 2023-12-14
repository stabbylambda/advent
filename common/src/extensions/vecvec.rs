pub trait VecVec<T: Clone + Copy> {
    fn rotate(&self) -> Vec<Vec<T>>;
}

impl<T: Clone + Copy> VecVec<T> for Vec<Vec<T>> {
    /** Rotate 90 degrees clockwise */
    fn rotate(&self) -> Vec<Vec<T>> {
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
}
