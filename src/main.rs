// use std::path::PathBuf;
// use std::path;
// use std::fs;
use std::*;
use clap::{Parser, Subcommand};
use reqwest::get;
use std::io::Write;


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

    /// Compile the FreeBSD ISO image ( Root required! )
    Build {
        #[arg(short = 't', long = "type", default_value = "iso", help = "Specify the format(s) in which you wish to build the final image")]
        formats: String,

        // Output Directoryy
        #[arg(short = 'o', long = "out", default_value = ".", help = "Specfify output directory of the resulting ISO image ( Split using commas, no spaces in-between )")]
        out: String
    }
}



// struct FetchFreeBSD;

// impl FileDownloader for FetchFreeBSD {

//     fn download(&self) -> &str {
//         "https://planet.openstreetmap.org/pbf/planet-latest.osm.pbf"
//     }

// }

async fn fetch_freebsd_images(version: &String, workdir: &String) -> Result<String, Box<dyn std::error::Error>> {
    // let downloader = FetchFreeBSD;
    let mut baseurl = format!("https://download.freebsd.org/releases/amd64/{}-RELEASE/base.txz", version);
    
    let resp = reqwest::get(&baseurl).await?;
    println!("{resp:#?}");
    // println!("{}", resp.status());

    // bruh WHO let me write this staircase of doom ahh if/else statement :headstone:^TREE(3)
    if resp.status().is_success() {
        println!("Valid");
    } else {
        baseurl = format!("https://archive.freebsd.org/old-releases/amd64/{}-RELEASE/base.txz", version);
        let resp = reqwest::get(&baseurl).await?;
        println!("{resp:#?}");

        if resp.status().is_success() {
            println!("Valid");
        } else {
            baseurl = format!("https://download.freebsd.org/snapshots/amd64/{}-STABLE/base.txz", version);
            let resp = reqwest::get(&baseurl).await?;
            println!("{resp:#?}");

            if resp.status().is_success() {
                println!("Valid");
            } else {
                baseurl = format!("https://download.freebsd.org/snapshots/amd64/{}-CURRENT/base.txz", version);
                let resp = reqwest::get(&baseurl).await?;
                println!("{resp:#?}");     
                
                if resp.status().is_success() {
                    println!("Valid");
                } else {
                    for i in 1..=10 {
                        baseurl = format!("https://download.freebsd.org/releases/amd64/{}-BETA{}/base.txz", version, i);

                        let resp = reqwest::get(&baseurl).await?;
                        println!("{resp:#?}");

                        if resp.status().is_success() {
                            println!("Valid");
                        } else {
                            println!("Invalid!");
                        }
                    }
                }
            } 
        }
    }

    // let response = reqwest::get(&baseurl).await?;
    // let mut basefile = std::fs::File::create("base.txz")?;

    // let mut stream = response.bytes_stream();

    // while let Some(chunk_result) = stream.next().await {
    //     let chunk = chunk_result?;
    //     basefile.write_all(&chunk)?;
    // }

    println!("\n[Project Initialisation] Downloading FreeBSD {} via `curl` - base.txz", version);
    println!("{}", &baseurl);

    std::process::Command::new("curl")
        .args(["-#", "-L", "-O", &baseurl])
        .status()?;

    let kernelurl = baseurl.replace("base.txz", "kernel.txz"); // my logic is just strange bro...

    println!("\n[Project Initialisation] Downloading FreeBSD {} via `curl` - kernel.txz", version);
    println!("{}", &kernelurl);
    

    std::process::Command::new("curl")
        .args(["-#", "-L", "-O", &kernelurl])
        .status()?;

    println!("\n");
    

    // let response = reqwest::get(&kernelurl).await?;
    // let mut kernelfile = std::fs::File::create("kernel.txz")?;

    // let mut stream = response.bytes_stream();

    // while let Some(chunk_result) = stream.next().await {
    //     let chunk = chunk_result?;
    //     kernelfile.write_all(&chunk)?;
    // }    

    Ok("".to_string())
}

fn init_project(directory: String, basepath: String, kernelpath: String, copy: bool) {
    std::fs::create_dir("mkfreebsdiso-project").unwrap();
    std::fs::create_dir("mkfreebsdiso-project/overlayfs").unwrap();
    println!("[Project Initialisation] Created project directory as `mkfreebsdiso-project`");

    let basefile = std::path::Path::new(&basepath); 
    let kernelfile = std::path::Path::new(&kernelpath); 

    if copy == true {
        if basefile.exists() && kernelfile.exists() {
            std::fs::copy(basefile, "mkfreebsdiso-project/base.txz");
            println!("[Project Initialisation] Copied {} as `base.txz` into project.", basepath);

            std::fs::copy(kernelfile, "mkfreebsdiso-project/kernel.txz");
            println!("[Project Initialisation] Copied {} as `kernel.txz` into project.", kernelpath);
        } else {
            println!("[Project Initialisation] ERROR! Your base.txz ( {} ) and/or kernel.txz ( {} ) is missing! Perhaps try downloading them?.", basepath, kernelpath);
        }
    } else {
        if basefile.exists() && kernelfile.exists() {
            std::fs::rename(basefile, "mkfreebsdiso-project/base.txz");
            println!("[Project Initialisation] Moved {} as `base.txz` into project.", basepath);

            std::fs::rename(kernelfile, "mkfreebsdiso-project/kernel.txz");
            println!("[Project Initialisation] Moved {} as `kernel.txz` into project.", kernelpath);
        } else {
            println!("[Project Initialisation] ERROR! Your base.txz ( {} ) and/or kernel.txz ( {} ) is missing! Perhaps try downloading them?.", basepath, kernelpath);
        }
    }

    

    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let args = Args::parse();

    // println!("Hello {} {}!", Commands::Setup.workdir, Commands::Setup.bsdver);

    match args.command {
        Commands::Project { mode, bsdver, basefile, kernelfile, workdir } => {
            // println!("Hello {} {}!", workdir, bsdver);
            if bsdver != "0.0" {
                fetch_freebsd_images(&bsdver, &workdir).await.unwrap();
                init_project(workdir, basefile, kernelfile, false);
            } else {
                init_project(workdir, basefile, kernelfile, true);

                // if std::fs::read_dir(&workdir)?.next().is_none() {
                // } else {
                // }
            }
        }

        Commands::Build { formats, out } => {
            let selected_formats: Vec<&str> = formats.split(',').collect();

            if !nix::unistd::Uid::effective().is_root() {
                eprintln!("This part of the program must be run as root.");
                std::process::exit(1);
            }

            println!("[Image Build] NOTE: Any selected formats will be ignored! An *.iso file will be generated instead as that is all that's implemented as of now. Apologies for the inconvenience...");
            
            let home = std::env::var("HOME")?;
            let cache_folder = std::path::PathBuf::from(&home).join(".cache");
            let genfscache_folder = std::path::PathBuf::from(&home).join(".cache/mkfreebsdiso_genfs");

            if !std::path::Path::new(&cache_folder).exists() {
                std::fs::create_dir(&cache_folder).unwrap();
            }

            if std::path::Path::new(&genfscache_folder).exists() {
                std::fs::remove_dir_all(&genfscache_folder)?;
                println!("[Image Build] Purged old build files");

            }

            std::fs::create_dir(&genfscache_folder).unwrap();

            println!("[Image Build] Extracting base contents...");
            let basefile = std::fs::File::open("base.txz")?;
            let kernelfile = std::fs::File::open("kernel.txz")?;

            let decompressor = xz2::read::XzDecoder::new(basefile);
            let mut archive = tar::Archive::new(decompressor);
            archive.set_preserve_ownerships(true);
            archive.set_preserve_permissions(true);
            archive.set_unpack_xattrs(true);
            
            archive.unpack(&genfscache_folder)?;

            println!("[Image Build] Extracting kernel contents...");
            let decompressor = xz2::read::XzDecoder::new(kernelfile);
            let mut archive = tar::Archive::new(decompressor);
            archive.set_preserve_ownerships(true);
            archive.set_preserve_permissions(true);
            archive.set_unpack_xattrs(true);

            archive.unpack(&genfscache_folder)?;

            println!("[Image Build] Overlaying filesystem contents...");
            dircpy::CopyBuilder::new("overlayfs", &genfscache_folder)
                .overwrite(true)
                .run()
                .unwrap();

            if !std::path::Path::new(&genfscache_folder.join("boot/cdboot")).exists() {
                println!("[Image Build] `/boot/cdboot` is missing from the base system. Make sure cdboot is present!");
                return Ok(());
            } 

            let mut fstab = std::fs::File::create_new(genfscache_folder.join("etc/fstab"))?;
            fstab.write_all("/dev/iso9660/MKFREEBSDISO / cd9660 ro 0 0".as_bytes())?;
            println!("[Image Build] Generated fstab");

            println!("[Image Build] Generating final image...");
            std::process::Command::new("xorriso")
                .args(["-as", "mkisofs", "-o", "mkfreebsdiso_output.iso", "-V", "MKFREEBSDISO", "-allow-lowercase", "-r", "-b", "boot/cdboot", "-no-emul-boot", genfscache_folder.to_str().unwrap()])
                .status()?;

            println!("[Image Build] Build complete");
            

        }
    }

    Ok(())
}