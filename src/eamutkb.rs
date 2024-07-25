use rand::Rng;
use std::process::exit;

pub fn ea_mut(f: f64, s: &Vec<Vec<f64>>, t: usize) -> Vec<Vec<f64>>
{
    let mut v: Vec<Vec<f64>> = s.to_vec().clone();
    let ni: usize = v[0].clone().len();
    let mut indexes: Vec<Vec<usize>> = vec![vec![0; 3]; ni];

    for i in 0..ni { for j in 0..3 {indexes[i][j] = rand::thread_rng().gen_range(0..ni); } }

    match v.len()
    {
        4 => {
            for i in 0..ni
            {
                v[0][i] = s[0][t] + f * (s[0][indexes[i][1]]-s[0][indexes[i][2]]);
                if v[0][i] <  1f64 {v[0][i] =  1f64;}
                if v[0][i] > 12f64 {v[0][i] = 12f64;}
                v[1][i] = s[1][t] + f * (s[1][indexes[i][1]]-s[1][indexes[i][2]]);
                if v[1][i] <  1f64 {v[1][i] =  1f64;}
                if v[1][i] >  6f64 {v[1][i] =  6f64;}
                v[2][i] = s[2][t] + f * (s[2][indexes[i][1]]-s[2][indexes[i][2]]);
                if v[2][i] <  1f64 {v[2][i] =  1f64;}
                if v[2][i] >  6f64 {v[2][i] =  6f64;}
                v[3][i] = s[3][t] + f * (s[3][indexes[i][1]]-s[3][indexes[i][2]]);
                if v[3][i] <  1f64 {v[3][i] =  1f64;}
                if v[3][i] >  8f64 {v[3][i] =  8f64;}
            }
        },
        7 => {
            for i in 0..ni
            {
                v[0][i] = s[0][t] + f * (s[0][indexes[i][1]]-s[0][indexes[i][2]]);
                if v[0][i] <  1f64 {v[0][i] =  1f64;}
                if v[0][i] > 12f64 {v[0][i] = 12f64;}
                v[1][i] = s[1][t] + f * (s[1][indexes[i][1]]-s[1][indexes[i][2]]);
                if v[1][i] <  1f64 {v[1][i] =  1f64;}
                if v[1][i] >  6f64 {v[1][i] =  6f64;}
                v[2][i] = s[2][t] + f * (s[2][indexes[i][1]]-s[2][indexes[i][2]]);
                if v[2][i] <  1f64 {v[2][i] =  1f64;}
                if v[2][i] >  6f64 {v[2][i] =  6f64;}
                v[3][i] = s[3][t] + f * (s[3][indexes[i][1]]-s[3][indexes[i][2]]);
                if v[3][i] <  1f64 {v[3][i] =  1f64;}
                if v[3][i] >  8f64 {v[3][i] =  8f64;}
                v[4][i] = s[4][t] + f * (s[4][indexes[i][1]]-s[4][indexes[i][2]]);
                if v[4][i] <  1f64 {v[4][i] =  1f64;}
                if v[4][i] > 12f64 {v[4][i] = 12f64;}
                v[5][i] = s[5][t] + f * (s[5][indexes[i][1]]-s[5][indexes[i][2]]);
                if v[5][i] <  1f64 {v[5][i] =  1f64;}
                if v[5][i] >  6f64 {v[5][i] =  6f64;}
                v[6][i] = s[6][t] + f * (s[6][indexes[i][1]]-s[6][indexes[i][2]]);
                if v[6][i] <  1f64 {v[6][i] =  1f64;}
                if v[6][i] >  8f64 {v[6][i] =  8f64;}
            }
        },
        8 => {
            for i in 0..ni
            {
                v[0][i] = s[0][t] + f * (s[0][indexes[i][1]]-s[0][indexes[i][2]]);
                if v[0][i] <  1f64 {v[0][i] =  1f64;}
                if v[0][i] > 12f64 {v[0][i] = 12f64;}
                v[1][i] = s[1][t] + f * (s[1][indexes[i][1]]-s[1][indexes[i][2]]);
                if v[1][i] <  1f64 {v[1][i] =  1f64;}
                if v[1][i] >  6f64 {v[1][i] =  6f64;}
                v[2][i] = s[2][t] + f * (s[2][indexes[i][1]]-s[2][indexes[i][2]]);
                if v[2][i] <  1f64 {v[2][i] =  1f64;}
                if v[2][i] >  6f64 {v[2][i] =  6f64;}
                v[3][i] = s[3][t] + f * (s[3][indexes[i][1]]-s[3][indexes[i][2]]);
                if v[3][i] <  1f64 {v[3][i] =  1f64;}
                if v[3][i] >  8f64 {v[3][i] =  8f64;}
                v[4][i] = s[4][t] + f * (s[4][indexes[i][1]]-s[4][indexes[i][2]]);
                if v[4][i] <  1f64 {v[4][i] =  1f64;}
                if v[4][i] > 12f64 {v[4][i] = 12f64;}
                v[5][i] = s[5][t] + f * (s[5][indexes[i][1]]-s[5][indexes[i][2]]);
                if v[5][i] <  1f64 {v[5][i] =  1f64;}
                if v[5][i] >  6f64 {v[5][i] =  6f64;}
                v[6][i] = s[6][t] + f * (s[6][indexes[i][1]]-s[6][indexes[i][2]]);
                if v[6][i] <  1f64 {v[6][i] =  1f64;}
                if v[6][i] >  6f64 {v[6][i] =  6f64;}
                v[7][i] = s[7][t] + f * (s[7][indexes[i][1]]-s[7][indexes[i][2]]);
                if v[7][i] <  1f64 {v[7][i] =  1f64;}
                if v[7][i] >  8f64 {v[7][i] =  8f64;}
            }
        },
        _ => exit(0),
    }

    v
}