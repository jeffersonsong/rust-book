#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                tem_vec.push($x);
            )*
            temp_vec
        }
    };
}
