use rand::Rng;
use std::process::exit;

pub fn ea_cross(cr: f64, s: &Vec<Vec<f64>>, v: &Vec<Vec<f64>>) -> Vec<Vec<f64>>
{
    let mut u: Vec<Vec<f64>> = s.clone().to_vec();
    let (length, width): (usize, usize) = (u.len(),u[0].len());
    let mut indexes: Vec<Vec<f64>> = vec![vec![0.0f64; width]; length];

    for j in 0..length
    {
        _ = rand::thread_rng().try_fill(&mut indexes[j][..]);
        for i in 0..width {if indexes[j][i] >= cr {u[j][i] = v[j][i];}}
    }

    for i in 0..width
    {
        match length
        {
            4 => {
                if u[0][i] < 1f64 {u[0][i] = 1f64;}
                if u[0][i] > 12f64 {u[0][i] = 12f64;}  
                if u[1][i] < 1f64 {u[1][i] = 1f64;}
                if u[1][i] > 6f64 {u[1][i] = 6f64;}  
                if u[2][i] < 1f64 {u[2][i] = 1f64;}
                if u[2][i] > 6f64 {u[2][i] = 6f64;}  
                if u[3][i] < 1f64 {u[3][i] = 1f64;}
                if u[3][i] > 8f64 {u[3][i] = 8f64;}  
            },
            7 => {
                if u[0][i] < 1f64 {u[0][i] = 1f64;}
                if u[0][i] > 12f64 {u[0][i] = 12f64;}  
                if u[1][i] < 1f64 {u[1][i] = 1f64;}
                if u[1][i] > 6f64 {u[1][i] = 6f64;}  
                if u[2][i] < 1f64 {u[2][i] = 1f64;}
                if u[2][i] > 6f64 {u[2][i] = 6f64;}  
                if u[3][i] < 1f64 {u[3][i] = 1f64;}
                if u[3][i] > 8f64 {u[3][i] = 8f64;}  
                if u[4][i] < 1f64 {u[4][i] = 1f64;}
                if u[4][i] > 12f64 {u[4][i] = 12f64;}  
                if u[5][i] < 1f64 {u[5][i] = 1f64;}
                if u[5][i] > 6f64 {u[5][i] = 6f64;}  
                if u[6][i] < 1f64 {u[6][i] = 1f64;}
                if u[6][i] > 8f64 {u[6][i] = 8f64;}  
            },
            8 => {
                if u[0][i] < 1f64 {u[0][i] = 1f64;}
                if u[0][i] > 12f64 {u[0][i] = 12f64;}  
                if u[1][i] < 1f64 {u[1][i] = 1f64;}
                if u[1][i] > 6f64 {u[1][i] = 6f64;}  
                if u[2][i] < 1f64 {u[2][i] = 1f64;}
                if u[2][i] > 6f64 {u[2][i] = 6f64;}  
                if u[3][i] < 1f64 {u[3][i] = 1f64;}
                if u[3][i] > 8f64 {u[3][i] = 8f64;}  
                if u[4][i] < 1f64 {u[4][i] = 1f64;}
                if u[4][i] > 12f64 {u[4][i] = 12f64;}  
                if u[5][i] < 1f64 {u[5][i] = 1f64;}
                if u[5][i] > 6f64 {u[5][i] = 6f64;}  
                if u[6][i] < 1f64 {u[6][i] = 1f64;}
                if u[6][i] > 6f64 {u[6][i] = 6f64;}  
                if u[7][i] < 1f64 {u[7][i] = 1f64;}
                if u[7][i] > 8f64 {u[7][i] = 8f64;} 
            },
            _ => exit(2137),
        }
    }
    
    u
}