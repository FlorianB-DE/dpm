use std::{ffi, io, process};

#[inline]
pub fn exec_cmd<S>(program: S, args: Vec<String>) -> io::Result<process::Output>
where
    S: AsRef<ffi::OsStr>,
{
    process::Command::new(program).args(args).output()
}

#[inline]
pub fn exec_shell_cmd(cmd: String) -> io::Result<process::Output> {
    exec_cmd(
        "/bin/sh",
        vec!["-c".to_string(), cmd],
    )
}
