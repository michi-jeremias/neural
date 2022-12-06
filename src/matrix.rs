#[derive(Debug)]
pub struct Matrix<T: Copy + PartialEq> {
    data: Vec<Vec<T>>,
    shape: (usize, usize),
}

impl<T: Copy + PartialEq> Matrix<T> {
    pub fn new(data: Vec<Vec<T>>) -> Matrix<T> {
        let shape = (data.len(), data[0].len());
        Matrix {
            data,
            shape,
        }
    }
}

