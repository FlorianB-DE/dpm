#[macro_use] mod macros;
mod program_execution;
mod cli_lib;

use std::{env, path::PathBuf};

use program_execution::{exec_cmd, exec_shell_cmd};

use cli_lib::{filter_options, usage};

#[derive(Debug)]
enum Errors {
    CouldNotGetPath,
    DockerNotFound,
    CommandExecutionFailed,
    STDINError,
    IOError,
}

fn main() -> Result<(), Errors> {
    let _path = env::current_dir().or(Err(Errors::CouldNotGetPath))?;
    let docker = find_docker().or(Err(Errors::DockerNotFound))?;
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // can never be zero due to the fact that the first element is always the program itself
        usage();
        return Ok(());
    }

    if filter_options(&args, &docker) {
        return Ok(());
    }

    let _docker_output = exec_cmd(&docker, vec![str!("run"), str!("hello-world")]).or_else(|e| {
        eprintln!("error while trying to execute {}: \n {}", docker.display(), e);
        Err(Errors::IOError)
    })?;
    println!("{}", String::from_utf8(_docker_output.stdout).unwrap());

    Ok(())
}

fn find_docker() -> Result<PathBuf, Errors> {
    let output = exec_shell_cmd(str!("which docker")).or(Err(Errors::CommandExecutionFailed))?;

    let mut path = PathBuf::new();
    let path_string = String::from_utf8(output.stdout).or(Err(Errors::STDINError))?;

    path.push(path_string.trim());
    Ok(path)
}