mod args;

use args::*;
use hash40::{read_labels, set_labels};
use serde_yaml::{from_str, to_string};
use sqb;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();

    if let Some(ref label_path) = args.label {
        match read_labels(label_path) {
            Ok(labels) => set_labels(labels),
            Err(e) => {
                println!("Error loading labels: {}", e);
                return
            },
        }
    }

    if let Err(e) = match args.mode {
        Mode::Disasm { file } => {
            convert_to_yaml(&file, &args.out.as_ref().map_or("out.yml", String::as_str))
        }
        Mode::Asm { file } => {
            convert_to_bin(&file, &args.out.as_ref().map_or("out.sqb", String::as_str))
        }
    } {
        println!("{}", e);
    }
}

fn convert_to_yaml(in_path: &str, out_path: &str) -> Result<(), Box<dyn Error>> {
    let sqb = sqb::open(in_path)?;
    let mut f = File::create(out_path)?;
    let pretty = to_string(&sqb)?;
    f.write_all(pretty.as_bytes())?;
    Ok(())
}

fn convert_to_bin(in_path: &str, out_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(in_path)?;
    let mut contents: String = String::default();
    file.read_to_string(&mut contents)?;

    let mlist = from_str(&contents)?;
    sqb::save(out_path, &mlist)?;
    Ok(())
}
