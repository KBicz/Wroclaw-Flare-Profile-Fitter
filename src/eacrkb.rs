use crate::{reparamkb, op, tpob, tptb};

pub fn ea_cr(s: &Vec<Vec<f64>>, u: &Vec<Vec<f64>>, time: &Vec<f64>, flux: &Vec<f64>, err: &Vec<f64>) -> (Vec<Vec<f64>>,usize,f64)
{    
    let mut minchi: f64;
    let mut min_ti: f64 = 0f64;
    let mut min_t: usize = 100000;
    let (mut chi_s, mut chi_u): (f64, f64);
    let mut new_s: Vec<Vec<f64>> = s.to_vec();
    let (mut flare_s, mut flare_u, mut pars, mut paru): (Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>);
    
    for i in 0..s[0].len()
    {
        match s.len()
        {
            4 => {
                pars = reparamkb::re_param(&vec![s[0][i],s[1][i],s[2][i],s[3][i]]);
                paru = reparamkb::re_param(&vec![u[0][i],u[1][i],u[2][i],u[3][i]]);
                flare_s = op::one_profile(time,pars[0],pars[1],pars[2],pars[3]);
                flare_u = op::one_profile(time,paru[0],paru[1],paru[2],paru[3]);
            },
            7 => {
                pars = reparamkb::re_param(&vec![s[0][i],s[1][i],s[2][i],s[3][i],s[4][i],s[5][i],s[6][i]]);
                paru = reparamkb::re_param(&vec![u[0][i],u[1][i],u[2][i],u[3][i],u[4][i],u[5][i],u[6][i]]);
                flare_s = tpob::two_profiles_one_b(time,pars[0],pars[1],pars[2],pars[3],pars[4],pars[5],pars[6]);
                flare_u = tpob::two_profiles_one_b(time,paru[0],paru[1],paru[2],paru[3],paru[4],paru[5],paru[6]);
            },
            8 => {
                pars = reparamkb::re_param(&vec![s[0][i],s[1][i],s[2][i],s[3][i],s[4][i],s[5][i],s[6][i],s[7][i]]);
                paru = reparamkb::re_param(&vec![u[0][i],u[1][i],u[2][i],u[3][i],u[4][i],u[5][i],u[6][i],u[7][i]]);
                flare_s = tptb::two_profiles_two_b(time,pars[0],pars[1],pars[2],pars[3],pars[4],pars[5],pars[6],pars[7]);
                flare_u = tptb::two_profiles_two_b(time,paru[0],paru[1],paru[2],paru[3],paru[4],paru[5],paru[6],paru[7]);
            },
            _ => std::process::exit(1),
        }

        chi_s = 0.0; chi_u = 0.0;

        for k in 0..time.len()
        {
            chi_s += ((flare_s[k]-flux[k])/err[k]).powf(2.0);
            chi_u += ((flare_u[k]-flux[k])/err[k]).powf(2.0);
        }
        
        if chi_s >= chi_u {for j in 0..new_s.len() {new_s[j][i] = u[j][i];} }
        minchi = chi_s;
        if chi_u < minchi {minchi = chi_u;}
        if i == 0 || min_ti > minchi
        {
            min_ti = minchi;  
            min_t = i;
        }
    }
    
    (new_s, min_t, min_ti)
}