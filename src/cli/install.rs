use std::{cell::RefCell, collections::HashMap, path::PathBuf};

use crate::{
    program_execution::{exec_cmd, print_output},
    Errors,
};

use crate::options::{handle_options, ArgLen, OptionHandler};

pub fn run(args: &Vec<String>, cmd_index: usize, docker: &PathBuf) -> Result<(), Errors> {
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
        None => return Ok(()),
    };

    // Calculate the index of the program argument
    let program_indices = cmd_index + added_index;
    let program = args.get(program_indices).ok_or_else(missing_program)?;

    // Retrieve the value of the "tag" option from the collected options
    // If not present, default to "latest"
    let tag = collected_options
        .borrow()
        .get("tag")
        .and_then(|f| Some(f.to_owned()))
        .unwrap_or(str!("latest"));

    // Format the program with the tag
    let program = format!("{}:{}", program, tag);

    // Execute the Docker command to pull the specified program
    let pull = exec_cmd(docker, vec![str!("pull"), program])?;
    print_output(pull.stdout)
}

fn missing_program() -> Errors {
    eprintln!("Missing argument 'program'.\n\nUsage:");
    eprintln!("{}", usage());
    Errors::MissingArgument
}

#[inline]
fn usage() -> &'static str {
    include_str!("install_usage.txt")
}
