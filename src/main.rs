use std::{env, process::Command, string::FromUtf8Error};

fn main() -> Result<()> {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let docker = find_docker()?;

    get_arguments();

    Ok(())
}

// annotation code written by @Tomyk9991 on Twitch

#[derive(Debug)]
enum DockerNotFoundError { 
    Utf8(FromUtf8Error)
}
 
impl From<FromUtf8Error> for DockerNotFoundError {
    fn from(s: FromUtf8Error) -> Self {
        Self::Utf8(s)
    }
}
 
fn find_docker() -> Result<String, DockerNotFoundError> {

    let output = Command::new("sh")
        .arg("-c")
        .arg("which docker")
        .output()
        .expect("failed to execute process");
    Ok(String::from_utf8(output.stdout)?)
}

fn get_arguments() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}
