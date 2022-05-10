use std::{fs::File, io::Write};

use anyhow::{anyhow, Result};
use chrono::{Datelike, Utc};
use inquire::{Confirm, Select};

use license_gen::{
    get_author_email, get_author_name,
    licenses::{ISC, MIT},
};

fn main() -> Result<()> {
    let author_name = get_author_name()?;
    let author_email = get_author_email()?;
    let mut license_content = get_license_content()?;
    let year = Utc::now().year();

    let mut license_file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("LICENSE")?;

    let license_file_has_content = license_file.metadata()?.len() != 0;
    if license_file_has_content {
        let confirm = Confirm::new("LICENSE file already exists. Overwrite?").prompt()?;
        if !confirm {
            println!("Aborted.");
            return Ok(());
        }
    }

    license_content = license_content
        .replace("[YEAR]", &year.to_string())
        .replace("[AUTHOR]", &format!("{author_name} <{author_email}>"));
    license_file.write_all(license_content.as_bytes())?;

    println!("Successfully generated license file 🎉");
    Ok(())
}

fn get_license_content() -> Result<String> {
    let licenses = vec!["MIT", "ISC"];
    let license = Select::new("Choose your license?", licenses).prompt()?;
    let content = match license {
        "MIT" => MIT.to_owned(),
        "ISC" => ISC.to_owned(),
        _ => return Err(anyhow!("Unknown license")),
    };

    Ok(content)
}
