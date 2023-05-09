mod program_execution;

use std::{env, path::PathBuf};

use program_execution::{exec_cmd, exec_shell_cmd};

#[derive(Debug)]
enum Errors {
    CouldNotGetPath,
    DockerNotFound,
    CommandExecutionFailed,
    STDINError,
    IOError,
}

fn main() -> Result<(), Errors> {
    let path = env::current_dir().or(Err(Errors::CouldNotGetPath))?;

    let docker = find_docker().or(Err(Errors::DockerNotFound))?;

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        usage();
        return Ok(());
    }

    if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
        let default = "failed to get version from Docker daemon".to_string();
        let docker_version = match exec_cmd(docker, vec!["--version".to_string()]) {
            Ok(output) => String::from_utf8(output.stdout).unwrap_or(default),
            Err(_) => default,
        };
        println!(
            "dpm: {}\ndocker: {}",
            env!("CARGO_PKG_VERSION"),
            docker_version
        );
        return Ok(());
    }

    println!(
        "docker at: {}, pwd: {}, exec: {}",
        docker.display(),
        path.display(),
        args[0]
    );

    let _docker_output = exec_cmd(&docker, Vec::new()).or_else(|e| {
        eprintln!("error while trying to execute {}: \n {e}", docker.display());
        Err(Errors::IOError)
    })?;
    // println!("{}", String::from_utf8(_docker_output.stderr).unwrap());

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

fn usage() {
    print!(
        "
Usage: dpm [OPTIONS] COMMAND [ARGS...]

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
