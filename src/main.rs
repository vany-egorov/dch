extern crate dch;
extern crate chrono;
extern crate yaml_rust;

use chrono::UTC;
use dch::changelog::Changelog;
use dch::config::Config;
use dch::dchfile::DCHFile;


fn main() {
    let config = match Config::new() {
        Ok(config) => config,
        Err(e) => panic!("{}", e),
    };

    println!("=> reading configuration file from: \"{}\"", config.path);
    println!("=> working configuration is:");
    println!("{}", config.to_string());

    println!("start");
    let start_at = UTC::now();

    for package_name in &config.packages {
        match config.packages_all.get(package_name) {
            Some(package) => {
                println!("bumping version for \"{}\"", package_name);
                println!("using changelog at \"{}\"", package.path_changelog);
                println!("using dchfile at \"{}\"", package.path_dchfile);

                let mut dchfile = DCHFile::new(
                    &package.path_dchfile,
                    package_name,
                ).unwrap();
                println!("dchfile:");
                println!("{}", dchfile.to_string());

                let mut changelog = Changelog::new();
                changelog.from(&package.path_changelog);
                changelog.up(
                    dchfile.package,
                    dchfile.version,
                    dchfile.distribution,
                    dchfile.urgency,

                    dchfile.details,

                    config.mantainer.name.to_string(),
                    config.mantainer.email.to_string(),
                );
                println!("up:");
                println!("{}", changelog.records[0].to_string());
                changelog.to(&package.path_changelog);
            }
            None => println!("missing configuration for {}.", package_name),
        }
    }

    // let mut changelog = Changelog::new();
    // changelog.from("/vagrant/dch/example-project/debian/changelog-1");
    // changelog.from("/vagrant/dch/example-project/debian/changelog-2");
    // changelog.from("/vagrant/dch/example-project/debian/changelog-3");
    // changelog.up();
    // changelog.to("/vagrant/dch/example-project/debian-out/changelog-1");
    // changelog.to("/vagrant/dch/example-project/debian-out/changelog-2");
    // changelog.to("/vagrant/dch/example-project/debian-out/changelog-3");

    let finsih_at = UTC::now();
    println!("finished at {}", finsih_at - start_at);
}
