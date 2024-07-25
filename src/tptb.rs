use libm::{exp,erf};

pub const PI: f64 = 3.14159265358979323846264338327950288f64;

pub fn two_profiles_two_b(t: &Vec<f64>, a1: f64, b1: f64, c1: f64, d1: f64, a2: f64, b2: f64, c2: f64, d2: f64) -> Vec<f64>
{
    let (a1, b1, c1, d1) = (a1.abs(), b1.abs(), c1.abs(), d1.abs());
    let (a2, b2, c2, d2) = (a2.abs(), b2.abs(), c2.abs(), d2.abs());
    let (mut th1,mut th2,mut f11,mut f12,mut f21,mut f22);
    let mut flare: Vec<f64> = vec![0f64;t.len()];
    let (om1, om2) = (c1*d1/2.0_f64,c2*d2/2.0_f64);
    let (mut val1, mut val2) = (1f64,1f64);

    if b1/c1+(c1*d1)/2. < 2.0 {val1 = erf(b1/c1+(c1*d1)/2.);}
    if b2/c2+(c2*d2)/2. < 2.0 {val2 = erf(b2/c2+(c2*d2)/2.);}
    

    for i in 0..t.len()
    {
        th1 = (t[i]-b1)/c1;
        th2 = (t[i]-b2)/c2;

        if (-2.0*om1*th1+om1.powf(2.0)).abs() > 650f64 || (-2.0*om2*th2+om2.powf(2.0)).abs() > 650f64 {return vec![0f64; t.len()];} 
        f11 = 0.5*PI.powf(0.5)*a1*c1*exp(-2.0*om1*th1+om1.powf(2.0));
        f12 = 0.5*PI.powf(0.5)*a2*c2*exp(-2.0*om2*th2+om2.powf(2.0));

        f21 = val1 + erf(th1-om1);
        f22 = val2 + erf(th2-om2);
        flare[i] = (f11*f21).abs() + (f12*f22).abs();
    }

    return flare;
}