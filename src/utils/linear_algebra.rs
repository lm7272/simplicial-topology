use std::ops::AddAssign;
use nalgebra::DMatrix;

pub fn rank(mut matrix: DMatrix<i32>, start_val: i32) -> i32{
    match (1..matrix.nrows()).flat_map(|i| (0..matrix.ncols()).map(move |j| (i, j))).find(|&(i, j)| matrix[(i, j)] == 1) {
        Some((i, j)) => {
            matrix.swap_rows(0, i);
            matrix.swap_columns(0, j);
        },
        None => return start_val,
    };
    for i in 1..matrix.nrows(){
        if matrix[(i,0)] == 1{
            let row = matrix.row(0).clone_owned();
            matrix.row_mut(i).add_assign(row);
            matrix.row_mut(i).iter_mut().for_each(|n| *n %= 2);
        }
    }
    for i in 1..matrix.ncols(){
        if matrix[(0,i)] == 1{
            let col = matrix.column(0).clone_owned();
            matrix.column_mut(i).add_assign(col);
            matrix.column_mut(i).iter_mut().for_each(|n| *n %= 2);
        }
    }
    rank(matrix.rows_range(1..).columns_range(1..).into(), start_val+1)
}

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


pub fn rank_smith_normal_matrix(smith_normal_matrix: &DMatrix<i32>) -> i32{
    let n = smith_normal_matrix.nrows().min(smith_normal_matrix.ncols());
    let mut running_rank = 0;
    for i in 0..n{
        if smith_normal_matrix[(i,i)] != 0{
            running_rank += 1;
        }
        else {
            return running_rank
        }
    }
    running_rank
}

pub fn row_nullity_smith_normal_matrix(smith_normal_matrix: &DMatrix<i32>) -> i32{
    i32::try_from(smith_normal_matrix.nrows()).unwrap() - rank_smith_normal_matrix(smith_normal_matrix)
}

pub fn columns_nullity(smith_normal_matrix: &DMatrix<i32>) -> i32{
    i32::try_from(smith_normal_matrix.ncols()).unwrap() - rank_smith_normal_matrix(smith_normal_matrix)
}