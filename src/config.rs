use std::fs::File;
use std::fmt;
use std::error;
use std::io::Read;
use std::io;
use std::convert::From;
use std::collections::HashMap;
use std::path::Path;
use std::string::FromUtf8Error;

use clap::{App, Arg};
use yaml_rust::YamlLoader;
use yaml_rust::scanner::ScanError;


const APP_NAME:         &'static str = "dch";
const AUTHOR_NAME:      &'static str = "Ivan Egorov";
const AUTHOR_EMAIL:     &'static str = "vany.egorov@gmail.com";
const DEFAULT_CONFIG:   &'static str = "~/.dchrc";


#[derive(Debug)]
pub enum ConfigError {
    IO(io::Error),
    UTF8(FromUtf8Error),
    YAML(ScanError),
    YAMLMissingDocument,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::IO(ref err)   => write!(f, "failed to open or read config \
                                                     file: {}", err),
            ConfigError::UTF8(ref err) => write!(f, "failed to convert config file \
                                                     content to utf8: {}", err),
            ConfigError::YAML(ref err) => write!(f, "failed to parse YAML content of \
                                                     config file: {}", err),
            ConfigError::YAMLMissingDocument => write!(f, "no yaml documents in \
                                                           config file"),
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        ConfigError::IO(err)
    }
}

impl From<FromUtf8Error> for ConfigError {
    fn from(err: FromUtf8Error) -> ConfigError {
        ConfigError::UTF8(err)
    }
}

impl From<ScanError> for ConfigError {
    fn from(err: ScanError) -> ConfigError {
        ConfigError::YAML(err)
    }
}

impl error::Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::IO(ref err) => err.description(),
            ConfigError::UTF8(ref err) => err.description(),
            ConfigError::YAML(ref err) => err.description(),
            _ => "",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ConfigError::IO(ref err) => Some(err),
            ConfigError::UTF8(ref err) => Some(err),
            ConfigError::YAML(ref err) => Some(err),
            _ => None,
        }
    }
}

pub struct ConfigPackage {
    pub name: String,
    pub path: String,
    pub path_changelog: String,
    pub path_dchfile: String,
}

impl ConfigPackage {
    pub fn calculate_path_changelog(&self) -> String {
        Path::new(&self.path)
            .join("debian")
            .join("changelog")
            .to_str().unwrap().to_string()

    }

    pub fn calculate_path_dchfile(&self) -> String {
        Path::new(&self.path)
            .join("debian")
            .join("Dchfile")
            .to_str().unwrap().to_string()
    }
}

pub struct ConfigMantainer {
    pub name: String,
    pub email: String,
}

pub struct Config {
    pub path: String,
    pub packages: Vec<String>,
    pub packages_all: HashMap<String, ConfigPackage>,
    pub mantainer: ConfigMantainer,
}

impl Config {
    pub fn to_string(&self) -> String {
        let mut s = String::new();

        s.push_str("packages:\n");
        for package in self.packages.iter() {
            s.push_str(&format!("\t - {}\n", package));
        }

        s.push_str("packages-all:\n");
        for (_, app) in self.packages_all.iter() {
            s.push_str(&format!("\t {}:\n", app.name));
            s.push_str(&format!("\t\t name: {}\n", app.name));
            s.push_str(&format!("\t\t path: {}\n", app.path));
            s.push_str(&format!("\t\t path-changelog: {}\n", app.path_changelog));
            s.push_str(&format!("\t\t path-dchfile: {}\n", app.path_dchfile));
        }

        s.push_str("mantainer:\n");
        s.push_str(&format!("\t name: {}\n", self.mantainer.name));
        s.push_str(&format!("\t email: {}\n", self.mantainer.email));

        s
    }

    pub fn new() -> Result<Config, ConfigError> {
        let flags = App::new(APP_NAME)
            .version(&crate_version!()[..])
            .author(&format!("{} <{}>", AUTHOR_NAME, AUTHOR_EMAIL))
            .arg(Arg::with_name("PACKAGES")
                .multiple(true)
                .required(false)
                .help("projects to work with"))
            .arg(Arg::with_name("CONFIG")
                .short("c")
                .long("config")
                .help("Sets a custom config file")
                .takes_value(true))
            .get_matches();

        let path = flags.value_of("CONFIG").unwrap_or(DEFAULT_CONFIG);

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(ConfigError::IO(err)),
        };

        let mut file_content_utf8 = Vec::new();
        if let Err(err) = file.read_to_end(&mut file_content_utf8) {
            return Err(ConfigError::IO(err))
        };
        let file_content = match String::from_utf8(file_content_utf8) {
            Ok(file_content) => file_content,
            Err(err) => return Err(ConfigError::UTF8(err)),
        };

        let yaml_configs = match YamlLoader::load_from_str(&file_content) {
            Ok(yaml_configs) => yaml_configs,
            Err(err) => return Err(ConfigError::YAML(err)),
        };

        let yaml_config = match yaml_configs.first() {
            Some(yaml_config) => yaml_config,
            None => return Err(ConfigError::YAMLMissingDocument),
        };

        let flag_packages = match flags.values_of("PACKAGES") {
            Some(packages) => packages,
            None => Vec::new(),
        };

        let mut packages: Vec<String> = Vec::new();
        for package in flag_packages { packages.push(package.to_string()); };

        let mut it = Config {
            path: path.to_string(),
            packages: packages,
            packages_all: HashMap::new(),
            mantainer: ConfigMantainer {
                name: yaml_config["mantainer"]["name"].as_str().unwrap().to_string(),
                email: yaml_config["mantainer"]["email"].as_str().unwrap().to_string(),
            },
        };

        for (package_name, package_config) in yaml_config["packages"].as_hash().unwrap() {
            let mut package = ConfigPackage {
                name: package_name.as_str().unwrap().to_string(),
                path: package_config["path"].as_str().unwrap().to_string(),
                path_changelog: "".to_string(),
                path_dchfile: "".to_string(),
            };

            package.path_changelog = package_config["path-changelog"]
                .as_str()
                .unwrap_or(&package.calculate_path_changelog())
                .to_string();
            package.path_dchfile = package_config["path-dchfile"]
                .as_str()
                .unwrap_or(&package.calculate_path_dchfile())
                .to_string();

            it.packages_all.insert(String::from(package.name.to_string()), package);
        }

        Ok(it)
    }
}
