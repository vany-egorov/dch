use chrono::{DateTime,UTC,FixedOffset,Local};

#[derive(Clone)]
pub struct MantainerDetails {
    pub mantainer: String,
    pub details: Vec<String>,
}

impl MantainerDetails {
    pub fn new() -> MantainerDetails {
        MantainerDetails {
            mantainer: "".to_string(),
            details: Vec::new(),
        }
    }
}

pub struct Record {
    pub package: String,
    pub version: String,
    pub distribution: String,
    pub urgency: String,

    pub mantainer_details: Vec<MantainerDetails>,

    pub mantainer_name: String,
    pub mantainer_email: String,
    pub date: DateTime<FixedOffset>,
}

impl Record {
    pub fn new() -> Record {
        Record {
            package: "".to_string(),
            version: "".to_string(),
            distribution: "".to_string(),
            urgency: "".to_string(),

            mantainer_details: vec![MantainerDetails::new()],

            mantainer_name: "".to_string(),
            mantainer_email: "".to_string(),
            date: UTC::now().with_timezone(Local::now().offset()),
        }
    }

    pub fn _log(&self) { println!("{}", self.to_string()); }

    pub fn to_string(&self) -> String {
        let mut s = String::new();

        s.push_str(&format!("{package} ({version}) {distribution}; urgency={urgency}\n",
            package=self.package,
            version=self.version,
            distribution=self.distribution,
            urgency=self.urgency,
        ));

        s.push_str("\n");
        for md in self.mantainer_details.iter() {
            if !md.mantainer.is_empty() {
                s.push_str(&format!("  [ {} ]\n", md.mantainer));
            }

            for detail in md.details.iter() {
                s.push_str(&format!("  * {}\n", detail));
            }

            s.push_str("\n");
        };

        s.push_str(&format!(" -- {mantainer_name} <{mantainer_email}>  {date}\n",
            mantainer_name=self.mantainer_name,
            mantainer_email=self.mantainer_email,
            date=self.date.format("%a, %d %b %Y %H:%M:%S %z").to_string(),
        ));

        s
    }

    pub fn copy(&self) -> Record {
        Record {
            package: String::from(self.package.to_string()),
            version: String::from(self.version.to_string()),
            distribution: String::from(self.distribution.to_string()),
            urgency: String::from(self.urgency.to_string()),

            mantainer_details: self.mantainer_details.to_vec(),

            mantainer_name: String::from(self.mantainer_name.to_string()),
            mantainer_email: String::from(self.mantainer_email.to_string()),
            date: self.date.clone(),
        }
    }
}
