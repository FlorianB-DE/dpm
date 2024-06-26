mod install;
mod run;
mod update;

use std::path::PathBuf;

use crate::{
    configuration_manager::get_config,
    option_manager::{handle_options, OptionHandler},
    program_execution::{exec_cmd, print_output},
    Errors,
};

pub fn run(args: &Vec<String>, cmd_index: usize, docker: &PathBuf) -> Result<(), Errors> {
    let cmd = match args.get(cmd_index) {
        Some(i) => i,
        None => {
            print!("{}", usage());
            return Ok(());
        }
    };

    let mut config = get_config()?;

    match cmd.as_str() {
        "install" => install::run(args, cmd_index + 1, docker, &mut config),
        "update" => update::run(),
        "hello" => run_hello(docker),
        "run" => run::run(args, cmd_index + 1, docker, &mut config),
        _ => command_not_found(cmd),
    }
}

/// returns None when the program should exit
pub fn filter_options(args: &Vec<String>, docker: &PathBuf) -> Result<Option<usize>, Errors> {
    handle_options(
        args,
        &vec![
            OptionHandler::new(
                vec!["-v", "--version"],
                Default::default(),
                &mut move |_| {
                    println!("{}", get_version(docker));
                    Ok(false)
                },
            ),
            OptionHandler::new(vec!["-h", "--help"], Default::default(), &mut |_| {
                println!("{}", usage());
                Ok(false)
            }),
        ],
        1,
    )
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

fn run_hello(docker: &PathBuf) -> Result<(), Errors> {
    let docker_output = exec_cmd(&docker, vec![str!("run"), str!("hello-world")])?;
    print_output(docker_output.stdout)?;
    Ok(())
}

fn command_not_found(cmd: &String) -> Result<(), Errors> {
    println!(
        "command '{}' not found. Use \n    dpm --help\nto check usage",
        cmd
    );
    Ok(())
}

pub fn usage() -> &'static str {
    include_str!("program_usage.txt")
}
