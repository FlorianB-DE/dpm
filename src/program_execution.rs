use std::{
    ffi, io,
    process::{self, Stdio},
};

use crate::Errors;

#[inline]
pub fn exec_shell_cmd(cmd: String) -> Result<process::Output, Errors> {
    exec_cmd("/bin/sh", vec!["-c".to_string(), cmd])
}

pub fn exec_cmd<S>(program: S, args: Vec<String>) -> Result<process::Output, Errors>
where
    S: AsRef<ffi::OsStr>,
{
    let out = Stdio::piped();
    match process::Command::new(&program)
        .stdout(out)
        .args(args)
        .output()
    {
        Ok(o) => Ok(o),
        Err(e) => print_error(&program, e),
    }
}

pub fn string_from_uft8(str: Vec<u8>) -> Result<String, Errors> {
    String::from_utf8(str).or(Err(Errors::UTF8Error))
}

#[inline]
pub fn print_output(output: Vec<u8>) -> Result<(), Errors> {
    print!("{}", string_from_uft8(output)?);
    Ok(())
}

fn print_error<S>(program: &S, e: io::Error) -> Result<process::Output, Errors>
where
    S: AsRef<ffi::OsStr>,
{
    eprintln!(
        "During execution of '{}', the following error occured:\n{}\n",
        program.as_ref().to_str().unwrap_or("unknown"),
        e
    );
    Err(Errors::CommandExecutionFailed)
}
