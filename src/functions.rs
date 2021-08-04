//! Various mathematical functions commonly used in statistics.

#![allow(clippy::excessive_precision)]

use reverse::*;
use std::f64::consts::PI;

/// Calculates the standard [logistic function](https://en.wikipedia.org/wiki/Logistic_function)
pub fn logistic(x: Var) -> Var {
    1. / (1. + (-x).exp())
}

/// Calculates the [logit function](https://en.wikipedia.org/wiki/Logit)
pub fn logit(p: Var) -> Var {
    if !(0. ..=1.).contains(&p) {
        panic!("p must be in [0, 1]");
    }
    (p / (1. - p)).ln()
}

/// Calculates the one-parameter Box-Cox transformation with some power parameter `lambda`.
pub fn boxcox(x: Var, lambda: f64) -> Var {
    assert!(x > 0., "x must be positive");
    if lambda == 0. {
        x.ln()
    } else {
        (x.powf(lambda) - 1.) / lambda
    }
}

/// Calculates the two-parameter Box-Cox transformation with some power parameter `lambda` and some
/// shift parameter `alpha`.
pub fn boxcox_shifted(x: Var, lambda: f64, alpha: f64) -> Var {
    assert!(x > alpha, "x must larger than alpha");
    if lambda == 0. {
        (x + alpha).ln()
    } else {
        ((x + alpha).powf(lambda) - 1.) / lambda
    }
}

/// Calculates the softmax (the normalized exponential) function, which is a generalization of the
/// logistic function to multiple dimensions.
///
/// Takes in a vector of real numbers and normalizes it to a probability distribution such that
/// each of the components are in the interval (0, 1) and the components add up to 1. Larger input
/// components correspond to larger probabilities.
pub fn softmax<'a>(x: &[Var<'a>]) -> Vec<Var<'a>> {
    let sum_exp: Var<'a> = x.iter().map(|i| i.exp()).sum();
    x.iter().map(|i| i.exp() / sum_exp).collect()
}

const ERF_P: f64 = 0.3275911;
const ERF_A1: f64 = 0.254829592;
const ERF_A2: f64 = -0.284496736;
const ERF_A3: f64 = 1.421413741;
const ERF_A4: f64 = -1.453152027;
const ERF_A5: f64 = 1.061405429;

/// Calculates the [error function](https://en.wikipedia.org/wiki/Error_function) erf(x).
///
/// # Remarks
/// Uses Equation 7.1.26 in Stegun in combination with Horner's Rule.
pub fn erf(x: Var) -> Var {
    if x >= 0. {
        let t = 1. / (1. + ERF_P * x);
        1. - (((((ERF_A5 * t + ERF_A4) * t) + ERF_A3) * t + ERF_A2) * t + ERF_A1)
            * t
            * (-x * x).exp()
    } else {
        // erf is an odd function
        -erf(-x)
    }
}

const G: f64 = 4.7421875 + 1.;

/// Coefficients from [here](https://my.fit.edu/~gabdo/gamma.txt).
const GAMMA_COEFFS: [f64; 14] = [
    57.156235665862923517,
    -59.597960355475491248,
    14.136097974741747174,
    -0.49191381609762019978,
    0.33994649984811888699e-4,
    0.46523628927048575665e-4,
    -0.98374475304879564677e-4,
    0.15808870322491248884e-3,
    -0.21026444172410488319e-3,
    0.21743961811521264320e-3,
    -0.16431810653676389022e-3,
    0.84418223983852743293e-4,
    -0.26190838401581408670e-4,
    0.36899182659531622704e-5,
];

/// Calculates the [Gamma function](https://en.wikipedia.org/wiki/Gamma_function) using the [Lanczos
/// approximation](https://en.wikipedia.org/wiki/Lanczos_approximation). It obeys the equation
/// `gamma(x+1) = gamma(x) * x`. This approximation uses the reflection formula to extend the
/// calculation to the entire complex plane.
pub trait Gamma {
    fn gamma(self) -> Self;
}

impl Gamma for f64 {
    fn gamma(self) -> Self {
        if self < 0.5 {
            PI / ((PI * self).sin() * Self::gamma(1. - self))
        } else {
            let mut x = 0.99999999999999709182;
            for (idx, &val) in GAMMA_COEFFS.iter().enumerate() {
                x = x + val / ((self - 1.) + (idx as f64) + 1.);
            }
            let t = (self - 1.) + G - 0.5;
            ((2. * PI) as f64).sqrt() * t.powf((self - 1.) + 0.5) * (-t).exp() * x
        }
    }
}

impl<'a> Gamma for Var<'a> {
    fn gamma(self) -> Self {
        if self < 0.5 {
            PI / ((PI * self).sin() * Self::gamma(1. - self))
        } else {
            let mut x = self.tape.add_var(0.99999999999999709182);
            for (idx, &val) in GAMMA_COEFFS.iter().enumerate() {
                x = x + val / ((self - 1.) + (idx as f64) + 1.);
            }
            let t = (self - 1.) + G - 0.5;
            ((2. * PI) as f64).sqrt() * t.powf((self - 1.) + 0.5) * (-t).exp() * x
        }
    }
}

pub fn gamma<T: Gamma>(z: T) -> T {
    Gamma::gamma(z)
}

/// Calculates the [digamma function](https://en.wikipedia.org/wiki/Digamma_function), which is the
/// logarithmic derivative of the gamma function. It obeys the equation `digamma(x+1) = digamma(x)
/// + 1/x`. The approximation works better for large values. If the value is small, this function
/// will shift it up using the digamma recurrence relation.
pub fn digamma(x: Var) -> Var {
    if x < 6. {
        digamma(x + 1.) - 1. / x
    } else {
        x.ln() - 1. / (2. * x) - 1. / (12. * x.powi(2)) + 1. / (120. * x.powi(4))
            - 1. / (252. * x.powi(6))
            + 1. / (240. * x.powi(8))
            - 5. / (660. * x.powi(10))
            + 691. / (32760. * x.powi(12))
            - 1. / (12. * x.powi(14))
    }
}
