use anyhow::{Result, anyhow};
use inquire::{Select};

const MIT: &str = include_str!("../licenses/MIT");
const ISC: &str = include_str!("../licenses/ISC");

pub fn get_license_content() -> Result<String> {
    let licenses = vec!["MIT", "ISC"];
    let license = Select::new("Choose your license?", licenses).prompt()?;
    let content = match license {
        "MIT" => MIT.to_owned(),
        "ISC" => ISC.to_owned(),
        _ => return Err(anyhow!("Unknown license")),
    };

    Ok(content)
}
