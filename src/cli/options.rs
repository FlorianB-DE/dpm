use std::ops::Range;

use crate::Errors;

#[derive(Clone)]
pub enum ArgLen {
    _Range(Range<usize>),
    Usize(usize),
}

impl Default for ArgLen {
    fn default() -> Self {
        // Default argument length is 0
        Self::Usize(0)
    }
}

impl Into<Range<usize>> for ArgLen {
    fn into(self) -> Range<usize> {
        match self {
            ArgLen::Usize(u) => 0..u, // Convert ArgLen::Usize into a Range starting from 0 and ending at u
            ArgLen::_Range(r) => r, // Convert ArgLen::_Range directly into a Range
        }
    }
}

pub struct OptionHandler<'a> {
    /// List of flags associated with this option handler
    for_flag: Vec<&'static str>,
    /// Expected argument length for this option handler
    arg_len: ArgLen,
    /// the bool in the return type indicates the continuation of execution:
    /// false means the program needs to terminate
    handler: &'a mut dyn Fn(&Vec<&String>) -> Result<bool, Errors>,
}

impl<'a> OptionHandler<'a> {
    pub fn new(
        for_flag: Vec<&'static str>,
        arg_len: ArgLen,
        handler: &'a mut dyn Fn(&Vec<&String>) -> Result<bool, Errors>,
    ) -> Self {
        OptionHandler {
            for_flag: for_flag,
            arg_len: arg_len,
            handler: handler,
        }
    }
}

pub fn handle_options(
    args: &Vec<String>,
    options: &Vec<OptionHandler>,
    start_index: usize,
) -> Result<Option<usize>, Errors> {
    let mut added_index = 0;

    while let Some(arg) = args.get(start_index + added_index) {
        if !arg.starts_with("-") {
            break;
        }
        let mut iteration = 0;
        let found_option = loop {
            let option = match options.get(iteration) {
                Some(p) => p,
                None => break false,
            };
            iteration += 1;
            if !option.for_flag.contains(&arg.as_str()) {
                continue;
            }
            added_index += 1;
            added_index += match handle_option(&option, start_index + added_index, args, arg)? {
                Some(p) => p,
                None => return Ok(None),
            };
            break true;
        };
        if !found_option {
            println!("Unknown tag: {arg}");
            return Err(Errors::InvalidOption);
        }
    }

    Ok(Some(added_index))
}

fn handle_option(
    handler: &OptionHandler,
    start_index: usize,
    args: &Vec<String>,
    calling_arg: &String,
) -> Result<Option<usize>, Errors> {
    let fail_on_missing = match handler.arg_len {
        ArgLen::Usize(_) => true,
        ArgLen::_Range(_) => false,
    };

    let range: Range<usize> = handler.arg_len.to_owned().into();
    let options: Vec<&String> = range.clone()
        .map(|i| args.get(start_index + i))
        .take_while(Option::is_some)
        .map(Option::unwrap)
        .collect();

    let options_len = options.len();

    if options_len < range.len() && fail_on_missing {
        eprintln!(
            "Expected at minimum {} arguments for option {}! Got {}",
            range.len(),
            calling_arg,
            options_len
        );
        return Err(Errors::MissingArgument);
    }

    if (handler.handler)(&options)? {
        Ok(Some(options_len))
    } else {
        Ok(None)
    }
}

