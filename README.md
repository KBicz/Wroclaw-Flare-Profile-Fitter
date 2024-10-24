# Wrocław-Flare-Profile-Fitter
Software written in Rust for any operational system to fit Wrocław Flare Profile to any observational data using evolutionary algorithms.

To compile the code enter the downloaded folder where the src folder and Cargo.toml file are and type in terminal "cargo build --release"

Usage: wroclaw_profile_ea <-lc=file> [-nparams=int] [-npop=int] [-niter=int] [--normback] [--noback] [--plot] [--save]
  - -lc - input file. The input file should contain three columns: the first column representing time in days, and the second column showing the normalized flux, expressed as parts per thousand (ppt), with values dispersed around 1 minus one, and third column representing the uncertainty of the flux from the second column.
  - -nparams - number of parameters of Wrocław Flare Profile - possible values are 4, 7, or 8 ([Bicz et al. 2022](https://iopscience.iop.org/article/10.3847/1538-4357/ac7ab3)) 
  - -niter   - number of iterations used to fit the profile to data (default value is 10000)
  - -popul   - number of specimens for each of the fitted parameters (default value is 200)
  - --normback - assume that the background signal of the quiet star is already normalized and do not try to normalize it with the code
  - --noback - assume no background signal from the quiet star (it is already at level 0)
  - --plot - display the results - to do it you need to install gnuplot
  - --save - save the results of the fit
