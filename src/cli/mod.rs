mod install;
mod options;

use std::path::PathBuf;

use crate::program_execution::{exec_cmd, string_from_uft8};
use crate::Errors;

use self::options::get_options;

fn get_version(docker: &PathBuf) -> String {
    let default = str!("failed to get version from Docker daemon");
    let docker_version = match exec_cmd(docker, vec![str!("--version")]) {
        Ok(output) => String::from_utf8(output.stdout).unwrap_or(default),
        Err(_) => default,
    };
    format!(
        "dpm: {}\ndocker: {}",
        env!("CARGO_PKG_VERSION"),
        docker_version
    )
}

/// returns None when the program should exit
pub fn filter_options(args: &Vec<String>, docker: &PathBuf) -> Option<usize> {
    let options = get_options(args, 1);

    if options.contains(&"-v") || options.contains(&"--version") {
        print!("{}", get_version(docker));
        return None;
    }

    if options.contains(&"-h") || options.contains(&"--help") {
        print!("{}", usage());
        return None;
    }

    Some(options.len() + 1)
}

pub fn match_command(args: &Vec<String>, cmd_index: usize, docker: &PathBuf) -> Result<(), Errors> {
    let cmd = match args.get(cmd_index) {
        Some(i) => i,
        None => {
            print!("{}", usage());
            return Ok(());
        }
    };

    match cmd.as_str() {
        "install" => install::run(args, cmd_index + 1, docker),
        "hello" => run_hello(docker),
        _ => command_not_found(cmd),
    }
}

fn print_output(output: Vec<u8>) -> Result<(), Errors> {
    println!("{}", string_from_uft8(output)?);
    Ok(())
}

fn command_not_found(cmd: &String) -> Result<(), Errors> {
    println!(
        "command '{}' not found. Use 
    dpm --help 
to check usage",
        cmd
    );
    Ok(())
}

fn run_hello(docker: &PathBuf) -> Result<(), Errors> {
    let docker_output = exec_cmd(&docker, vec![str!("run"), str!("hello-world")])?;
    print_output(docker_output.stdout)?;
    Ok(())
}

pub fn usage() -> String {
    str!(
        "Usage: dpm [OPTIONS] COMMAND [ARGS...]

Docker Package Manager - Manage locally installed programs with Docker containers.

Options:
  -h, --help     Show this help message and exit.
  -v, --version  Show the version number and exit.

Commands:
  install       Install a program as a Docker container.
  list          List all installed programs.
  remove        Remove a program and its associated Docker container.
  update        Update a program's Docker container to the latest version.

all available programs are also considered valid commands
"
    )
}
