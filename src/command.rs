use crate::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    Display,
    Open(String),
    Make(String),
    Move,
    Remove(String, Vec<Argument>),
    Exit,
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.is_empty() {
            return Err(Error::CommandNotRecognized("Command not recognized"));
        }

        match parts[0] {
            "help" => Ok(Command::Help),
            "ls" => Ok(Command::Display),
            "cd" if parts.len() == 2 => Ok(Command::Open(parts[1].to_string())),
            "mk" if parts.len() == 2 => Ok(Command::Make(parts[1].to_string())),
            "mv" => Ok(Command::Move),
            "rm" if parts.len() > 2 => {
                if parts.len() < 2 {
                    return Err(Error::CommandNotRecognized("rm expects at least one name"));
                }
                let name: String = parts[1].to_string();
                let mut args_vec: Vec<Argument> = Vec::new();
                for token in &parts[1..] {
                    let arg = Argument::from_str(token)?;
                    args_vec.push(arg);
                }

                Ok(Command::Remove(name, args_vec))
            }
            "exit" => Ok(Command::Exit),
            _ => Err(Error::CommandNotRecognized("Command not recognized")),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Argument {
    Force,
    Recursive,
}

impl FromStr for Argument {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "-f" | "--force" => Ok(Argument::Force),
            "-r" | "--recursive" => Ok(Argument::Recursive),
            _ => Err(Error::ArgsNotRecognized(format!(
                "Argument {} not recognized",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;

    #[test]
    fn command_not_recognized() {
        let res = Command::from_str("unknowncmd");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            Error::CommandNotRecognized("Command not recognized")
        );
    }

    #[test]
    fn command_help() {
        let res = Command::from_str("help");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Command::Help);
    }

    #[test]
    fn command_exit() {
        let res = Command::from_str("exit");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Command::Exit);
    }

    // #[test]
    // fn parse_rm_multiple_names() {
    //     let cmd = Command::from_str("rm a b c").unwrap();
    //     match cmd {
    //         Command::Remove(names, args) => {
    //             assert_eq!(
    //                 names,
    //                 vec!["a".to_string(), "b".to_string(), "c".to_string()]
    //             );
    //             assert!(args.is_empty());
    //         }
    //         _ => panic!("unexpected command variant"),
    //     }
    // }

    // #[test]
    // fn parse_rm_with_flags_before_names() {
    //     let cmd = Command::from_str("rm -f a b").unwrap();
    //     match cmd {
    //         Command::Remove(names, args) => {
    //             assert_eq!(names, vec!["a".to_string(), "b".to_string()]);
    //             assert_eq!(args, vec![Argument::Force]);
    //         }
    //         _ => panic!("unexpected command variant"),
    //     }
    // }

    // #[test]
    // fn parse_rm_with_flags_between_names() {
    //     let cmd = Command::from_str("rm a -r b").unwrap();
    //     match cmd {
    //         Command::Remove(names, args) => {
    //             assert_eq!(names, vec!["a".to_string(), "b".to_string()]);
    //             assert_eq!(args, vec![Argument::Recursive]);
    //         }
    //         _ => panic!("unexpected command variant"),
    //     }
    // }

    // #[test]
    // fn parse_rm_unknown_flag_error() {
    //     let res = Command::from_str("rm -x a b");
    //     assert!(matches!(res, Err(Error::ArgsNotRecognized(_))));
    // }

    // #[test]
    // fn parse_rm_no_name_error() {
    //     let res = Command::from_str("rm -f -r");
    //     assert!(matches!(res, Err(Error::CommandNotRecognized(_))));
    // }
}
