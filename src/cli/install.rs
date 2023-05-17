use std::{collections::HashMap, path::PathBuf};

use crate::{
    program_execution::{exec_cmd, print_output},
    Errors,
};

use super::options::handle_options;

pub fn run(args: &Vec<String>, cmd_index: usize, docker: &PathBuf) -> Result<(), Errors> {
    let mut options: HashMap<&str, String> = HashMap::new();
    let mut added_index = 0;
    options.insert("tag", str!("latest"));

    loop {
        let current_arg = args.get(cmd_index + added_index).ok_or_else(missing_program)?;

        if !current_arg.starts_with("-") {
            break;
        }

        added_index += match handle_options(args, cmd_index, &mut options, &usage()) {
            Some(i) => i,
            None => {
                return Err(Errors::InvalidOption);
            }
        };
    }

    let program_indices = cmd_index + added_index;

    let program = match args.get(program_indices) {
        Some(p) => format!("{}:{}", p, options.get("tag").unwrap_or(&str!("latest"))),
        None => return Err(missing_program())
        
    };

    let pull = exec_cmd(docker, vec![str!("pull"), program])?;
    print_output(pull.stdout)
}

fn missing_program() -> Errors {
    eprintln!("missing argument 'program'.\nUsage:");
    eprintln!("{}", usage());
    Errors::MissingArgument
}

#[inline]
fn usage() -> String {
    str!(
        "dpm install [options] program
        
Options:
    -t, --tag: specify image tag default: '--tag latest'"
    )
}
