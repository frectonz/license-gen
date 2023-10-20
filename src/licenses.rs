use anyhow::{anyhow, Result};
use inquire::Select;

const MIT: &str = include_str!("licenses/MIT.txt");
const ISC: &str = include_str!("licenses/ISC.txt");
const APACHE_V2: &str = include_str!("licenses/Apache-2.0.txt");

pub fn get_license_content() -> Result<String> {
    let licenses = vec!["MIT", "ISC", "Apache-2.0"];
    let license = Select::new("Choose your license?", licenses).prompt()?;
    let content = match license {
        "MIT" => MIT.to_owned(),
        "ISC" => ISC.to_owned(),
        "Apache-2.0" => APACHE_V2.to_owned(),
        _ => return Err(anyhow!("Unknown license")),
    };

    Ok(content)
}
