use chrono::{DateTime,UTC,FixedOffset,Local};

pub struct Record {
    pub package: String,
    pub version: String,
    pub distribution: String,
    pub urgency: String,

    pub details: Vec<String>,

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

            details: Vec::new(),

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
        for detail in self.details.iter() {
            s.push_str(&format!("  * {}\n", detail));
        };
        s.push_str("\n");

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

            details: self.details.to_vec(),

            mantainer_name: String::from(self.mantainer_name.to_string()),
            mantainer_email: String::from(self.mantainer_email.to_string()),
            date: self.date.clone(),
        }
    }
}
