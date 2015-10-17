extern crate dch;
extern crate chrono;
extern crate yaml_rust;

use chrono::UTC;
use dch::changelog::Changelog;
use dch::config::{Config, ConfigError};


fn main() {
    let config = match Config::new() {
        Ok(config) => config,
        Err(e) => {
            match e {
                ConfigError::OpenConfigFile => panic!("open config file failed!")
            }
        }
    };

    println!("start");
    let start_at = UTC::now();

    let mut changelog = Changelog::new();
    changelog.from("/vagrant/dch/example-project/debian/changelog");
    changelog.dch();
    changelog.to("/vagrant/dch/example-project/debian/changelog-1");

    let finsih_at = UTC::now();
    println!("finished at {}", finsih_at - start_at);
}
