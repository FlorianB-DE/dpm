use std::path::PathBuf;

use crate::{configuration_manager::AppConfig, program_execution::exec_cmd, Errors};

use super::install;

pub fn run(
    args: &Vec<String>,
    cmd_index: usize,
    docker: &PathBuf,
    config: &mut AppConfig,
) -> Result<(), Errors> {
    install::run(args, cmd_index, docker, config)?;

    let out = exec_cmd(docker, args)?;
    Ok(())
}
