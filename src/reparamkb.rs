use std::process::exit;

pub fn re_param(args: &Vec<f64>) -> Vec<f64>
{
    match args.len()
    {
        4 => {
            let mut an = 10f64.powf(args[0]-6f64);
            let mut bn = 10f64.powf(args[1]-3f64);
            let mut cn = 10f64.powf(args[2]-3f64);
            let mut dn = 10f64.powf(args[3]-5f64);
            if an < 0f64 {an = 0f64;}
            if bn < 0f64 {bn = 0f64;}
            if cn < 0f64 {cn = 0f64;}
            if dn < 0f64 {dn = 0f64;}
            return vec![an,bn,cn,dn];
        },
        7 => {
            let mut a1n = 10f64.powf(args[0]-6f64);
            let mut bn = 10f64.powf(args[1]-3f64);
            let mut c1n = 10f64.powf(args[2]-3f64);
            let mut d1n = 10f64.powf(args[3]-5f64);
            let mut a2n = 10f64.powf(args[4]-6f64);
            let mut c2n = 10f64.powf(args[5]-3f64);
            let mut d2n = 10f64.powf(args[6]-5f64);
            if a1n < 0f64 {a1n = 0f64;}
            if bn < 0f64 {bn = 0f64;}
            if c1n < 0f64 {c1n = 0f64;}
            if d1n < 0f64 {d1n = 0f64;}
            if a2n < 0f64 {a2n = 0f64;}
            if c2n < 0f64 {c2n = 0f64;}
            if d2n < 0f64 {d2n = 0f64;}
            return vec![a1n,bn,c1n,d1n,a2n,c2n,d2n];
        },
        8 => {
            let mut a1n = 10f64.powf(args[0]-6f64);
            let mut b1n = 10f64.powf(args[1]-3f64);
            let mut c1n = 10f64.powf(args[2]-3f64);
            let mut d1n = 10f64.powf(args[3]-5f64);
            let mut a2n = 10f64.powf(args[4]-6f64);
            let mut b2n = 10f64.powf(args[5]-3f64);
            let mut c2n = 10f64.powf(args[6]-3f64);
            let mut d2n = 10f64.powf(args[7]-5f64);
            if a1n < 0f64 {a1n = 0f64;}
            if b1n < 0f64 {b1n = 0f64;}
            if c1n < 0f64 {c1n = 0f64;}
            if d1n < 0f64 {d1n = 0f64;}
            if a2n < 0f64 {a2n = 0f64;}
            if b2n < 0f64 {b2n = 0f64;}
            if c2n < 0f64 {c2n = 0f64;}
            if d2n < 0f64 {d2n = 0f64;}
            return vec![a1n,b1n,c1n,d1n,a2n,b2n,c2n,d2n];
        }
        _ => exit(0),
    }
}