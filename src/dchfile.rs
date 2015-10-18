use std::io;
use std::io::Read;
use std::fmt;
use std::error;
use std::fs::File;
use std::convert::From;
use std::string::FromUtf8Error;

use yaml_rust::YamlLoader;
use yaml_rust::scanner::ScanError;


#[derive(Debug)]
pub enum DCHFileError {
    IO(io::Error),
    UTF8(FromUtf8Error),
    YAML(ScanError),
    YAMLMissingDocument,
}

impl fmt::Display for DCHFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DCHFileError::IO(ref err)   => write!(f, "failed to open or read dchfile: {}", err),
            DCHFileError::UTF8(ref err) => write!(f, "failed to convert dchfile \
                                                      content to utf8: {}", err),
            DCHFileError::YAML(ref err) => write!(f, "failed to parse YAML content of \
                                                      dchfile: {}", err),
            DCHFileError::YAMLMissingDocument => write!(f, "no yaml documents found in dchfile"),
        }
    }
}

impl From<io::Error> for DCHFileError {
    fn from(err: io::Error) -> DCHFileError {
        DCHFileError::IO(err)
    }
}

impl From<FromUtf8Error> for DCHFileError {
    fn from(err: FromUtf8Error) -> DCHFileError {
        DCHFileError::UTF8(err)
    }
}

impl From<ScanError> for DCHFileError {
    fn from(err: ScanError) -> DCHFileError {
        DCHFileError::YAML(err)
    }
}

impl error::Error for DCHFileError {
    fn description(&self) -> &str {
        match *self {
            DCHFileError::IO(ref err) => err.description(),
            DCHFileError::UTF8(ref err) => err.description(),
            DCHFileError::YAML(ref err) => err.description(),
            _ => "",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DCHFileError::IO(ref err) => Some(err),
            DCHFileError::UTF8(ref err) => Some(err),
            DCHFileError::YAML(ref err) => Some(err),
            _ => None,
        }
    }
}

pub struct DCHFile {
    pub package: String,
    pub version: String,
    pub distribution: String,
    pub urgency: String,

    pub details: Vec<String>,

    pub before: Vec<Vec<String>>,
    pub after: Vec<Vec<String>>,
}

impl DCHFile {
    pub fn to_string(&self) -> String {
        let mut s = String::new();

        s.push_str(&format!("package: {}\n", self.package));
        s.push_str(&format!("version: {}\n", self.version));
        s.push_str(&format!("distribution: {}\n", self.distribution));
        s.push_str(&format!("urgency: {}\n", self.urgency));

        s
    }

    pub fn new(path: &str, name: &str) -> Result<DCHFile, DCHFileError> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(DCHFileError::IO(err)),
        };

        let mut file_content_utf8 = Vec::new();
        if let Err(err) = file.read_to_end(&mut file_content_utf8) {
            return Err(DCHFileError::IO(err))
        };
        let file_content = match String::from_utf8(file_content_utf8) {
            Ok(file_content) => file_content,
            Err(err) => return Err(DCHFileError::UTF8(err)),
        };

        let yamls = match YamlLoader::load_from_str(&file_content) {
            Ok(yamls) => yamls,
            Err(err) => return Err(DCHFileError::YAML(err)),
        };

        let yaml = match yamls.first() {
            Some(yaml) => yaml,
            None => return Err(DCHFileError::YAMLMissingDocument),
        };

        let mut it = DCHFile {
            package: yaml["package"].as_str().unwrap_or(&name).to_string(),
            version: yaml["version"].as_str().unwrap().to_string(),
            distribution: yaml["distribution"].as_str().unwrap_or("stable").to_string(),
            urgency: yaml["urgency"].as_str().unwrap_or("medium").to_string(),

            details: Vec::new(),

            before: Vec::new(),
            after: Vec::new(),
        };

        Ok(it)
    }
}
