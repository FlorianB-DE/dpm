use crate::Errors;
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, fs::write, path::PathBuf};

static SOURCE_FILE_REMOTE: &'static str =
    "https://raw.githubusercontent.com/FlorianB-DE/dpm-sources/main/sources.yml";
static SOURCE_FILE_NAME: &'static str = "sources";

#[derive(Debug, Serialize, Deserialize)]
struct Program {
    commands: Vec<String>,
    image: Option<String>,
}

impl Default for Program {
    fn default() -> Self {
        Self {
            commands: Vec::new(),
            image: None,
        }
    }
}

type SourcesFile = HashMap<String, HashMap<String, Program>>;

pub fn get_program_image(program: String) -> Result<String, Errors> {
    let sources = load_file()?;

    find_image_path(&program, sources).ok_or_else(|| {
        eprintln!(
            "Program {program} not found! Consider updating the sources file with:\n\tdpm update"
        );
        Errors::ProgramNotFound
    })
}

fn find_image_path(program: &String, sources: SourcesFile) -> Option<String> {
    for (source, programs) in &sources {
        for (program_name, program_struct) in programs {
            if !program_struct.commands.contains(&program) {
                continue;
            }
            let image = match program_struct.image {
                Some(ref s) => s,
                None => program_name,
            };
            // else (found program)
            return Some(format!("{source}/{image}"));
        }
    }

    None
}

fn load_file() -> Result<SourcesFile, Errors> {
    let path = confy::get_configuration_file_path(env!("CARGO_PKG_NAME"), SOURCE_FILE_NAME)
        .or(Err(Errors::IOError))?;
    if !path.exists() {
        println!("no source file found at {}", path.display());
        update_source(Some(&path))?;
    }

    confy::load(env!("CARGO_PKG_NAME"), SOURCE_FILE_NAME).or_else(|e| {
        eprintln!("{}", e);
        Err(Errors::ConfigLoadFailed)
    })
}

pub fn update_source(path: Option<&PathBuf>) -> Result<(), Errors> {
    let content = fetch_source_from_remote()?;
    let config_path = confy::get_configuration_file_path(env!("CARGO_PKG_NAME"), SOURCE_FILE_NAME)
        .or(Err(Errors::IOError))?;
    println!("writing updated sources list to {}", config_path.display());
    write(path.unwrap_or(&config_path), content).or_else(|e| {
        eprintln!("{}", e);
        Err(Errors::SavingSourcesFileFailed)
    })?;
    Ok(())
}

fn fetch_source_from_remote() -> Result<String, Errors> {
    println!("downloading source from {SOURCE_FILE_REMOTE}");
    let resp = reqwest::blocking::get(SOURCE_FILE_REMOTE).or_else(|e| {
        eprintln!("{e}");
        return Err(Errors::HTTPRequestFailed);
    })?;
    resp.text().or(Err(Errors::UTF8Error))
}
