extern crate dch;
#[macro_use(crate_version)]
extern crate clap;
extern crate chrono;
extern crate yaml_rust;

use std::io::Read;
use std::fs::File;
use dch::changelog::Changelog;
use chrono::UTC;
use clap::{App, Arg};
use yaml_rust::YamlLoader;


fn main() {
    let app = App::new("dch")
        .version(&crate_version!()[..])
        .author("Ivan Egorov <vany.egorov@gmail.com>")
        .arg(Arg::with_name("CONFIG")
            .short("c")
            .long("config")
            .help("Sets a custom config file")
            .takes_value(true))
        .get_matches();


    let config_path = app.value_of("CONFIG").unwrap_or("~/.dchrc");
    println!("Using config: {}", config_path);

    let mut config_file = File::open(config_path).unwrap();
    let mut config_file_content_utf8 = Vec::new();
    config_file.read_to_end(&mut config_file_content_utf8);
    let config_file_content = String::from_utf8(config_file_content_utf8).unwrap();

    let config = YamlLoader::load_from_str(&config_file_content).unwrap();
    println!("{:?}", config);

    println!("start");
    let start_at = UTC::now();

    let mut changelog = Changelog::new();
    changelog.from("/vagrant/dch/example-project/debian/changelog");
    changelog.dch();
    changelog.to("/vagrant/dch/example-project/debian/changelog-1");

    let finsih_at = UTC::now();
    println!("finished at {}", finsih_at - start_at);
}
