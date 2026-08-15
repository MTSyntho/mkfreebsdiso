use std::path::PathBuf;
use std::path;
use std::fs;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Streamline creating FreeBSD spinoff images from pre-compiled images.", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a mkfreebsdiso project
    Project {
        #[arg(short = 'm', long = "mode", default_value = "direct", help = "Specify how you wish to make modifications to the base system")]
        mode: String,

        // Setup - FreeBSD Version
        #[arg(short = 'v', long = "bsd-version", default_value = "0.0", value_name = "VERSION", required_unless_present_all = ["basefile", "kernelfile"], help = "Specify a FreeBSD version to download from servers")]
        bsdver: String,

        #[arg(short = 'b', long = "base-file", default_value = "base.txz", value_name = "FILE", conflicts_with_all = ["bsdver"], help = "Provide a FreeBSD base.txz file")]
        basefile: String,

        #[arg(short = 'k', long = "kernel-file", default_value = "kernel.txz", value_name = "FILE", conflicts_with_all = ["bsdver"], help = "Provide a FreeBSD kernel.txz file")]
        kernelfile: String,

        // Work Directory
        #[arg(short = 'p', long = "path", default_value = ".", help = "Specify working directory for the project")]
        workdir: String,
    },

    /// Compile the FreeBSD ISO image
    Build {
        #[arg(short = 't', long = "type", help = "Specify the format(s) in which you wish to build the final image")]
        formats: String,

        // Output Directoryy
        #[arg(short = 'o', long = "out", required = false, help = "Specfify output directory of the resulting ISO image")]
        out: String
    }
}



// struct FetchFreeBSD;

// impl FileDownloader for FetchFreeBSD {

//     fn download(&self) -> &str {
//         "https://planet.openstreetmap.org/pbf/planet-latest.osm.pbf"
//     }

// }

async fn fetch_freebsd_images(version: String, workdir: String) -> Result<(), Box<dyn std::error::Error>> {
    // let downloader = FetchFreeBSD;
    let mut url = format!("https://download.freebsd.org/releases/amd64/{}-RELEASE/base.txz", version);
    let resp = reqwest::get(url).await?;
    println!("{resp:#?}");
    // println!("{}", resp.status());

    if resp.status().is_success() {
        println!("Valid");
    } else {
        let mut url = format!("https://archive.freebsd.org/old-releases/amd64/{}-RELEASE/base.txz", version);
        let resp = reqwest::get(url).await?;
        println!("{resp:#?}");

        if resp.status().is_success() {
            println!("Valid");
        } else {
            let mut url = format!("https://download.freebsd.org/snapshots/amd64/{}-STABLE/base.txz", version);
            let resp = reqwest::get(url).await?;
            println!("{resp:#?}");

            if resp.status().is_success() {
                println!("Valid");
            } else {
                let mut url = format!("https://download.freebsd.org/snapshots/amd64/{}-CURRENT/base.txz", version);
                let resp = reqwest::get(url).await?;
                println!("{resp:#?}");     
                
                if resp.status().is_success() {
                    println!("Valid");
                } else {
                    for i in 1..=10 {
                        url = format!("https://download.freebsd.org/releases/amd64/{}-BETA{}/base.txz", version, i);

                        let resp = reqwest::get(url).await?;
                        println!("{resp:#?}");

                        if resp.status().is_success() {
                            println!("Valid");
                            return Ok(());
                        } else {
                            println!("Invalid!");
                        }
                    }
                }
            } 
        }
    }
    
    // let path = downloader.find_file_locally_or_download_into("./images").await?;
    Ok(())
}

fn init_project(directory: String) {
    std::fs::create_dir("mkfreebsdiso-project");

    
}

#[tokio::main]
async fn main()  {
    let args = Args::parse();

    // println!("Hello {} {}!", Commands::Setup.workdir, Commands::Setup.bsdver);

    match args.command {
        Commands::Project { mode, bsdver, basefile, kernelfile, workdir } => {
            // println!("Hello {} {}!", workdir, bsdver);
            if bsdver != "0.0" {
                fetch_freebsd_images(bsdver, workdir).await.unwrap();
            } else {
                if std::fs::read_dir(workdir)?.next().is_none() {
                    init_project(workdir)
                } 
            }
        }

        Commands::Build { formats, out } => {
            println!("{}", out);
        }
    }
}