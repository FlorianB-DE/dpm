use std::{fs, env, error::Error, io, path::PathBuf, process, str::FromStr};

fn main() -> Result<(), Error> {
    let _path = env::current_dir()?;

    let _docker = find_docker()?;

    let _args: Vec<String> = env::args().collect();

    println!(
        "docker at: {}, pwd: {}, exec: {}",
        _docker.display(),
        _path.display(),
        _args[0]
    );

    let _docker_output = exec_cmd(_docker, Vec::new())?;
    // println!("{:?}", String::from_utf8(docker_output.stderr).unwrap());

    Ok(())
}

fn find_docker() -> Result<PathBuf, Box<dyn Error>> {
    let output = exec_shell_cmd("which docker".to_string())?;

    let mut path = PathBuf::new();
    path.push(String::from_utf8(output.stdout)?);
    Ok(path)
}

fn exec_cmd(program: PathBuf, args: Vec<String>) -> io::Result<process::Output> {
    process::Command::new(program.to_str().unwrap()).args(args).output()
}

fn exec_shell_cmd(cmd: String) -> Result<process::Output, Box<dyn Error>> {
    Ok(exec_cmd(
        PathBuf::from_str("/bin/sh")?,
        vec!["-c".to_string(), cmd],
    )?)
}
