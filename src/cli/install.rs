use std::{cell::RefCell, collections::HashMap, path::PathBuf};

use crate::{
    configuration_manager::AppConfig,
    option_manager::{handle_options, ArgLen, OptionHandler},
    program_execution::{exec_cmd, print_output},
    sources::get_program_image,
    Errors,
};

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

    let program = get_program_image(program)?;

    install(program, &options, docker, config)
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

pub fn install(
    program: String,
    options: &HashMap<&str, String>,
    docker: &PathBuf,
    config: &mut AppConfig,
) -> Result<(), Errors> {
    // Retrieve the value of the "tag" option from the collected options
    // If not present, default to "latest"
    let tag = options
        .get("tag")
        .and_then(|f| Some(f.to_owned()))
        .unwrap_or(str!("latest"));

    
    let mut installed_tags = vec![tag.clone()];

    // Format the program with the tag
    let program_with_tag = format!("{}:{}", program, &tag);

    if let Some(p) = config.installed_programs.get(&program) {
        if p.contains(&tag) {
            println!("{program_with_tag} is already installed");
            return Ok(());
        }

        installed_tags.append(p.clone().as_mut());
    }

    // Execute the Docker command to pull the specified program
    let pull = exec_cmd(docker, vec!["pull", &program_with_tag])?;

    config.installed_programs.insert(program, installed_tags);

    let code = match pull.status.code() {
        Some(c) => c,
        None => {
            eprintln!("Docker terminated by signal");
            return Err(Errors::CommandExecutionFailed);
        }
    };

    if code == 0 {
        print_output(pull.stdout)?;
        println!("successfully installed {program_with_tag}");
        return Ok(());
    }

    eprintln!("Docker exited with code: {code}");
    print_output(pull.stderr)?;
    return Err(Errors::CommandExecutionFailed);
}

pub fn missing_program() -> Errors {
    eprintln!("Missing argument 'program'.\n\nUsage:");
    eprintln!("{}", usage());
    Errors::MissingArgument
}

#[inline]
fn usage() -> &'static str {
    include_str!("install_usage.txt")
}
