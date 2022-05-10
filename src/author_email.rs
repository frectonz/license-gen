use std::{fs, process::Command};

use anyhow::Result;
use inquire::{Select, Text};
use serde_json::Value;

fn get_author_email_from_git() -> Result<String> {
    let command = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.email")
        .output()?;

    let author_email = String::from_utf8(command.stdout)?;
    let author_email = author_email.trim();

    return Ok(author_email.into());
}

fn get_author_email_from_package_json() -> Result<String> {
    let content = fs::read_to_string("package.json")?;
    let value: Value = serde_json::from_str(content.as_str())?;

    let author_email = value["author"]["email"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("author email not found"))?;

    return Ok(author_email.into());
}

pub fn get_author_email() -> Result<String> {
    let author_emails = vec![
        get_author_email_from_package_json().ok(),
        get_author_email_from_git().ok(),
    ];

    let mut author_emails = author_emails
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    const EMAIL_PROMPT: &str = "No, I will input my email.";
    author_emails.push(EMAIL_PROMPT.into());

    let mut email = Select::new("Choose yor email?", author_emails).prompt()?;

    if email == EMAIL_PROMPT {
        email = Text::new("What is your email?").prompt()?;
    }

    Ok(email)
}
