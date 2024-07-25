use gnuplot::{Figure, Caption, Color, Graph, AxesCommon, PointSymbol, PointSize, LineWidth, Fix, Font, LineStyle, Dash, DotDash, AutoOption::Auto, TickOption::{MajorScale, MinorScale}};

pub fn fplotd(x: &Vec<f64>, x2: &Vec<f64>, y: &Vec<f64>, y2: &Vec<f64>, tit: String, xlabel: String, ylabel: String, col: String, fluxbak: &Vec<f64>, fluxfull: &Vec<f64>, timefull: &Vec<f64>,profile1: &Vec<f64>,profile2: &Vec<f64>, leg1: String, leg2: String, system: &str)
{
    let mut backmodel = vec![0f64; x2.len()];
    let (mut profile1b, mut profile2b) = (vec![0f64; x2.len()],vec![0f64; x2.len()]);
    for i in 0..x2.len() 
    {
        backmodel[i] = y2[i] + y[i];
        if profile1.len() != 0 {profile1b[i] = profile1[i] + y[i]; profile2b[i] = profile2[i] + y[i];}
    }
    
    let mut fig = Figure::new(); //.points(x,fluxbak,&[Color(&col),PointSymbol('O'),PointSize(0.0),Caption("Flare points")]) ; "#6495ED"
    if system.eq("linux") { let _a = fig.set_pre_commands(&format!("set term x11 persist size 1000 150")); }
    if profile1.len() == 0
    {
        fig.axes2d().set_title(&tit, &[Font("Arial", 20.0)]).set_x_label(&xlabel, &[Font("Arial", 20.0)]).set_y_label(&ylabel, &[Font("Arial", 20.0)]).set_legend(Graph(1.0), Graph(1.0), &[], &[Font("Arial", 15.0)]).points(timefull,fluxfull,&[Color("#00000"),PointSymbol('O'),PointSize(1.0),Caption("Data points")]).points(x,fluxbak,&[Color(&col),PointSymbol('O'),PointSize(1.0),Caption("Flare points")]).lines(x2,y,&[Caption("Background"), Color("#ff8000"),LineWidth(3.0)]).lines(x2,backmodel,&[Caption("Flare model"), Color("#25B76E"),LineWidth(3.0)]).set_x_range(Fix(timefull[0]),Fix(timefull[timefull.len()-1])).set_x_ticks(Some((Auto, 4)), &[MajorScale(1.5), MinorScale(0.75)],&[Font("Arial", 20.0)]).set_y_ticks(Some((Auto, 4)), &[MajorScale(1.5), MinorScale(0.75)],&[Font("Arial", 20.0)]);
    }
    else
    {
        fig.axes2d().set_title(&tit, &[Font("Arial", 20.0)]).set_x_label(&xlabel, &[Font("Arial", 20.0)]).set_y_label(&ylabel, &[Font("Arial", 20.0)]).set_legend(Graph(1.0), Graph(1.0), &[], &[Font("Arial", 15.0)]).points(timefull,fluxfull,&[Color("#00000"),PointSymbol('O'),PointSize(1.0),Caption("Data points")]).points(x,fluxbak,&[Color(&col),PointSymbol('O'),PointSize(1.0),Caption("Flare points")]).lines(x2,y,&[Caption("Background"), Color("#ff8000"),LineWidth(3.0)]).lines(x2,backmodel,&[Caption("Flare model"), Color("#25B76E"),LineWidth(3.0)]).lines(x2,profile1b,&[Caption(&leg1), Color("#6495ED"),LineWidth(3.0),LineStyle(DotDash)]).lines(x2,profile2b,&[Caption(&leg2), Color("#b300b3"),LineWidth(3.0),LineStyle(Dash)]).set_x_range(Fix(timefull[0]),Fix(timefull[timefull.len()-1])).set_x_ticks(Some((Auto, 4)), &[MajorScale(1.5), MinorScale(0.75)],&[Font("Arial", 20.0)]).set_y_ticks(Some((Auto, 4)), &[MajorScale(1.5), MinorScale(0.75)],&[Font("Arial", 20.0)]);
    }
    let _a = fig.show().unwrap();
}