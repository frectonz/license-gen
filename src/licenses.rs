use anyhow::{anyhow, Result};
use inquire::Select;

const MIT: &str = include_str!("licenses/MIT.txt");
const ISC: &str = include_str!("licenses/ISC.txt");
const APACHE_V2: &str = include_str!("licenses/Apache-2.0.txt");
const BSD_3_CLAUSE: &str = include_str!("licenses/BSD-3-Clause.txt");
const BSD_2_CLAUSE: &str = include_str!("licenses/BSD-2-Clause.txt");
const BSD_1_CLAUSE: &str = include_str!("licenses/BSD-1-Clause.txt");
const UNLICENSE: &str = include_str!("licenses/UNLICENSE.txt");
const WTFPL: &str = include_str!("licenses/WTFPL.txt");
const GLWTPL: &str = include_str!("licenses/GLWTPL.txt");

pub fn get_license_content() -> Result<String> {
    let licenses = vec![
        "MIT",
        "ISC",
        "Apache 2.0",
        "BSD 3-Clause",
        "BSD 2-Clause",
        "BSD 1-Clause",
        "Unlicense",
        "WTFPL",
        "GLWTPL"
    ];
    let license = Select::new("Choose your license?", licenses).prompt()?;
    let content = match license {
        "MIT" => MIT.to_owned(),
        "ISC" => ISC.to_owned(),
        "Apache 2.0" => APACHE_V2.to_owned(),
        "BSD 3-Clause" => BSD_3_CLAUSE.to_owned(),
        "BSD 2-Clause" => BSD_2_CLAUSE.to_owned(),
        "BSD 1-Clause" => BSD_1_CLAUSE.to_owned(),
        "Unlicense" => UNLICENSE.to_owned(),
        "WTFPL" => WTFPL.to_owned(),
        "GLWTPL" => GLWTPL.to_owned(),
        _ => return Err(anyhow!("Unknown license")),
    };

    Ok(content)
}
