use libm::{exp,erf};

pub const PI: f64 = 3.14159265358979323846264338327950288f64;

pub fn one_profile(t: &Vec<f64>, a: f64, b: f64, c: f64, d: f64) -> Vec<f64>
{
    let (a, b, c, d) = (a.abs(), b.abs(), c.abs(), d.abs());
    let (mut th1,mut f11,mut f21);
    let mut flare: Vec<f64> = vec![0f64;t.len()];
    let (om1, mut val1) = (c*d/2f64,1f64);

    if b/c+(c*d)/2.0 < 2.0 {val1 = erf(b/c+(c*d)/2.0);}

    for i in 0..t.len()
    {
        th1 = (t[i]-b)/c;
        if (-2.0*om1*th1+om1.powf(2.0)).abs() > 600f64 {return vec![0f64; t.len()];} 
        
        f11 = 0.5*PI.powf(0.5)*a*c*exp(-2.0*om1*th1+om1.powf(2.0));
        f21 = val1 + erf(th1-om1);
        flare[i] = (f11*f21).abs();
    }

    return flare;
}