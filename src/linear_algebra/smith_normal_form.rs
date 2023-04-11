use nalgebra::DMatrix;
pub fn pivot_rows_and_cols(a: &DMatrix<i32>, x: usize) -> DMatrix<i32>{
    let (k, l) = match (x..a.nrows()).flat_map(|i| (x..a.ncols()).map(move |j| (i, j))).find(|&(i, j)| a[(i, j)] == 1) {
        Some((i, j)) => (i, j),
        None => return a.clone(),
    };
    let mut b = a.clone();
    println!("{}, {}", k, l);
    b.swap_rows(x, k);
    b.swap_columns(x, l);
    b
}

// pub fn func(a: DMatrix<i32>, x: usize) -> DMatrix<i32>{
//     for i in x..a.nrows(){
//         if a[(i,x)] == 1{
//             a.row(i) += a.row(x);
//         }
//     }
// }