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

#[tokio::main]
async fn main()  {
    let args = Args::parse();

    // println!("Hello {} {}!", Commands::Setup.workdir, Commands::Setup.bsdver);

    match args.command {
        Commands::Setup { bsdver, workdir } => {
            // println!("Hello {} {}!", workdir, bsdver);
            fetch_freebsd_images(bsdver, workdir).await.unwrap();
        }

        Commands::Build { out } => {
            println!("{}", out);
        }
    }
}