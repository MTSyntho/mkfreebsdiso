use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Streamline creating FreeBSD spinoff images from pre-compiled images.", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup a mkfreebsdiso project
    Setup {
        // Setup - FreeBSD Version
        #[arg(short = 'b', long = "bsd_version", value_name = "VERSION", help = "Specify a FreeBSD version to download from servers")]
        bsdver: String,

        // Work Directory
        #[arg(short = 'd', long = "directory", help = "Specify working directory for the project")]
        workdir: String,
    },

    /// Compile the FreeBSD ISO image
    Build {
        // Output Directoryy
        #[arg(short = 'o', long = "out", help = "Specfify output directory of the resulting ISO image")]
        out: String
    }
}
fn main() {
    let args = Args::parse();

    // println!("Hello {} {}!", Commands::Setup.workdir, Commands::Setup.bsdver);

    match args.command {
        Commands::Setup { bsdver, workdir } => {
            println!("Hello {} {}!", workdir, bsdver);
        }

        Commands::Build { out } => {
            println!("{}", out);
        }
    }


}