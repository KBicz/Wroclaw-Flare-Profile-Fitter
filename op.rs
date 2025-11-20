use libm::{exp, erf};

pub const PI: f64 = 3.14159265358979323846264338327950288;

pub fn one_profile(t: &[f64], a: f64, b: f64, c: f64, d: f64) -> Vec<f64> {
    // Take absolute values of parameters
    let (a, b, c, d) = (a.abs(), b.abs(), c.abs(), d.abs());

    // Pre-compute constants
    let om1 = c * d / 2.0;
    let val1 = if b / c + om1 < 2.0 {
        erf(b / c + om1)
    } else {
        1.0
    };
    let sqrt_pi_half = 0.5 * PI.sqrt();

    // Initialize result vector with pre-allocated capacity
    let mut flare = Vec::with_capacity(t.len());

    for &time in t {
        let th1 = (time - b) / c;

        // Avoid computation if the exponential term would overflow
        if (-2.0 * om1 * th1 + om1.powi(2)).abs() > 600.0 {
            return vec![0.0; t.len()];
        }

        // Compute components
        let exp_term = exp(-2.0 * om1 * th1 + om1.powi(2));
        let erf_term = val1 + erf(th1 - om1);

        // Compute flare value and store in the vector
        let flare_value = sqrt_pi_half * a * c * exp_term * erf_term.abs();
        flare.push(flare_value);
    }

    flare
}
