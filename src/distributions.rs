use crate::functions;

#[macro_export]
macro_rules! normal {
    ( $var: expr ; $mean: expr, $sigma: expr  ) => {{
        assert!($sigma > 0., "Sigma must be positive.");
        -$sigma.ln() - 0.5_f64 * (($var - $mean) / $sigma).powi(2)
    }};
}

#[macro_export]
macro_rules! exponential {
    ( $var: expr ; $rate: expr ) => {{
        assert!($rate > 0., "Rate parameter must be positive.");
        $rate.ln() * (-$rate * $var)
    }};
}

#[macro_export]
macro_rules! uniform {
    ( $var: expr; $lower: expr, $upper: expr ) => {{
        assert!($upper > $lower, "Upper must be greater than lower.");
        if $lower <= $var && $var <= $upper {
            (1._f64 / ($upper - $lower)).ln()
        } else {
            f64::NEG_INFINITY
        }
    }};
}

#[macro_export]
macro_rules! gamma {
    ($var: expr; $alpha: expr, $beta: expr) => {{
        assert!($var > 0., "Variable must be positive.");
        assert!($alpha > 0., "Alpha must be positive.");
        assert!($beta > 0., "Beta must be positive.");
        $alpha * $beta.ln() + ($alpha - 1_f64) * $var.ln()
            - ($beta * $var)
            - functions::gamma($alpha).ln()
    }};
}

#[macro_export]
macro_rules! laplace {
    ( $var: expr; $mu: expr, $b: expr ) => {{
        assert!($b > 0., "Scale parameter `b` must be positive.");
        (1_f64 / (2_f64 * $b)).ln() - ($var - $mu).abs() / $b
    }};
}

#[macro_export]
macro_rules! beta {
    ( $var: expr; $alpha: expr, $beta: expr ) => {{
        assert!(
            ($var >= 0.) && ($var <= 1.),
            "Variable must be between 0. and 1."
        );
        assert!($alpha > 0., "Alpha must be positive.");
        assert!($beta > 0., "Beta must be positive.");
        ($alpha - 1.) * $var.ln() + ($beta - 1.) * (1. - $var).ln()
            - functions::beta($alpha, $beta).ln()
    }};
}

#[macro_export]
macro_rules! bernoulli {
    ( $var: expr; $theta: expr ) => {{
        assert!(
            ($theta >= 0.) && ($theta <= 1.),
            "Theta must be between 0 and 1."
        );
        match $var {
            0 => 1. - $theta,
            1 => $theta,
            _ => panic!("Variable must be an integer that is either either 0 or 1."),
        }
    }};
}

#[macro_export]
macro_rules! binomial {
    ( $n: expr; $N: expr, $theta: expr ) => {{
        assert!(
            ($theta >= 0.) && ($theta <= 1.),
            "Theta must be between 0 and 1."
        );
        assert!($N > 0, "N must be a positive integer.");
        assert!($n >= 0, "n must be a non-negative integer.");
        let success = $n as f64;
        let trials = $N as f64;
        // functions::gamma(trials + 1.).ln()
        //     - functions::gamma(success + 1.).ln()
        //     - functions::gamma(trials - success + 1.).ln() +
        success * $theta.ln() + (trials - success) * (1. - $theta).ln()
    }};
}
