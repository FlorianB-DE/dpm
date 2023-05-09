mod program_execution;

use std::{env, path::PathBuf};

use program_execution::{exec_cmd, exec_shell_cmd};

#[derive(Debug)]
enum Errors {
    CouldNotGetPath,
    DockerNotFound,
    CommandExecutionFailed,
    STDINError,
    Unknown,
}

fn main() -> Result<(), Errors> {
    let _path = env::current_dir().or(Err(Errors::CouldNotGetPath))?;

    let _docker = find_docker().or(Err(Errors::DockerNotFound))?;

    let _args: Vec<String> = env::args().collect();

    println!(
        "docker at: {}, pwd: {}, exec: {}",
        _docker.display(),
        _path.display(),
        _args[0]
    );

    let _docker_output = exec_cmd(_docker, Vec::new()).or_else(|e| {
        eprintln!("{e}");

        Err(Errors::Unknown)
    })?;
    println!("{}", String::from_utf8(_docker_output.stderr).unwrap());

    Ok(())
}

fn find_docker() -> Result<PathBuf, Errors> {
    let output =
        exec_shell_cmd("which docker".to_string()).or(Err(Errors::CommandExecutionFailed))?;

    let mut path = PathBuf::new();
    let path_string = String::from_utf8(output.stdout).or(Err(Errors::STDINError))?;

    path.push(path_string.trim());
    Ok(path)
}
