use std::{fs, io};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Stdio;
use clap::{Command, Parser};
use crate::{Cli, Commands, CompileArgs, ConfigArgs, FlashArgs};

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
            flash(args);
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

pub fn flash(config: &FlashArgs) {
    if !Path::new("./config.toml").exists() {
        panic!("config.toml doesn't exist! Run santool config --help for more info!");
    }

    let config_file = fs::read_to_string("./config.toml").expect("Unable to read file config.toml!");
    let config_toml: ConfigArgs = toml::from_str(config_file.as_str()).expect("Unable to deserialize file config.toml!");

    if let None = config_toml.compiler_path {
        panic!("SanUSB path is not defined in config.toml! Run santool config --help for more info!");
    }
    let mut sanusb_path = config_toml.sanusb_path.clone().unwrap();
    let config_path = config.config_path.clone();
    std::process::Command::new("cp")
        .args([config_path, sanusb_path])
        .output()
        .expect("failed to execute process");

    sanusb_path = config_toml.sanusb_path.clone().unwrap();
    if let Some(bytecode) = &config.bytecode_path {
        std::process::Command::new("cp")
            .args([bytecode, &(sanusb_path + "/payload.sanb")])
            .output()
            .expect("failed to execute process");
    } else if let Some(source) = &config.source_path {
        if let None = config_toml.compiler_path {
            panic!("Compiler path is not defined in config.toml! Run santool config --help for more info!");
        }

        let output = std::process::Command::new(config_toml.compiler_path.unwrap())
            .args([source, &(sanusb_path + "/payload.sanb")])
            .output()
            .expect("failed to execute process");
        io::stdout().write_all(&output.stdout).unwrap();
    } else {
        panic!("Neither souce path nor bytecode path are defined! Run santool flash --help for more info!")
    }

    sanusb_path = config_toml.sanusb_path.clone().unwrap();
    let mut cmd = std::process::Command::new("sh")
        .args(["-c", "cargo run"])
        .current_dir(sanusb_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let output = BufReader::new(cmd.stdout.take().unwrap());

    output.lines().for_each(|line| {
        println!("output: {}", line.unwrap());
    })
}