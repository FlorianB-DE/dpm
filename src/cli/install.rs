use std::{collections::HashMap, path::PathBuf};

use crate::{program_execution::exec_cmd, Errors};

pub fn run(args: &Vec<String>, cmd_index: usize, docker: &PathBuf) -> Result<(), Errors> {
    if args.len() == cmd_index {
        usage();
        return Ok(());
    }

    let mut options: HashMap<&str, String> = HashMap::new();
    let mut added_index = 0;
    options.insert("tag", str!("latest"));

    match args.get(cmd_index) {
        Some(o) => {
            if o.starts_with("-") {
                added_index = match handle_options(args, cmd_index, &mut options) {
                    Some(i) => i,
                    None => {
                        return Ok(());
                    }
                };
            }
        }
        None => {
            eprintln!("{}", usage());
            return Ok(());
        }
    }

    let program_indices = cmd_index + added_index;

    let program = match args.get(program_indices) {
        Some(p) => format!("{}:{}", p, options.get("tag").unwrap_or(&str!("latest"))),
        None => {
            eprintln!("missing argument 'program'.\nUsage:");
            eprintln!("{}", usage());
            return Ok(());
        }
    };

    let pull = exec_cmd(docker, vec![str!("pull"), program])?;
    print!("{}", String::from_utf8(pull.stderr).or(Err(Errors::UTF8Error))?);
    Ok(())
}

fn handle_options(
    args: &Vec<String>,
    cmd_index: usize,
    options: &mut HashMap<&str, String>,
) -> Option<usize> {
    let mut added_indices = 1;
    match args[cmd_index].as_str() {
        "--tag" | "-t" => {
            if cmd_index + added_indices < args.len() {
                options.insert(
                    "tag",
                    args.get(cmd_index + added_indices)
                        .unwrap_or(&str!("latest"))
                        .to_owned(),
                );
                added_indices += 1;
            }
        }
        "-h" | "--help" => {
            println!("{}", usage());
            return None;
        }
        _ => {
            eprintln!("unknown option '{}'", args[cmd_index]);
            return None;
        }
    }
    Some(added_indices)
}

#[inline]
fn usage() -> String {
    str!(
        "dpm install [options] program
        
Options:
    -t, --tag: specify image tag default: '--tag latest'"
    )
}