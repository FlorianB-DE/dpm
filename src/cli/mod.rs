mod install;
mod options;

use std::error::Error;
use std::path::PathBuf;

use crate::program_execution::exec_cmd;

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
        usage();
        return None;
    }
    // there are currently two options. u8 has the opportunity to contain 255...
    Some(options.len() + 1)
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

pub fn match_command(
    args: &Vec<String>,
    cmd_index: usize,
    docker: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let cmd = match args.get(cmd_index) {
        Some(i) => i,
        None => {
            usage();
            return Ok(());
        }
    };

    match cmd.as_str() {
        "install" => install::run(args, cmd_index + 1, docker),
        "hello" => {
            let _docker_output = exec_cmd(&docker, vec![str!("run"), str!("hello-world")])?;
            println!("{}", String::from_utf8(_docker_output.stdout)?);
            return Ok(());
        }
        _ => {
            println!(
                "command '{}' not found. Use 
    dpm --help 
to check usage", cmd
            );
            Ok(())
        }
    }
}
