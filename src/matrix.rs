use std::ops::Add;

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

impl<T: Add<Output = Matrix<T>>> Add for Matrix<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        
        Self {
            for i in 0..self.shape.0 {
                for j in 0..self.shape.1 {

                }
        }
    }
}
