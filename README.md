# Wrocław-Flare-Profile-Fitter
Software written in Rust for any operational system to fit Wrocław Flare Profile to any observational data using evolutionary algorithms.

To compile the code enter the downloaded folder where the src folder and Cargo.toml file are and type in terminal "cargo build --release"

    Usage: nprofileitter <-lc=file> [-nrpof=int] [-npop=int] [-niter=int] [--constb] [--normback]
                         [--noback] [--plot] [--save]

              option -lc        : light curve file name.
                     -nprof     : number of profiles (default nprof = 1).
                     -npop      : number of random drawings for each param. (deafult npop = 1000).
                     -niter     : number of iterations to do (default niter = 2500).
                     --constb   : make b parameter constant for all of the profiles.
                     --normback : set the keyword when the flare background is equal to 1.
                     --noback   : set the keyword when the flare background is equal to 0.
                     --plot     : plot the results.

