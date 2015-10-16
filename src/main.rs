extern crate dch;
extern crate chrono;

use dch::changelog::Changelog;
use chrono::{UTC};


fn main() {
    println!("start");
    let start_at = UTC::now();

    let mut changelog = Changelog::new();
    changelog.from("/vagrant/dch/example-project/debian/changelog");
    changelog.dch();
    changelog.to("/vagrant/dch/example-project/debian/changelog-1");

    let finsih_at = UTC::now();
    println!("finished at {}", finsih_at - start_at);
}
