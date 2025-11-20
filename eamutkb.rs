use rand::Rng;

pub fn ea_mut(f: f64, s: &Vec<Vec<Vec<f64>>>, t: usize, nprofiles: &usize, npop: &usize)
    -> Vec<Vec<Vec<f64>>>
{
    let mut v: Vec<Vec<Vec<f64>>> = s.to_vec().clone();
    let mut indexes: Vec<Vec<usize>> = vec![vec![0; 4]; *npop];

    for i in 0..*npop {
        for j in 0..4 {
            indexes[i][j] = rand::thread_rng().gen_range(0..*npop);
        }
    }

    for j in 0..*nprofiles {
        for i in 0..*npop {
            for (dim, &(min, max)) in [(1.0, 12.0), (1.0, 6.0), (1.0, 6.0),
                (1.0, 8.0)].iter().enumerate() {
                v[j][dim][i] = s[j][dim][t] + f * (s[j][dim][indexes[i][1]] -
                    s[j][dim][indexes[i][2]]);
                v[j][dim][i] = v[j][dim][i].clamp(min, max);
            }
        }
    }

    v
}