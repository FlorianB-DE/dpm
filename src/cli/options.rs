use std::ops::Range;

use crate::Errors;


#[derive(Clone)]
pub enum ArgLen {
    _Range(Range<usize>),
    Usize(usize),
}

impl Default for ArgLen {
    fn default() -> Self {
        Self::Usize(0)
    }
}

impl Into<Range<usize>> for ArgLen {
    fn into(self) -> Range<usize> {
        match self {
            ArgLen::Usize(u) => 0..u,
            ArgLen::_Range(r) => r,
        }
    }
}

pub struct OptionHandler<'a> {
    for_flag: Vec<&'static str>,
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
    calling_arg: &String
) -> Result<Option<usize>, Errors> {
    let fail_on_missing = match handler.arg_len {
        ArgLen::Usize(_) => true,
        ArgLen::_Range(_) => false,
    };

    let range: Range<usize> = handler.arg_len.to_owned().into();
    let mut options: Vec<&String> = Vec::with_capacity(range.len());

    // collect args
    for i in range {
        let arg = args.get(start_index + i);
        match arg {
            Some(a) => options.push(a),
            None => {
                if fail_on_missing {
                    // everything of this I hate! 5 levels of indentation ;((
                    eprintln!("Expected at minimum {} arguments for option {calling_arg}! Got {i}", i + 1);
                    return Err(Errors::MissingArgument);
                } else {
                    break;
                }
            }
        }
    }

    let options_len = options.len();

    // call handler function
    if (handler.handler)(&options)? {
        return Ok(Some(options_len));
    }

    Ok(None)
}
