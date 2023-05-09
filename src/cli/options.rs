
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