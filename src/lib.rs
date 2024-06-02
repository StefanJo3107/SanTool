pub mod runner;

use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Config(ConfigArgs),
    Compile(CompileArgs),
    Flash(FlashArgs)
}

#[derive(Args, Serialize, Deserialize)]
struct ConfigArgs{
    #[arg(short, long)]
    compiler_path: Option<String>,
    #[arg(short, long)]
    vm_path: Option<String>,
    #[arg(short, long)]
    sanusb_path: Option<String>,
    #[arg(short, long)]
    infra_path: Option<String>
}

#[derive(Args)]
struct CompileArgs{
    source_path: String,
    output_path: Option<String>,
}

#[derive(Args)]
struct FlashArgs{
    source_path: String,
    config_path: String
}