use clap::{Args, ValueEnum};
use std::{path::PathBuf};
use super::*;

///syntaxe: todo list (value)* (-s)* (-p "path")*
///description: list all todos in text, 
///args:
///     if no value is provided then it returns all todos by default
///flag
///     -s -> silences description making it less noisy
///     -p -> returns value of specifique path
#[derive(Args, Debug)]
pub struct ListCommand {     
    value: Option<String>,
        
    #[arg(short, default_value_t = false)]
    silent: bool,

    #[arg(short,default_value = ".")]
    path: PathBuf,

    #[arg(long, value_enum, default_value_t = OutputType::Visual)]
    output: OutputType,
}
#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum OutputType {
    Visual,
    Json,
    None,
}

impl Into<Cmd> for ListCommand {
    fn into(self) -> Cmd {
        Cmd {
            silent: self.silent,
            value : self.value,
            path  : self.path,
            output: self.output
        }
    }
}



/*
 * deprecated
///syntaxe: todo open (value)* (-p "path")*
///description: open all todos in files, 
///args:
///     if no value is provided then it returns opens all file containing todos
///flag
///     -p -> returns values of specifique path
///     -r (not implemented yet) -> recursively search through child folders
#[derive(Args, Debug)]
pub struct OpenCommand {
    value: Option<String>,

    #[arg(short,default_value = ".")]
    path: PathBuf, 
}

impl Into<Cmd> for OpenCommand {
    fn into(self) -> Cmd {
         Cmd {
            silent: false,
            value : self.value,
            path  : self.path,
            output: OutputType::None,
        }
    }
}
*/


#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    pub fn listcmd_default() {
        let args = Cli::parse_from(["todo","list"]);
        let Command::List(cmd) = args.command; 

        assert_eq!(None,cmd.value);
        assert_eq!(false, cmd.silent);
        assert_eq!(Some("."), cmd.path.to_str());
    }

    #[test]
    pub fn listcmd_custom() {
        let args = Cli::parse_from(["todo","list", "test_value", "-sp","test_path"]);
        let Command::List(cmd) = args.command;

        assert_eq!(Some("test_value"),cmd.value.as_ref().map(|s|s.as_str()));
        assert_eq!(true, cmd.silent);
        assert_eq!(Some("test_path"), cmd.path.to_str());
    }
/*
    #[test]
    pub fn opencmd_default(){
        let args = Cli::parse_from(["todo", "open"]);
        let Command::Open(cmd) = args.command else { panic!("command should be open")};

        assert_eq!(None,cmd.value);
        assert_eq!(Some("."), cmd.path.to_str());
    }

    #[test]
    pub fn opencmd_custom(){
        let args = Cli::parse_from(["todo", "open", "test_value","-p","test_path"]);
        let Command::Open(cmd) = args.command else { panic!("command should be open")};

        assert_eq!(Some("test_value"),cmd.value.as_ref().map(|s|s.as_str()));
        assert_eq!(Some("test_path"), cmd.path.to_str());
    }
*/
}
