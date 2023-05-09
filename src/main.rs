#[macro_use]
mod macros;
mod cli;
mod program_execution;

use std::{env, path::PathBuf};

use program_execution::exec_shell_cmd;

use cli::{filter_options, usage};

use crate::cli::match_command;

#[derive(Debug)]
enum Errors {
    CouldNotGetPath,
    DockerNotFound,
    CommandExecutionFailed,
    STDINError,
    // IOError,
}

fn main() -> Result<(), Errors> {
    let _path = env::current_dir().or(Err(Errors::CouldNotGetPath))?;
    let docker = find_docker().or(Err(Errors::DockerNotFound))?;
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // can never be zero due to the fact that the first element is always the program itself
        usage();
        return Ok(());
    }

    // when filter_options returns none, the program should exit
    let commands_index = match filter_options(&args, &docker) {
        Some(i) => i,
        None => return Ok(()),
    };

    match_command(&args, commands_index, &docker).unwrap();

    Ok(())
}

fn find_docker() -> Result<PathBuf, Errors> {
    let output = exec_shell_cmd(str!("which docker")).or(Err(Errors::CommandExecutionFailed))?;

    let mut path = PathBuf::new();
    let path_string = String::from_utf8(output.stdout).or(Err(Errors::STDINError))?;

    path.push(path_string.trim());
    Ok(path)
}
