use gnuplot::{Figure, Caption, Color, Graph, AxesCommon, PointSymbol, PointSize, LineWidth, Fix,
              Font, LineStyle, DotDash, AutoOption::Auto, TickOption::{MajorScale, MinorScale}};

pub fn fplotd(x: &Vec<f64>, x2: &Vec<f64>, y: &Vec<f64>, y2: &Vec<f64>, tit: String,
              xlabel: String, ylabel: String, col: String, fluxbak: &Vec<f64>, fluxfull: &Vec<f64>,
              timefull: &Vec<f64>, profiles: &mut Vec<Vec<f64>>, nprofiles: &usize, system: &str)
{
    let mut lw: f64 = 6.0;
    let mut backmodel = vec![0f64; x2.len()];
    let colors: Vec<String> = vec![String::from("#1f77b4"), String::from("#7833DD"),
                                   String::from("#8c564b"), String::from("#e377c2"),
                                   String::from("#7f7f7f"), String::from("#bcbd22"),
                                   String::from("#17becf")];

    for i in 0..x2.len()
    {
        backmodel[i] = y2[i] + y[i];
        for j in 0..*nprofiles { profiles[j][i] += y[i]; }
    }

    let mut fig = Figure::new();
    if system.eq("linux") { _ = fig.set_pre_commands(
        &format!("set term x11 persist size 1000 150")); lw = 3.0;}
    else if system.eq("windows") { _ = fig.set_pre_commands(
        &format!("set term windows"));}

    let plt = fig.axes2d();
    plt.set_title(&tit, &[Font("Arial", 12.0)]);
    plt.set_x_label(&xlabel, &[Font("Arial", 12.0)]);
    plt.set_y_label(&ylabel, &[Font("Arial", 12.0)]);
    plt.set_legend(Graph(1.0), Graph(1.0), &[], &[Font("Arial", 12.0)]);
    plt.points(timefull,fluxfull,&[Color("#00000"),PointSymbol('O'),PointSize(1.0),
        Caption("Data points")]);
    plt.points(x,fluxbak,&[Color(&col),PointSymbol('O'),PointSize(1.0),
        Caption("Flare points")]);
    plt.lines(x2,y,&[Caption("Background"), Color("#ff8000"),LineWidth(lw)]);
    plt.lines(x2,backmodel,&[Caption("Flare model"), Color("#25B76E"),LineWidth(lw)]);
    for i in 0..*nprofiles
    {
        plt.lines(x2,&profiles[i],&[Caption(&format!["Profile{}",i+1]),
        Color(&colors[i.rem_euclid(colors.len())]),LineWidth(lw),LineStyle(DotDash)]);
    }
    plt.set_x_range(Fix(timefull[0]),Fix(timefull[timefull.len()-1]));
    plt.set_x_ticks(Some((Auto, 4)), &[MajorScale(1.5), MinorScale(0.75)],
                    &[Font("Arial", 12.0)]);
    plt.set_y_ticks(Some((Auto, 4)), &[MajorScale(1.5), MinorScale(0.75)],
                    &[Font("Arial", 12.0)]);

     _ = fig.show().unwrap();
}