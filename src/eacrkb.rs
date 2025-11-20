use crate::{reparamkb, op};

pub fn ea_cr(
    s: &[Vec<Vec<f64>>],
    u: &[Vec<Vec<f64>>],
    time: &[f64],
    flux: &[f64],
    err: &[f64],
    nprofiles: usize,
    npop: usize,
) -> (Vec<Vec<Vec<f64>>>, usize, f64) {
    let mut parucd: bool;
    let mut min_chi = f64::MAX;
    let mut min_index = usize::MAX;
    let mut new_s = s.to_vec();

    // Pre-allocate vectors for profiles and their sums
    let mut flare_s = vec![0.0; time.len()];
    let mut flare_u = vec![0.0; time.len()];

    for i in 0..npop {
        let mut chi_s = 0.0;
        let mut chi_u = 0.0;
        parucd = true;

        for k in 0..nprofiles {
            // Precompute parameters for both `s` and `u`
            let pars = reparamkb::re_param(&[s[k][0][i], s[k][1][i], s[k][2][i],
                s[k][3][i]]);
            let paru = reparamkb::re_param(&[u[k][0][i], u[k][1][i], u[k][2][i],
                u[k][3][i]]);

            if paru[2]*paru[3] > 8.0
            { parucd = false; }
            // Generate profiles and accumulate their sums
            let profile_s = op::one_profile(time, pars[0], pars[1], pars[2],
                                            pars[3]);
            let profile_u = op::one_profile(time, paru[0], paru[1], paru[2],
                                            paru[3]);

            // Update flare sums directly
            for j in 0..time.len() {
                flare_s[j] += profile_s[j];
                flare_u[j] += profile_u[j];
            }
        }

        // Calculate chi-squared values
        for k in 0..time.len() {
            let diff_s = (flare_s[k] - flux[k]) / err[k];
            let diff_u = (flare_u[k] - flux[k]) / err[k];
            chi_s += diff_s * diff_s;
            chi_u += diff_u * diff_u;
        }

        // Update `new_s` if chi-squared of `u` is smaller
        if chi_u < chi_s && parucd {
            for k in 0..nprofiles {
                for j in 0..4 {
                    new_s[k][j][i] = u[k][j][i];
                }
            }
        }

        // Track minimum chi-squared and corresponding index
        let min_chi_i = chi_s.min(chi_u);
        if min_chi_i < min_chi {
            min_chi = min_chi_i;
            min_index = i;
        }

        // Clear flare sums for next iteration
        flare_s.fill(0.0);
        flare_u.fill(0.0);
    }

    (new_s, min_index, min_chi)
}
