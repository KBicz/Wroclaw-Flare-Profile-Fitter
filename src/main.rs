mod op;
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
    println!("\n Program nprofileitter for Windows, Linux, and Mac, written by K. Bicz, version of 06 dec 2024.");
    println!(" Usage: nprofileitter <-lc=file> [-nrpof=int] [-npop=int] [-niter=int] [--constb] \
    [--normback]");
    println!("                                 [--noback] [--plot] [--save]\n");
    println!("              option -lc        : light curve file name.");
    println!("                     -nprof     : number of profiles (default nprof = 1).");
    println!("                     -npop      : number of random drawings for each param. \
    (deafult npop = 1000).");
    println!("                     -niter     : number of iterations to do (default niter = \
    2500).");
    println!("                     --constb   : make b parameter constant for all of the \
    profiles.");
    println!("                     --normback : set the keyword when the flare background is equal \
    to 1.");
    println!("                     --noback   : set the keyword when the flare background is equal \
    to 0.");
    println!("                     --plot     : plot the results.\n");
    exit(0);
}

fn ea_main(file: &str, nprofiles: usize, npop: usize, plotctrl: bool, savectrl: bool,
           normback: bool, noback: bool, ntryes: u64, constb: bool)
{
    let start: Instant = Instant::now();
    let nprofchi: usize = {
        if constb
        { nprofiles*4 - (nprofiles - 1) }
        else
        { nprofiles*4 }
    };
    let mut t: usize = 10;
    let mut printstr: String;
    let nss: f64 = npop as f64;
    let mut minchi: f64 = 0f64;
    let mut s: Vec<Vec<Vec<f64>>> = vec![];
    let mut vecp: Vec<Vec<f64>> = Vec::new();
    let mut time2: Vec<f64> = vec![0f64;1000];
    let mut background: Vec<f64> = vec![0f64;1000];
    let pb: ProgressBar = ProgressBar::new(ntryes);
    let (mut para, mut profile): (Vec<Vec<f64>>,Vec<f64>);
    let scalars: Vec<f64> = vec![12f64,6f64,6f64,8f64,12f64,6f64,6f64,8f64];
    let (mut m, mut v, mut u, mut result, mut tit): (f64, Vec<Vec<Vec<f64>>>, Vec<Vec<Vec<f64>>>,
                                                 (Vec<Vec<Vec<f64>>>, usize,f64), String);
    let (time,flux,err,a,b,_,_,fluxbak,timefull,fluxfull): (Vec<f64>, Vec<f64>, Vec<f64>,
                                                                       f64, f64, usize, usize,
                                                                       Vec<f64>, Vec<f64>, Vec<f64>)
        = readdkb::read_data(file,normback,noback);
    let nel: f64 = time2.len() as f64;
    let mut diff: f64 = time[time.len()-1]-time[0];
    let system: &str = env::consts::OS;
    diff /= nel;
    pb.set_style(ProgressStyle::with_template(&format!("{} {} {{spinner:.green}} \
    [{{elapsed_precise}}] ╢{{bar:35.white/gray}}╟ {{percent}}% [{{eta_precise}}, {{per_sec}}]",
                                                       Style::new().bold().paint("Fitting").
                                                           to_string(),Style::new().bold().
            paint("profile").to_string())).unwrap());

    if npop <= 10 
    {
        let temp_t = (nss-1f64)/2f64;
        t = temp_t.round() as usize;
    }

    for k in 0..nprofiles
    {
        s.push(vec![]);
        for i in 0..4
        {
            let mut indeksy = vec![0f64; npop];
            for j in 0..npop { indeksy[j] = rand::thread_rng().gen_range(0f64..1f64) *
                (scalars[i] - 1f64) + 1f64; }
            s[k].push(indeksy);
        }

        for j in 0..npop
        {
            while 10f64.powf(s[k][2][j]-3.0)*10f64.powf(s[k][3][j]-5.0) >= 8.0
            {
                s[k][2][j] = rand::thread_rng().gen_range(0f64..1f64) * (scalars[2] - 1f64)
                    + 1f64;
                s[k][3][j] = rand::thread_rng().gen_range(0f64..1f64) * (scalars[3] - 1f64)
                    + 1f64;
            }
        }
    }
    
    if constb
    {
        for j in 1..nprofiles
        { s[j][1] = s[0][1].clone(); }
    }

    for i in 0..ntryes
    {
        if i <= 1000 {m = 0.9f64;}
        else {m = 0.2f64;}
        v = eamutkb::ea_mut(m,&s,t,&nprofiles,&npop);
        if constb
        { for j in 1..nprofiles
          { v[j][1] = v[0][1].clone(); } }
        u = eacrosskb::ea_cross(0.1f64,&s,&v,&nprofiles,&npop);
        if constb
        { for j in 1..nprofiles
          { u[j][1] = u[0][1].clone(); } }
        result = eacrkb::ea_cr(&s,&u,&time,&flux,&err,nprofiles,npop);
        s = result.0; t = result.1; minchi = result.2;
        pb.inc(1);
    }
    pb.finish();
    println!("chi^2/(N-{}) = {}",nprofchi,minchi/( (time.len()-nprofchi) as f64));

    for i in 0..time2.len()
    {
        time2[i] = time[0] + (i as f64)*diff;
        background[i] = time2[i]*a+b
    }

    tit = "".to_owned(); printstr = "".to_owned();
    para = Vec::new(); profile = vec![0f64; time2.len()];
    for k in 0..nprofiles
    {
        para.push(reparamkb::re_param(&vec![s[k][0][t],s[k][1][t],s[k][2][t],s[k][3][t]]));
        vecp.push(op::one_profile(&time2,para[k][0],para[k][1],para[k][2],
                                  para[k][3]));
        tit.push_str(&format!("A{}={:.10} B{}={:.10} C{}={:.10} D{}={:.10}\n\n",k+1,
                              para[k][0],k+1,para[k][1],k+1,para[k][2],k+1,para[k][3]));
        printstr.push_str(&format!("A{}={:.10} B{}={:.10} C{}={:.10} D{}={:.10}\n",k+1,
                                   para[k][0],k+1,para[k][1],k+1,para[k][2],k+1,para[k][3]));
        for i in 0..vecp[k].len()
        {profile[i] += vecp[k][i];}
    }
    printstr.pop(); tit.pop(); tit.pop();
    println!("{}",printstr);
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);

    if plotctrl
    {
        let (xlab, ylab): (String, String) = ("Time [minutes]".to_string(),"Normalized flux\n\n".
            to_string());
        let col: String = "#DD2B2B".to_string(); //"#6495ED"
         fplotd::fplotd(&time,&time2,&background,&profile,tit,xlab,ylab,col,&fluxbak,&fluxfull,
                        &timefull, &mut vecp, &nprofiles, system);
    }

    if savectrl
    {
        let mut parameters: String = "".to_owned();
        for k in 0..nprofiles
        { parameters.push_str(&format!("{} {} {} {}\n",para[k][0],para[k][1],para[k][2],
                para[k][3])); }
        parameters.push_str(&format!("{} {}",a,b));
        fs::write("parametry_rust.txt", parameters).expect("Unable to write file");
    }
}

fn main() 
{
    let mut n_iter: u64 = 2500;
    let mut constb: bool = false;
    let argc: usize = env::args().len();
    let mut file: &str = "rozblysk.dat";
    let argv: Vec<String> = env::args().collect();
    let (mut nprof, mut n_pop): (usize, usize) = (1, 1000);
    let (mut normback, mut noback): (bool, bool) = (false, false);
    let (mut plotctrl, mut savectrl): (bool, bool) = (false, false);

    if argc == 1 {helpf();}

    for arg in argv.iter() {
        match arg.split_once('=') {
            Some(("-lc", value)) => file = value,
            Some(("-nprof", value)) => nprof = value.parse().unwrap(),
            Some(("-npop", value)) => n_pop = value.parse().unwrap(),
            Some(("-niter", value)) => n_iter = value.parse().unwrap(),
            _ => match arg.as_str() {
                "--normback" => normback = true,
                "--noback" => noback = true,
                "--plot" => plotctrl = true,
                "--save" => savectrl = true,
                "--constb" => constb = true,
                "--help" | "-h" => helpf(),
                _ => {}
            },
        }
    }

    if normback && noback {println!("# Error! You have to choose only one type of background!");
        exit(666);}
    ea_main(file,nprof,n_pop,plotctrl,savectrl,normback,noback,n_iter,constb);
}