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

    // for app_name in &config.apps {
    //     match config.apps_all.get(app_name) {
    //         Some(app) => {
    //             let changelog_path = app.path_changelog().to_str().unwrap().to_string();

    //             println!("bumping version for \"{}\"", app_name);
    //             println!("using changelog at \"{}\"", changelog_path);

    //             let mut changelog = Changelog::new();
    //             changelog.from(&changelog_path);
    //             changelog.dch();
    //             changelog.to(&changelog_path);
    //         }
    //         None => println!("missing configuration for {}.", app_name),
    //     }
    // }

    let mut changelog = Changelog::new();
    changelog.from("/vagrant/dch/example-project/debian/changelog-1");
    // changelog.from("/vagrant/dch/example-project/debian/changelog-2");
    // changelog.from("/vagrant/dch/example-project/debian/changelog-3");
    changelog.dch();
    changelog.to("/vagrant/dch/example-project/debian-out/changelog-1");
    // changelog.to("/vagrant/dch/example-project/debian-out/changelog-2");
    // changelog.to("/vagrant/dch/example-project/debian-out/changelog-3");

    let mut dchfile = DCHFile::new(
        "/vagrant/dch/example-project/debian/Dchfile",
        "python-project-bin-deb",
    ).unwrap();
    println!("{}", dchfile.to_string());

    let finsih_at = UTC::now();
    println!("finished at {}", finsih_at - start_at);
}
