use std::path::PathBuf;

use crate::program_execution::exec_cmd;

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
pub fn filter_options(args: &Vec<String>, docker: &PathBuf) -> bool {
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

pub fn usage() {
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
