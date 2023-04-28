#[macro_export]
macro_rules! sc {
    ( $( $x:expr ),* ) => {
        SimplicialComplex::new_from_vec(vec![ $( $x ),* ])
    };
}

#[macro_export]
macro_rules! simplex {
    ( $( $x:expr ),* ) => {
        Simplex::new(vec![ $( $x ),* ])
    };
}