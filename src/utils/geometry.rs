use nalgebra::DMatrix;
use ndarray::Array1;

fn euclidean_distance(x: &Array1<f64>, y: &Array1<f64>) -> f64 {
    assert_eq!(x.len(), y.len(), "vectors must have the same length");
    let squared_distance = x.iter().zip(y.iter())
        .map(|(&a, &b)| (a - b).powi(2))
        .sum::<f64>();
    squared_distance.sqrt()
}

fn distance_matrix(v: Vec<&Array1<f64>>) -> DMatrix<f64>{
    let n = v.len();
    let mut A: DMatrix<f64> = DMatrix::from_element(n, n, 0.0);
    for i in 0..(n-1){
        for j in (i+1)..n {
            let d = euclidean_distance(&v[i], &v[j]);
            A[(i, j)] = d;
            A[(j, i)] = d;
        }
    }
    A
}