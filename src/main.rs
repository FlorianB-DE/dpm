#[macro_use]
mod macros;
mod cli;
mod program_execution;
mod option_manager;
mod configuration_manager;

use std::{env, path::PathBuf};

use program_execution::{exec_shell_cmd, string_from_uft8};
use cli::{filter_options, usage};

/// Represents the possible errors that can occur in the program.
#[derive(Debug)]
pub enum Errors {
    /// Error when the current path could not be retrieved.
    CouldNotGetPath,
    /// Error when Docker is not found.
    DockerNotFound,
    /// Error when a command execution fails.
    CommandExecutionFailed,
    /// Error thrown when reading STDIN from spawned process.
    STDINError,
    /// I/O error.
    IOError,
    /// Error for an invalid option.
    InvalidOption,
    /// Error related to UTF-8 encoding/decoding.
    UTF8Error,
    /// Error when a required argument is missing.
    MissingArgument,
    /// Error when loading the configuration File
    ConfigLoadFailed,
    /// Error when saving the configuration File
    ConfigSaveFailed,
    InsufficientRights,
}

fn main() -> Result<(), Errors> {
    // Get the current directory path or return an error if it fails.
    let _path = env::current_dir().or(Err(Errors::CouldNotGetPath))?;

    // Find Docker executable or return an error if it is not found.
    let docker = find_docker().or(Err(Errors::DockerNotFound))?;

    // Collect command-line arguments.
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{}", usage()); // Print the usage information if no arguments are provided
        return Ok(());
    }

    // When `filter_options` returns `None`, the program should exit.
    let args_added_indices = match filter_options(&args, &docker) {
        Ok(p) => match p {
            Some(u) => u, // OptionHandler returned Some, indicating continuation.
            None => return Ok(()), // OptionHandler returned None, indicating program exit.
        },
        Err(_) => {
            println!("{}", usage()); // Print the usage information if there was an error parsing options.
            return Err(Errors::MissingArgument);
        }
    };

    // Run the CLI with the processed arguments.
    cli::run(&args, args_added_indices + 1, &docker)
}

fn find_docker() -> Result<PathBuf, Errors> {
    // Execute the shell command to find the Docker executable.
    let output = exec_shell_cmd(str!("which docker"))?;

    let mut path = PathBuf::new();
    // Convert the output of the shell command to a UTF-8 string.
    let path_string = string_from_uft8(output.stdout)?;

    // Trim the path string and add it to the PathBuf.
    path.push(path_string.trim());
    Ok(path)
}
