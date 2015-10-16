use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use regex::Regex;
use chrono::{DateTime,UTC};
use record::Record;

pub struct Changelog {
    records: Vec<Record>,
}

impl Changelog {
    pub fn new() -> Changelog {
        Changelog{
            records: Vec::new(),
        }
    }

    pub fn _log(&self) {
        for record in self.records.iter() {
            record._log();
            println!("");
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();

        for record in self.records.iter() {
            s.push_str(&record.to_string());
            s.push_str("\n");
        }

        s
    }

    pub fn dch(&mut self) {
        let version = format!("3.0.0.{}", UTC::now().format("%Y%m%d-%H%M%S"));

        let mut record = Record::new();
        record.package = "cdn-api".to_string();
        record.version = version.to_string();
        record.distribution = "stable".to_string();
        record.urgency = "medium".to_string();

        record.details.push("autocommit-fix".to_string());

        record.mantainer_name = "Ivan Egorov".to_string();
        record.mantainer_email = "vany.egorov@gmail.com".to_string();

        self.records.insert(0, record);
    }

    pub fn from(&mut self, path: &str) {
        let re1 = Regex::new(r"(?x)
            ^
                (?P<package>[\w-]+)                           # package
            \s
                \((?P<version>[\d\.]+)\)                      # version
            \s
                (?P<distribution>[\w\s\d]+); # distribution
            \s
                urgency=(?P<urgency>\w+)                      # urgency
        $").unwrap();

        let re2 = Regex::new(r"(?x)
            ^\s*\*\s*           # (  * ) - details separator
                (?P<details>.*) # details
        ").unwrap();

        let re3 = Regex::new(r"(?x)
            ^\s*--\s*
                (?P<mantainer_name>[\d\w\s-_.=+:]+) # mantainer_name
            <
                (?P<mantainer_email>.*)             # mantainer_email
            >
            \s*
                (?P<date>[\d\w\s-_.=+:,]+)          # date
        $").unwrap();

        match File::open(path) {
            Err(why) => { println!("open file failed: {:?}", why); }
            Ok(file) => {
                let reader = BufReader::new(file);

                let mut is_re1 = false;
                let mut is_re2 = false;
                let mut is_re3 = false;

                let mut accumulator = Record::new();

                for line in reader.lines() {
                    let l = &line.unwrap();
                    if l == "" { continue; }; // FIXME

                    let mut is_re2_local = false; // FIXME

                    for cap in re1.captures_iter(l) {
                        is_re1 = true;

                        let package = cap.name("package").unwrap_or("failed-to-find-package-name");
                        let version = cap.name("version").unwrap_or("failed-to-find-package-version");
                        let distribution = cap.name("distribution").unwrap();
                        let urgency = cap.name("urgency").unwrap();

                        accumulator.package = package.to_string();
                        accumulator.version = version.to_string();
                        accumulator.distribution = distribution.to_string();
                        accumulator.urgency = urgency.to_string();
                    }

                    for cap in re2.captures_iter(l) {
                        is_re2 = true;
                        is_re2_local = true;
                        accumulator.details.push(cap.name("details").unwrap().to_string());
                    }

                    for cap in re3.captures_iter(l) {
                        is_re3 = true;

                        let mantainer_name = cap.name("mantainer_name").unwrap().trim();
                        let mantainer_email = cap.name("mantainer_email").unwrap();
                        let date = DateTime::parse_from_rfc2822(cap.name("date").unwrap());

                        accumulator.mantainer_name = mantainer_name.to_string();
                        accumulator.mantainer_email = mantainer_email.to_string();
                        if !date.is_err() {
                            accumulator.date = date.unwrap();
                        } else {
                            println!("failed to parse RFC2822: {}", cap.name("date").unwrap());
                        }
                    }

                    if is_re1 && is_re2 && !is_re2_local && !is_re3 {
                        let mut detail = String::from(accumulator.details.last().unwrap().to_string());
                        detail.push_str("\n");
                        detail.push_str(l);
                        let len = accumulator.details.len();
                        accumulator.details[len-1] = detail;
                    }

                    if is_re1 && is_re2 && is_re3 {
                        self.records.push(accumulator.copy());

                        is_re1 = false;
                        is_re2 = false;
                        is_re3 = false;
                        accumulator.details.clear();
                    };
                }
            }
        };
    }

    pub fn to(&mut self, path: &str) {
        match File::create(path) {
            Err(why) => { println!("create file failed: {:?}", why); }
            Ok(mut file) => {
                let s = self.to_string();

                match file.write(s.as_bytes()){
                    Ok(..) => {
                        file.sync_all().ok().expect("sync all failed");
                    }
                    Err(why) => {
                        println!("write to file failed: {}", why);
                    }
                }
            }
        };
    }
}
