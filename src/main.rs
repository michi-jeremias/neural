use std::collections::HashMap;

use neural::matrix::Matrix;

fn main() {
    let m1 = Matrix::new(vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]);
    println!("{:?}", &m1);

    let m2 = Matrix::new(vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]);
    println!("{:?}", &m2);

    println!("{:?}", m1.is_compatible(m2));

    let mut map = HashMap::new();
    for i in 0..3 {
        for j in 0..3 {
            map.insert((i, j), i+j);
        }
    }
    println!("map: {:?}", map);
    
}

