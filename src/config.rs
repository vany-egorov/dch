use std::fs;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use clap::{App, Arg};
use yaml_rust::YamlLoader;

pub struct ConfigApp {
    name: String,
}

pub struct Config {
    pub apps: HashMap<String, ConfigApp>,
}

pub enum ConfigError {
    OpenConfigFile
}

impl Config {
    pub fn new() -> Result<Config, ConfigError> {
        let flags = App::new("dch")
            .version(&crate_version!()[..])
            .author("Ivan Egorov <vany.egorov@gmail.com>")
            .arg(Arg::with_name("APPS")
                .multiple(true)
                .help("projects to work with"))
            .arg(Arg::with_name("CONFIG")
                .short("c")
                .long("config")
                .help("Sets a custom config file")
                .takes_value(true))
            .get_matches();

        let path = flags.value_of("CONFIG").unwrap_or("~/.dchrc");
        let apps = flags.values_of("APPS").unwrap();

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(ConfigError::OpenConfigFile),
        };

        let mut config_file_content_utf8 = Vec::new();
        file.read_to_end(&mut config_file_content_utf8);
        let config_file_content = String::from_utf8(config_file_content_utf8).unwrap();

        let configs = YamlLoader::load_from_str(&config_file_content).unwrap();
        let config = &configs[0];
        println!("{:?}", config);

        for app in apps {
            let app_path = config["apps"][app]["path"].as_str().unwrap();
            println!("Working with apps: {} path={}", app, app_path);

            let paths = fs::read_dir(app_path).unwrap();
            for path in paths {
                println!("Name: {}", path.unwrap().path().display())
            }
        }

        let it = Config{
            apps: HashMap::new(),
        };

        Ok(it)
    }
}
