
#[derive(Clone, Debug)]
pub(crate) struct ConfigError {
    kind: ConfigErrorKind,
}


#[derive(Clone, Debug)]
enum ConfigErrorKind {
    //this for now stops the program from working 
    //if we cant parse we cant run it
    BadConfig(String),
    BadRegex{ext:String, msg:String},
    //Shouldn't stop people from using the entire program 
    //but without home directory the program cant hardlink 
    //concerned folders and has to default to default config
    NoHomeDirectory(&'static str),
    NoConfigFileFound(&'static str),
}
impl ConfigError {
    pub(crate)fn bad_config(err: String)->Self{
        ConfigError {
            kind: ConfigErrorKind::BadConfig(err)
        }
    }
    pub(crate)fn bad_regex(ext:String, err: String)->Self{
        ConfigError {
            kind: ConfigErrorKind::BadRegex{ext:ext, msg: err}
        }
    }
    pub(crate)fn no_home(err: &'static str)-> Self {
        ConfigError {
            kind: ConfigErrorKind::NoHomeDirectory(err)
        }
    }

}

impl std::error::Error for ConfigError {}

impl core::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.kind {
            ConfigErrorKind::BadConfig(ref err) => {
                write!(f,"[FATAL] Bad Config: {err}")
            }
            ConfigErrorKind::BadRegex{ ref ext,ref msg }=>{
                write!(f,"[WARNING] Regex of {ext} is invalid therefore will be skipped, {msg}")
            },
            ConfigErrorKind::NoHomeDirectory(ref err)=>{
                write!(f,"[WARNING]No home directory is found, some features will be closed, {err}")
            },
            ConfigErrorKind::NoConfigFileFound(ref err)=>{
                write!(f,"[FATAL]The config file not be read, {err}")
            },
        }
    }
}
