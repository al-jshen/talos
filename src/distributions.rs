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
            -($upper - $lower).ln()
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
        -$b.ln() - ($var - $mu).abs() / $b
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

#[macro_export]
macro_rules! poisson {
    ( $n: expr; $lambda: expr ) => {
        assert!($n >= 0, "n must be a non-negative integer.");
        $n as f64 * $lambda.ln() - $lambda
    };
}

#[macro_export]
macro_rules! cauchy {
    ( $y: expr; $mu: expr, $sigma: expr ) => {{
        assert!($sigma > 0., "Sigma must be positive.");
        ($sigma / ($sigma.powi(2) + ($mu - $y).powi(2))).ln()
    }};
}

#[macro_export]
macro_rules! lognormal {
    ( $y: expr; $mu: expr, $sigma: expr ) => {{
        assert!($sigma > 0., "Sigma must be positive.");
        assert!($y > 0., "Varaible must be positive.");
        -0.5 * (($y.ln() - $mu) / $sigma).powi(2) - $y.ln() - $sigma.ln()
    }};
}

#[macro_export]
macro_rules! rayleigh {
    ( $y: expr; $sigma: expr) => {{
        assert!($sigma > 0., "Sigma must be positive.");
        assert!($y >= 0., "Variable must be non-negative.");
        -$y.powi(2) / (2. * $sigma.powi(2)) + y.ln() - 2. * $sigma.ln()
    }};
}

#[macro_export]
macro_rules! pareto {
    ( $y: expr; $ymin: expr, $alpha: expr) => {{
        assert!($ymin > 0., "y_min must be positive.");
        assert!($alpha > 0., "alpha must be positive.");
        assert!(
            $y >= $ymin,
            "Variable y must be at least as large as y_min."
        );
        $alpha.ln() + $alpha * $y_min.ln() - ($alpha - 1.) * $y.ln()
    }};
}
