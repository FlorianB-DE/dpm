mod program_execution;
#[macro_use] mod macros;

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
        // can never be zero due to the fact that the first element is always the program itself
        usage();
        return Ok(());
    }

    if filter_options(&args, &docker) {
        return Ok(());
    }

    println!(
        "docker at: {}, pwd: {}, exec: {}",
        docker.display(),
        path.display(),
        args[0]
    );

    let _docker_output = exec_cmd(&docker, Vec::new()).or_else(|e| {
        eprintln!("error while trying to execute {}: \n {}", docker.display(), e);
        Err(Errors::IOError)
    })?;
    // println!("{}", String::from_utf8(_docker_output.stderr).unwrap());

    Ok(())
}

fn find_docker() -> Result<PathBuf, Errors> {
    let output = exec_shell_cmd(str!("which docker")).or(Err(Errors::CommandExecutionFailed))?;

    let mut path = PathBuf::new();
    let path_string = String::from_utf8(output.stdout).or(Err(Errors::STDINError))?;

    path.push(path_string.trim());
    Ok(path)
}

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

/// returns true when the program should exit
fn filter_options(args: &Vec<String>, docker: &PathBuf) -> bool {
let mut options: Vec<&str> = Vec::new();

    for i in 1..args.len() {
        if !args[i].starts_with("-") {
            break;
        }

        options.push(&args[i]);
    }


    if options.contains(&"-v") || options.contains(&"--version") {
        print!("{}", get_version(docker));
        return true;
    }

    if options.contains(&"-h") || options.contains(&"--help") {
        usage();
        return true;
    }

    false
}

fn usage() {
    print!(
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
