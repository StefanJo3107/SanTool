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
    #[arg(short, long)]
    source_path: String,
    #[arg(short, long)]
    output_path: Option<String>,
}

#[derive(Args)]
struct FlashArgs{
    #[arg(short, long)]
    source_path: Option<String>,
    #[arg(short, long)]
    bytecode_path: Option<String>,
    #[arg(short, long)]
    config_path: String
}