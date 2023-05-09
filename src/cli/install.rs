use std::{error::Error, path::PathBuf};

use crate::program_execution::exec_cmd;

pub fn run(args: &Vec<String>, cmd_index: usize, docker: &PathBuf) -> Result<(), Box<dyn Error>> {
    if args.len() == cmd_index {
        usage();
        return Ok(());
    }
    let pull = exec_cmd(docker, vec![str!("pull")])?;
    print!("{}", String::from_utf8(pull.stderr).unwrap());
    Ok(())
}

fn usage() {
    println!(
        "dpm install [options] program
        
OPTIONS:
    -t, --tag: specify image tag default=latest"
    )
}