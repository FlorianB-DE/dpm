use std::{cell::RefCell, collections::HashMap, env, path::PathBuf};

use users::{get_current_gid, get_current_uid};

use crate::{
    configuration_manager::AppConfig,
    option_manager::{handle_options, ArgLen, OptionHandler},
    program_execution::{exec_cmd, print_output},
    sources::get_program_image,
    Errors,
};

use super::install::{install, missing_program};

pub fn run(
    args: &Vec<String>,
    cmd_index: usize,
    docker: &PathBuf,
    config: &mut AppConfig,
) -> Result<(), Errors> {
    let (added_index, options) = match read_options(args, cmd_index)? {
        Some(p) => p,
        None => return Ok(()),
    };

    // Calculate the index of the program argument
    let program_indices = cmd_index + added_index;
    let program = args.get(program_indices).ok_or_else(missing_program)?;

    let image = get_program_image(program)?;

    install(image.clone(), &options, docker, config)?;

    let cwd = env::current_dir().or(Err(Errors::IOError))?;
    let cwd_path = cwd.as_path().to_str().ok_or(Errors::UTF8Error)?;
    let container_path = format!("/tmp{cwd_path}");
    let volume = format!("{cwd_path}:{container_path}");

    let mut docker_args: Vec<String> = vec![
        str!("run"),
        str!("--workdir"),
        container_path,
        format!("--user=\"{}:{}\"", get_current_uid(), get_current_gid()),
        str!("--volume"),
        volume,
        str!("--volume=\"/etc/group:/etc/group:ro\""),
        str!("--volume=\"/etc/passwd:/etc/passwd:ro\""),
        str!("--volume=\"/etc/shadow:/etc/shadow:ro\""),
        str!("--rm"),
        image,
        program.to_owned(),
    ];
    let mut index = program_indices + 1;
    while let Some(item) = args.get(index) {
        docker_args.push(item.clone());
        index += 1;
    }

    println!("{:?}", docker_args);

    let out = exec_cmd(docker, docker_args)?;
    print_output(out.stdout)?;
    print_output(out.stderr)
}

fn read_options(
    args: &Vec<String>,
    cmd_index: usize,
) -> Result<Option<(usize, HashMap<&str, String>)>, Errors> {
    // Create a RefCell to hold the collected options
    let collected_options: RefCell<HashMap<&str, String>> = RefCell::new(HashMap::new());
    // Define a mutable closure for handling the "tag" option
    let mut tag_handler = |list: &Vec<&String>| -> Result<bool, Errors> {
        let mut options_collection_ref = collected_options.borrow_mut();
        // Insert the value of the "tag" option into the options collection
        options_collection_ref.insert(
            "tag",
            list.get(0)
                .map_or_else(|| str!("latest"), |s| s.to_string()),
        );
        Ok(true)
    };

    // Handle options and retrieve the number of added indices
    let added_index = match handle_options(
        args,
        &vec![
            // OptionHandler for the "-h" and "--help" options
            OptionHandler::new(vec!["-h", "--help"], Default::default(), &mut |_| {
                println!("{}", usage());
                Ok(false)
            }),
            OptionHandler::new(vec!["-t", "--tag"], ArgLen::Usize(1), &mut tag_handler),
        ],
        cmd_index,
    )
    .or_else(|e| {
        eprintln!("{}", usage());
        Err(e)
    })? {
        Some(p) => p,
        None => return Ok(None),
    };

    Ok(Some((added_index, collected_options.into_inner())))
}

fn usage() -> &'static str {
    "lol"
}
