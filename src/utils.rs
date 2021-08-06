use compute::linalg::Vector;

pub enum Data {
    Float(f64),
    Int(i32),
    FloatArray(Vector),
    IntArray(Vec<i32>),
}

#[macro_export]
macro_rules! unpack {
    ( $var: expr; $t: path ) => {{
        match &$var {
            $t(d) => d,
            _ => panic!(),
        }
    }};
}
