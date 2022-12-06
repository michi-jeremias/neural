pub struct Matrix<T: Copy + PartialEq> {
    data: Vec<T>,
    shape: (usize, usize),
}

impl<T: Copy + PartialEq> Matrix<T> {
    pub fn new(mut v: Vec<Vec<T>>) -> Matrix<T> {
        let shape = (v.len(), v[0].len());
        Matrix {
            data,
            shape,
        }
    }
}

