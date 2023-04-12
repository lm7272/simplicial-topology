use std::ops::AddAssign;

use nalgebra::DMatrix;
pub fn gaussian_elimination(mut a: DMatrix<i32>) -> DMatrix<i32>{
    for x in 0..a.nrows().max(a.ncols()){
        // if there exists some i/geq x and j\geq x s.t. a[i,j] = 1 then swap rows x and i and swap cols x and j
        match (x+1..a.nrows()).flat_map(|i| (x..a.ncols()).map(move |j| (i, j))).find(|&(i, j)| a[(i, j)] == 1) {
            Some((i, j)) => {
                a.swap_rows(x, i);
                a.swap_columns(x, j);
            },
            None => return a,
        };
        for i in x+1..a.nrows(){
            if a[(i,x)] == 1{
                let row_x = a.row(x).clone_owned();
                a.row_mut(i).add_assign(row_x);
                a.row_mut(i).iter_mut().for_each(|n| *n %= 2);
            }
        }
        for i in x+1..a.ncols(){
            if a[(x,i)] == 1{
                let col_x = a.column(x).clone_owned();
                a.column_mut(i).add_assign(col_x);
                a.column_mut(i).iter_mut().for_each(|n| *n %= 2);
            }
        }

    }
    a
}