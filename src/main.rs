use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use competency_library::datasets;

#[derive(Parser)]
#[command(name = "competency-library", about = "Competency taxonomy explorer")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Download raw dataset files
    Download {
        #[command(subcommand)]
        source: DownloadSource,
    },
    /// Process raw data into normalized JSON
    Process {
        #[command(subcommand)]
        source: ProcessSource,
    },
}

#[derive(Subcommand)]
enum DownloadSource {
    /// Download O*NET 30.2 database
    Onet,
    /// Download OPM MOSAIC competencies
    Opm,
    /// Download WORKBank tasks from HuggingFace/GitHub
    Workbank,
    /// Fetch Lightcast skills via API (requires .env credentials)
    Lightcast,
}

#[derive(Subcommand)]
enum ProcessSource {
    /// Process O*NET text files into JSON
    Onet,
    /// Process OPM Excel into JSON
    Opm,
    /// Process WORKBank CSV into JSON
    Workbank,
    /// Process cached Lightcast response into JSON
    Lightcast,
    /// Process all datasets
    All,
}

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let dir = data_dir();

    match cli.command {
        Command::Download { source } => match source {
            DownloadSource::Onet => {
                println!("Downloading O*NET...");
                datasets::onet::download(&dir).await?;
            }
            DownloadSource::Opm => {
                println!("Downloading OPM MOSAIC...");
                datasets::opm::download(&dir).await?;
            }
            DownloadSource::Workbank => {
                println!("Downloading WORKBank...");
                datasets::workbank::download(&dir).await?;
            }
            DownloadSource::Lightcast => {
                println!("Downloading Lightcast skills...");
                datasets::lightcast::download(&dir).await?;
            }
        },
        Command::Process { source } => match source {
            ProcessSource::Onet => {
                println!("Processing O*NET...");
                datasets::onet::process(&dir)?;
            }
            ProcessSource::Opm => {
                println!("Processing OPM...");
                datasets::opm::process(&dir)?;
            }
            ProcessSource::Workbank => {
                println!("Processing WORKBank...");
                datasets::workbank::process(&dir)?;
            }
            ProcessSource::Lightcast => {
                println!("Processing Lightcast...");
                datasets::lightcast::process(&dir)?;
            }
            ProcessSource::All => {
                println!("Processing all datasets...");
                if let Err(e) = datasets::onet::process(&dir) {
                    eprintln!("O*NET processing failed: {e:#}");
                }
                if let Err(e) = datasets::opm::process(&dir) {
                    eprintln!("OPM processing failed: {e:#}");
                }
                if let Err(e) = datasets::workbank::process(&dir) {
                    eprintln!("WORKBank processing failed: {e:#}");
                }
                if let Err(e) = datasets::lightcast::process(&dir) {
                    eprintln!("Lightcast processing failed: {e:#}");
                }
                println!("Done.");
            }
        },
    }

    Ok(())
}
