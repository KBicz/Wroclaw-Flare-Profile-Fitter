use rand::Rng;

pub fn ea_cross(cr: f64, s: &Vec<Vec<Vec<f64>>>, v: &Vec<Vec<Vec<f64>>>, nprofiles: &usize,
                npop: &usize) -> Vec<Vec<Vec<f64>>>
{
    let mut u: Vec<Vec<Vec<f64>>> = s.clone().to_vec();
    let mut indexes: Vec<Vec<f64>> = vec![vec![0.0f64; *npop]; 4];
    let bounds: [(f64, f64); 4] = [(1.0, 12.0), (1.0, 6.0), (1.0, 6.0), (1.0, 8.0)];

    for k in 0..*nprofiles {

        for j in 0..4 {
            _ = rand::thread_rng().try_fill(&mut indexes[j][..]);
            for i in 0..*npop {if indexes[j][i] >= cr {u[k][j][i] = v[k][j][i];}}
        }

        for i in 0..*npop {
            for (j, &(min, max)) in bounds.iter().enumerate() {
                u[k][j][i] = u[k][j][i].clamp(min, max);
            }
        }
    }

    u
}