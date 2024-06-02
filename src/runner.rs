use std::{fs, io};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use clap::{Command, Parser};
use crate::{Cli, Commands, CompileArgs, ConfigArgs};

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Config(args) => {
            parse_config(args);
        }
        Commands::Compile(args) => {
            compile(args);
        }
        Commands::Flash(args) => {
            println!("'santool flash' was used, values are: {:?}", args.config_path)
        }
    }
}

pub fn parse_config(config: &ConfigArgs) {
    let mut final_config = ConfigArgs { compiler_path: None, vm_path: None, sanusb_path: None, infra_path: None };
    if Path::new("./config.toml").exists() {
        let config_file = fs::read_to_string("./config.toml").expect("Unable to read file config.toml!");
        final_config = toml::from_str(config_file.as_str()).expect("Unable to deserialize file config.toml!");
    }

    if let Some(compiler_path) = &config.compiler_path {
        final_config.compiler_path = Some(compiler_path.clone());
    }

    if let Some(vm_path) = &config.vm_path {
        final_config.vm_path = Some(vm_path.clone());
    }

    if let Some(sanusb_path) = &config.sanusb_path {
        final_config.sanusb_path = Some(sanusb_path.clone());
    }

    if let Some(infra_path) = &config.infra_path {
        final_config.infra_path = Some(infra_path.clone());
    }

    fs::write("./config.toml", toml::to_string(&final_config).expect("Unable to serialize config to toml!")).expect("Unable to write file");
}

pub fn compile(config: &CompileArgs) {
    if !Path::new("./config.toml").exists() {
        panic!("config.toml doesn't exist! Run santool config --help for more info!");
    }

    let config_file = fs::read_to_string("./config.toml").expect("Unable to read file config.toml!");
    let config_toml: ConfigArgs = toml::from_str(config_file.as_str()).expect("Unable to deserialize file config.toml!");

    if let None = config_toml.compiler_path {
        panic!("Compiler path is not defined in config.toml! Run santool config --help for more info!");
    }

    if !Path::new(&config.source_path).exists() {
        panic!("Source path does not exist!");
    }

    if !(Path::new(&config.source_path).extension().expect("Source file missing extension!") == "san") {
        panic!("Source file should have .san extension!");
    }

    let mut output_path = config.source_path.clone() + "b";
    if let Some(output) = config.output_path.clone() {
        output_path = output;
    }

    let output = std::process::Command::new(config_toml.compiler_path.unwrap())
        .args([config.source_path.clone(), output_path])
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
}