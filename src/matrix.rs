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

    pub fn is_compatible(self, other: Self) -> bool {
        self.shape == other.shape
    }
}

impl<T: Copy + PartialEq> Add for Matrix<T> where T: Add<Output = Matrix<T>> {
    type Output = Self;

    fn add(self, other: Self) -> Matrix<T> {
        // let data = vec![vec![0; self.shape.1]; self.shape.0];
        //
        // for i in 0..self.shape.0 {
        //     for j in 0..self.shape.1 {
        //         data 
        //     }
        // }
        // Matrix{data, shape}
        self
    }
}
