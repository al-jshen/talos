use compute::linalg::Vector;

pub enum Data {
    Float(f64),
    Int(i32),
    FloatArray(Vector),
    IntArray(Vec<i32>),
}

#[macro_export]
macro_rules! unpack {
    ( $var: expr, $t: ident ) => {{
        match &$var {
            Data::$t(d) => d,
            _ => panic!("Type to unpack must be a variant of the Data enum (Float, Int, FloatArray, IntArray)."),
        }
    }};
}

#[macro_export]
macro_rules! unpack_tuple {
    ( $data: expr, $n: expr ) => {
        seq_macro::seq!(i in 0..$n {
            ( #( $data[i], )* )
        }
    )};
}
