use std::fs;
use polyfit_rs::polyfit_rs::polyfit;

pub fn mean_stddev(data: &Vec<f64>) -> (f64, f64)
{
    let (mut mean, mut sigma): (f64, f64) = (0f64,0f64);
    for i in 0..data.len() {mean += data[i];}
    let dlugosc: f64 = data.len() as f64;
    mean /= dlugosc;
    for i in 0..data.len() {sigma += (data[i]-mean).powf(2f64);}
    sigma = (sigma/(dlugosc-1f64)).powf(0.5f64);
    
    (mean,sigma)
}

pub fn rm_outliers(helptabx: &Vec<f64>, helptaby: &Vec<f64>, xs: &Vec<f64>, ys: &Vec<f64>) -> (Vec<f64>, Vec<f64>)
{
    let (meanf,sigmaf): (f64, f64) = mean_stddev(&helptaby);
    let mut xsh: Vec<f64> = xs.to_vec();
    let mut ysh: Vec<f64> = ys.to_vec();
    for i in 0..helptaby.len()
    {
        if helptaby.len() <= 3 { xsh.push(helptabx[i]); ysh.push(helptaby[i]); }
        else
        {
           if helptaby[i] <= meanf+sigmaf && helptaby[i] >= meanf-sigmaf { xsh.push(helptabx[i]); ysh.push(helptaby[i]); }
        }
    }
    
    (xsh, ysh)
}

pub fn read_data(path: &str, normback: bool, noback: bool) -> (Vec<f64>,Vec<f64>,Vec<f64>,f64,f64,usize,usize,Vec<f64>,Vec<f64>,Vec<f64>)
{
    let data: String = fs::read_to_string(path).expect("# Unable to read file!");
    let mut datavec: Vec<&str> = data.lines().collect();
    let fline: Vec<&str> = datavec[0].split(" ").collect();
    let (istart, istop): (&str,&str)  = (&fline[1],&fline[2]);
    let (mut iii, mut jjj): (i32, i32) = (istart.parse().unwrap(),istop.parse().unwrap());
    let (istart, istop): (usize,usize) = (istart.parse().unwrap(),istop.parse().unwrap());
    datavec = datavec.into_iter().filter(|&i| i.trim() != "" && i.chars().next().unwrap() != '#').collect::<Vec<_>>();
    let (mut time, mut flux, mut err): (Vec<f64>, Vec<f64>, Vec<f64>) = (vec![0f64;datavec.len()],vec![0f64;datavec.len()],vec![0f64;datavec.len()]);

    for i in 0..time.len()
    {
        let mut line: Vec<&str> = datavec[i].split(" ").collect();
        line = line.into_iter().filter(|&i| i.trim() != "").collect::<Vec<_>>();
        let (timeh, fluxh, errh): (&str, &str, &str) = (&line[0],&line[1],&line[2]);

        time[i] = timeh.parse().unwrap();
        flux[i] = fluxh.parse().unwrap();
        err[i] = errh.parse().unwrap();
    }

    let t0: f64 = time[0];
    for i in 0..time.len() {time[i] = (time[i]-t0)*24f64*60f64;}

    let (mut help_t1, mut help_f1): (Vec<f64>, Vec<f64>) = (vec![],vec![]);
    let (mut help_t2, mut help_f2): (Vec<f64>, Vec<f64>) = (vec![],vec![]);

    iii -= 11; jjj += 11;
    while iii <= jjj
    {   
        if (iii >= 0 && iii < time.len() as i32) && (iii < istart as i32 || iii > istop as i32) 
        {
            if iii >= 0 && iii < istart as i32 {help_t1.push(time[iii as usize]); help_f1.push(flux[iii as usize]);}
            else if iii < time.len() as i32 && iii > istop as i32 {help_t2.push(time[iii as usize]); help_f2.push(flux[iii as usize]);}
        }
        iii += 1;
    }

    let (mut b, mut a): (f64, f64) = (0f64, 0f64);
    if !noback && !normback
    {
        let (xs, ys): (Vec<f64>,Vec<f64>) = (vec![], vec![]);
        let (xs, ys): (Vec<f64>, Vec<f64>) = rm_outliers(&help_t1,&help_f1,&xs,&ys);
        let (xs, ys): (Vec<f64>, Vec<f64>) = rm_outliers(&help_t2,&help_f2,&xs,&ys); 
        let res: Vec<f64> = polyfit(&xs,&ys,1).unwrap();
        b = res[0]; a = res[1];
    }
    else if normback {b = 1f64;}

    let (mut time1, mut flux1, mut err1): (Vec<f64>,Vec<f64>,Vec<f64>) = (vec![0f64;istop-istart+1],vec![0f64;istop-istart+1],vec![0f64;istop-istart+1]);
    let mut fluxbak: Vec<f64> = vec![0f64;istop-istart+1];
    for i in istart..=istop
    {
        time1[i-istart] = time[i];
        flux1[i-istart] = flux[i]-(time[i]*a+b);
        err1[i-istart] = err[i];
        fluxbak[i-istart] = flux[i]
    }

    (time1,flux1,err1,a,b,istart,istop,fluxbak,time,flux)
}