use toml::value::Datetime;

pub struct PKG {
    pub name: String,
    pub version: u64,
    pub subversion: u64,
    pub description: String,
    pub url: String,
    pub packager: String,
    pub date: Datetime,
    pub license: Licenses,
    pub dependence: Vec<String>,
    pub architecture: Architecture,
    pub optional_dependence: Vec<String>
}

impl PKG {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_version(&self) -> u64 {
        self.version
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
    pub fn get_url(&self) -> String {
        self.url.clone()
    }
    pub fn get_packager(&self) -> String {
        self.packager.clone()
    }
    pub fn get_date(&self) -> Datetime {
        self.date.clone()
    }
    pub fn get_license(&self) -> Licenses {
        self.license.clone()
    }
    pub fn get_dependence(&self) -> Vec<String> {
        self.dependence.clone()
    }
    pub fn get_architecture(&self) -> Architecture {
        self.architecture.clone()
    }
    pub fn get_optional_dependence(&self) -> Vec<String> {
        self.dependence.clone()
    }
}

pub enum Licenses {
    MIT,
    GPL,
    GPLv2,
    APACHE,
    PROPRIETARY,
}
impl Clone for Licenses {
    fn clone(&self) -> Self {
        self.to_owned()
    }
}
impl Licenses {
    pub fn format(&self) -> String {
        match &self {
            Licenses::MIT => {String::from("MIT")}
            Licenses::GPL => {String::from("GPL")}
            Licenses::GPLv2 => {String::from("GPLv2")}
            Licenses::APACHE => {String::from("APACHE")}
            Licenses::PROPRIETARY => {String::from("PROPRIETARY")}
        }
    }
}

pub enum Architecture {
    X8664,
    X64,
    RISCV,
}

impl Clone for Architecture {
    fn clone(&self) -> Self {
        self.to_owned()
    }
}

impl Architecture {
    pub fn format(&self) -> String {
        match &self {
            Architecture::X8664 => {String::from("x86_64")}
            Architecture::X64 => {String::from("x64")}
            Architecture::RISCV => {String::from("RISCV")}
        }
    }
}
