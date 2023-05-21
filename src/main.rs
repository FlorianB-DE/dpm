#[macro_use]
mod macros;
mod cli;
mod program_execution;

use std::{env, path::PathBuf};

use program_execution::{exec_shell_cmd, string_from_uft8};
use cli::{filter_options, usage};

#[derive(Debug)]
pub enum Errors {
    CouldNotGetPath,
    DockerNotFound,
    CommandExecutionFailed,
    STDINError,
    IOError,
    InvalidOption,
    UTF8Error,
    MissingArgument,
}

fn main() -> Result<(), Errors> {
    let _path = env::current_dir().or(Err(Errors::CouldNotGetPath))?;
    let docker = find_docker().or(Err(Errors::DockerNotFound))?;
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{}", usage());
        return Ok(());
    }

    // when filter_options returns none, the program should exit
    let args_added_indecies = match filter_options(&args, &docker) {
        Ok(p) => match p {
            Some(u) => u,
            None => return Ok(()),
        },
        Err(_) => {
            println!("{}", usage());
            return Err(Errors::MissingArgument)
        }
    };

    cli::run(&args, args_added_indecies + 1, &docker)
}

fn find_docker() -> Result<PathBuf, Errors> {
    let output = exec_shell_cmd(str!("which docker"))?;

    let mut path = PathBuf::new();
    let path_string = string_from_uft8(output.stdout)?;

    path.push(path_string.trim());
    Ok(path)
}
