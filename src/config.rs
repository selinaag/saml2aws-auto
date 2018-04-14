use std::io;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::env;

use serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_filename")]
    filename: String,
    pub groups: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub accounts: Vec<Account>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub arn: String,
}

fn default_filename() -> String {
    let mut path = env::home_dir().unwrap();
    path.push(".saml2aws-auto.yml");

    format!("{}", path.to_str().unwrap())
}

pub fn load_or_default() -> Result<Config, io::Error> {
    let default = default_filename();
    let paths = vec!["./saml2aws-auto.yml", &default];

    for path in &paths {
        if Path::new(path).exists() {
            let mut f = File::open(path)?;

            let mut buf = String::new();

            f.read_to_string(&mut buf)?;

            return match serde_yaml::from_str::<Config>(&buf) {
                Ok(mut cfg) => {
                    cfg.filename = path.to_owned().into();

                    Ok(cfg)
                }
                Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.description())),
            };
        }
    }

    Ok(Config {
        filename: default_filename(),
        groups: vec![],
    })
}

impl Config {
    pub fn save(&self) -> Result<(), io::Error> {
        let f = File::create(&self.filename)?;

        serde_yaml::to_writer(f, self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.description()))
    }
}
