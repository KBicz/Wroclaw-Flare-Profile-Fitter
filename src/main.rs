mod op;
mod tpob;
mod tptb;
mod fplotd;
mod eacrkb;
mod readdkb;
mod eamutkb;
mod reparamkb;
mod eacrosskb;

use rand::Rng;
use std::{fs,env};
use ansi_term::Style;
use std::time::Instant;
use std::process::exit;
use indicatif::{ProgressStyle,ProgressBar};

fn helpf() 
{
    println!("\n Program ea_main_rust, version of 02 Jun 2021.");
    println!(" Usage: ea_main_rust <-lc=file> [-nparams=int] [-npop=int] [-niter=int] [--normback] [--noback] [--plot] [--save]");
    println!("\n    nparams = 4 or 7 or 8");
    println!("      niter   = 10000");
    println!("      popul   = 200\n");
    exit(2137);
}

fn ea_main(file: &str, params: usize, n_s: usize, plotctrl: bool, savectrl: bool, normback: bool, noback: bool, ntryes: u64)
{
    let start: Instant = Instant::now();
    let mut t: usize = 10;
    let nss: f64 = n_s as f64;
    let mut minchi: f64 = 0f64;
    let mut s: Vec<Vec<f64>> = vec![];
    let mut time2: Vec<f64> = vec![0f64;1000];
    let mut background: Vec<f64> = vec![0f64;1000];
    let (mut leg1, mut leg2): (&str, &str) = ("","");
    let pb: ProgressBar = ProgressBar::new(ntryes);
    let mut scalars: Vec<f64> = vec![12f64,6f64,6f64,8f64,12f64,6f64,6f64,8f64];
    let (para, profile, profile1, profile2): (Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>);
    let (mut m, mut v, mut u, mut result, tit): (f64, Vec<Vec<f64>>, Vec<Vec<f64>>, (Vec<Vec<f64>>,usize,f64), String);
    let (time,flux,err,a,b,_istart,_istop,fluxbak,timefull,fluxfull): (Vec<f64>, Vec<f64>, Vec<f64>, f64, f64, usize, usize, Vec<f64>, Vec<f64>, Vec<f64>) = readdkb::read_data(file,normback,noback);
    let mut diff: f64 = time[time.len()-1]-time[0];
    let mut nel: f64 = time2.len() as f64;
    let system: &str = env::consts::OS;
    diff /= nel;
    pb.set_style(ProgressStyle::with_template(&format!("{} {} {{spinner:.green}} [{{elapsed_precise}}] ╢{{bar:45.white/gray}}╟ {{percent}}% [{{eta_precise}}, {{per_sec}}]",Style::new().bold().paint("Fitting").to_string(),Style::new().bold().paint("profile").to_string())).unwrap());

    if n_s <= 10 
    {
        let temp_t = (nss-1f64)/2f64;
        t = temp_t.round() as usize;
    }

    if params == 7 {scalars[6] = 8f64;}
    for i in 0..params
    {
        let mut indeksy = vec![0f64; n_s];
        for j in 0..n_s {indeksy[j] = rand::thread_rng().gen_range(0f64..1f64)*(scalars[i]-1f64)+1f64;}
        s.push(indeksy);
    }

    for i in 0..ntryes
    {
        if i <= 1000 {m = 0.9f64;}
        else {m = 0.2f64;}
        v = eamutkb::ea_mut(m,&s,t);
        u = eacrosskb::ea_cross(0.1f64,&s,&v);
        result = eacrkb::ea_cr(&s,&u,&time,&flux,&err);
        s = result.0; t = result.1; minchi = result.2;
        pb.inc(1);
    }
    pb.finish();
    println!("chi^2/(N-{}) = {}",params,minchi/( (time.len()-params) as f64 ) );

    for i in 0..time2.len()
    {
        nel = i as f64;
        time2[i] = time[0] + nel*diff;
        background[i] = time2[i]*a+b
    }

    match params
    {
        4 => {
            para = reparamkb::re_param(&vec![s[0][t],s[1][t],s[2][t],s[3][t]]);
            profile = op::one_profile(&time2,para[0],para[1],para[2],para[3]);
            profile1 = vec![]; profile2 = vec![];
            tit = format!("A = {}, B = {}, C = {}, D = {}",para[0],para[1],para[2],para[3]);
        },
        7 => {
            para = reparamkb::re_param(&vec![s[0][t],s[1][t],s[2][t],s[3][t],s[4][t],s[5][t],s[6][t]]);
            profile = tpob::two_profiles_one_b(&time2,para[0],para[1],para[2],para[3],para[4],para[5],para[6]);
            profile1 = op::one_profile(&time2,para[0],para[1],para[2],para[3]); leg1 = "A1, B, C1, D1";
            profile2 = op::one_profile(&time2,para[4],para[1],para[5],para[6]); leg2 = "A2, B, C2, D2";
            tit = format!("A1 = {}, B = {}, C1 = {}, D1 = {}\n\nA2 = {}, B = {}, C2 = {}, D2 = {}",para[0],para[1],para[2],para[3],para[4],para[1],para[5],para[6]);
        },
        8 => {
            para = reparamkb::re_param(&vec![s[0][t],s[1][t],s[2][t],s[3][t],s[4][t],s[5][t],s[6][t],s[7][t]]);
            profile = tptb::two_profiles_two_b(&time2,para[0],para[1],para[2],para[3],para[4],para[5],para[6],para[7]);
            profile1 = op::one_profile(&time2,para[0],para[1],para[2],para[3]); leg1 = "A1, B1, C1, D1";
            profile2 = op::one_profile(&time2,para[4],para[5],para[6],para[7]); leg2 = "A2, B2, C2, D2";
            tit = format!("A1 = {}, B1 = {}, C1 = {}, D1 = {}\n\nA2 = {}, B2 = {}, C2 = {}, D2 = {}",para[0],para[1],para[2],para[3],para[4],para[5],para[6],para[7]); 
        },
        _ => exit(666),
    }

    println!("{:?}",para);

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);

    if plotctrl
    {
        let (xlab, ylab): (String, String) = ("Time [minutes]".to_string(),"Normalized flux\n\n".to_string());
        let col: String = "#DD2B2B".to_string(); //"#6495ED"
        fplotd::fplotd(&time,&time2,&background,&profile,tit,xlab,ylab,col,&fluxbak,&fluxfull,&timefull,&profile1,&profile2,leg1.to_string(),leg2.to_string(),system);
    }
    
    if savectrl
    {
        let parameters;
        match params
        {
            4 => parameters = format!("{} {} {} {}\n{} {}",para[0],para[1],para[2],para[3],a,b),
            7 => parameters = format!("{} {} {} {}\n{} {} {} {}\n{} {}",para[0],para[1],para[2],para[3],para[4],para[1],para[5],para[6],a,b),
            8 => parameters = format!("{} {} {} {}\n{} {} {} {}\n{} {}",para[0],para[1],para[2],para[3],para[4],para[5],para[6],para[7],a,b),
            _ => exit(666),
        }
        fs::write("parametry_rust.txt", parameters).expect("Unable to write file");

    }    
}

fn main() 
{
    let mut v: Vec<&str>;
    let argc: usize = env::args().len();
    let mut file: &str = "rozblysk.dat";
    let argv: Vec<String> = env::args().collect();
    let mut n_iter: u64 = 10000; 
    let (mut params, mut n_pop): (usize, usize) = (7, 200);
    let (mut normback, mut noback): (bool, bool) = (false, false);
    let (mut plotctrl, mut savectrl): (bool, bool) = (false, false);

    if argc == 1 {helpf();}

    for i in 0..argc
    {
        if argv[i].contains("-lc=") {v = argv[i].split("=").collect(); file = v[1];}
        else if argv[i].contains("-nparams=") {v = argv[i].split("=").collect(); params = v[1].parse().unwrap();}
        else if argv[i].contains("-npop=") {v = argv[i].split("=").collect(); n_pop = v[1].parse().unwrap();}
        else if argv[i].contains("-niter=") {v = argv[i].split("=").collect(); n_iter = v[1].parse().unwrap();}
        else if argv[i].eq("--normback") {normback = true;}
        else if argv[i].eq("--noback") {noback = true;}
        else if argv[i].eq("--plot") {plotctrl = true;}
        else if argv[i].eq("--save") {savectrl = true;}
        else if argv[i].eq("--help") || argv[i].eq("-h")  {helpf();}
    }

    if normback && noback {println!("# Error! You have to choose only one type of background!"); exit(666);}
    if params != 4 && params != 7 && params != 8 {helpf();}
    ea_main(file,params,n_pop,plotctrl,savectrl,normback,noback,n_iter);
}