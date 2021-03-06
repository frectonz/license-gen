use std::{fs, process::Command};

use anyhow::Result;
use inquire::{Select, Text};
use serde_json::Value;

fn get_author_name_from_git() -> Result<String> {
    let command = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.name")
        .output()?;

    let author_name = String::from_utf8(command.stdout)?;
    let author_name = author_name.trim();

    return Ok(author_name.into());
}

fn get_author_name_from_package_json() -> Result<String> {
    let content = fs::read_to_string("package.json")?;
    let value: Value = serde_json::from_str(content.as_str())?;

    let author_name = value["author"]
        .as_str()
        .or(value["author"]["name"].as_str())
        .ok_or_else(|| anyhow::anyhow!("author name not found"))?;

    return Ok(author_name.into());
}

fn get_author_name_from_env() -> Result<String> {
    let author_name = std::env::var("USER")?;

    return Ok(author_name);
}

pub fn get_author_name() -> Result<String> {
    let author_names = vec![
        get_author_name_from_env().ok(),
        get_author_name_from_package_json().ok(),
        get_author_name_from_git().ok(),
    ];

    let mut author_names = author_names
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    const NAME_PROMPT: &str = "No, I will input my name.";
    author_names.push(NAME_PROMPT.into());

    let mut name = Select::new("Choose yor name?", author_names).prompt()?;

    if name == NAME_PROMPT {
        name = Text::new("What is your name?").prompt()?;
    }

    Ok(name)
}
