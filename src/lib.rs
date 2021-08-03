#[macro_export]
macro_rules! normal {
    ( $var: expr ; $mean: expr, $sigma: expr  ) => {{
        assert!($sigma > 0., "Sigma must be positive.");
        -$sigma.ln() - 0.5 * (($var - $mean) / $sigma).powi(2)
    }};
}

#[macro_export]
macro_rules! exponential {
    ( $var: expr ; $rate: expr ) => {{
        assert!($rate > 0., "Rate parameter must be positive.");
        $rate.ln() * (-$rate * $var)
    }};
}
