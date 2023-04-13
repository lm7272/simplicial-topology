use std::ops::AddAssign;
use nalgebra::DMatrix;

pub fn gaussian_elimination(mut matrix: DMatrix<i32>) -> DMatrix<i32>{
    for x in 0..matrix.nrows().max(matrix.ncols()){
        // if there exists some i/geq x and j\geq x s.t. a[i,j] = 1 then swap rows x and i and swap cols x and j
        match (x+1..matrix.nrows()).flat_map(|i| (x..matrix.ncols()).map(move |j| (i, j))).find(|&(i, j)| matrix[(i, j)] == 1) {
            Some((i, j)) => {
                matrix.swap_rows(x, i);
                matrix.swap_columns(x, j);
            },
            None => return matrix,
        };
        for i in x+1..matrix.nrows(){
            if matrix[(i,x)] == 1{
                let row_x = matrix.row(x).clone_owned();
                matrix.row_mut(i).add_assign(row_x);
                matrix.row_mut(i).iter_mut().for_each(|n| *n %= 2);
            }
        }
        for i in x+1..matrix.ncols(){
            if matrix[(x,i)] == 1{
                let col_x = matrix.column(x).clone_owned();
                matrix.column_mut(i).add_assign(col_x);
                matrix.column_mut(i).iter_mut().for_each(|n| *n %= 2);
            }
        }

    }
    matrix
}

pub fn rank(smith_normal_matrix: DMatrix<i32>) -> usize{
    let n = smith_normal_matrix.nrows().min(smith_normal_matrix.ncols());
    let eliminated_matrix = gaussian_elimination(smith_normal_matrix);
    let mut running_rank = 0;
    for i in 0..n{
        if eliminated_matrix[(i,i)] != 0{
            running_rank += 1;
        }
        else {
            return running_rank
        }
    }
    running_rank
}

pub fn row_nullity(matrix: DMatrix<i32>) -> usize{
    matrix.nrows() - rank(matrix)
}

pub fn columns_nullity(matrix: DMatrix<i32>) -> usize{
    matrix.ncols() - rank(matrix)
}