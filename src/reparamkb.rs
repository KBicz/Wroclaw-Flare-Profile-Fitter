pub fn re_param(args: &[f64]) -> Vec<f64> {
    let exponents = [-6.0, -3.0, -3.0, -5.0];
    let mut result = Vec::with_capacity(4);

    for (&arg, &exp) in args.iter().zip(exponents.iter()) {
        result.push(10f64.powf(arg + exp));
    }

    result
}
