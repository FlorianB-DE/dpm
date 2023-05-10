#[macro_use]
mod macros;
mod cli;
mod program_execution;

use std::{env, path::PathBuf};

use program_execution::{exec_shell_cmd, string_from_uft8};

use cli::{filter_options, usage};

use crate::cli::match_command;

#[derive(Debug)]
pub enum Errors {
    CouldNotGetPath,
    DockerNotFound,
    CommandExecutionFailed,
    STDINError,
    IOError,
    InvalidOption,
    UTF8Error,
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

    match_command(&args, commands_index, &docker)
}

fn find_docker() -> Result<PathBuf, Errors> {
    let output = exec_shell_cmd(str!("which docker"))?;

    let mut path = PathBuf::new();
    let path_string = string_from_uft8(output.stdout)?;

    path.push(path_string.trim());
    Ok(path)
}
