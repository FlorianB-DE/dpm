use std::collections::HashMap;

// struct OptionHandler {
//     for_flag: &'static str,
//     arg_len: u32,
//     continue_execution: bool,
//     handler: fn(Vec<String>),
// }

// pub fn handle_options(
//     args: &Vec<String>,
//     options: Vec<OptionHandler>,
//     start_index: usize,
//     usage: &String,
// ) -> Option<usize> {
//     let mut added_index = 0;

//     while let Some(arg) = args.get(start_index + added_index) {
//         for option in options {
//             if arg != option.for_flag {
//                 continue;
//             }
//             handle_option(&option);
//             if !option.continue_execution {
//                 return None;
//             }
//         }
//     }

//     return Some(added_index);
// }

// fn handle_option(handler: &OptionHandler) {}

pub fn get_options(args: &Vec<String>, start: usize) -> Vec<&str> {
    let mut options: Vec<&str> = Vec::new();

    for i in start..args.len() {
        if !args[i].starts_with("-") {
            break;
        }

        options.push(&args[i]);
    }

    options
}

pub fn handle_options(
    args: &Vec<String>,
    cmd_index: usize,
    options: &mut HashMap<&str, String>,
    usage: &String,
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
            println!("{}", usage);
            return None;
        }
        _ => {
            eprintln!("unknown option '{}'", args[cmd_index]);
            return None;
        }
    }
    Some(added_indices)
}
