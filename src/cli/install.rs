use std::{collections::HashMap, error::Error, path::PathBuf};

use crate::program_execution::exec_cmd;

pub fn run(args: &Vec<String>, cmd_index: usize, docker: &PathBuf) -> Result<(), Box<dyn Error>> {
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
                added_index = handle_options(args, cmd_index, &mut options);
            }
        }
        None => {
            usage();
            return Ok(());
        }
    }

    let program_indices = cmd_index + added_index;

    let program = match args.get(program_indices) {
        Some(p) => format!("{}:{}", p, options.get("tag").unwrap_or(&str!("latest"))),
        None => {
            usage();
            return Ok(());
        }
    };

    let pull = exec_cmd(docker, vec![str!("pull"), program])?;
    print!("{}", String::from_utf8(pull.stderr)?);
    Ok(())
}

fn handle_options(args: &Vec<String>, cmd_index: usize, options: &mut HashMap<&str, String>) -> usize {
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
        _ => {}
    }
    added_indices
}

fn usage() {
    println!(
        "dpm install [options] program
        
Options:
    -t, --tag: specify image tag default: '--tag latest'"
    )
}
