#[derive(Debug, PartialEq)] // PartialEq needed for test
pub enum Method {
    Encode,
    Decode,
    Extra1,
    Extra2,
    Extra3,
    Extra4,
    Permutations,
}

#[derive(Debug, PartialEq)] // PartialEq needed for test
pub struct Config {
    pub method: Method,
    pub word: Option<String>,
}

pub fn get_config(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 2 {
        return Err("Not enough arguments");
    }
    log::debug!("args: {:?}", args);
    let config: Config = match args[1].as_str() {
        "encode" => {
            if args.len() != 3 {
                return Err("Wrong number of arguments, must be 2");
            }
            let w = Some(args[2].clone());
            Config {
                method: Method::Encode,
                word: w,
            }
        }
        "decode" => {
            if args.len() != 3 {
                return Err("Wrong number of arguments, must be 2");
            }
            let w = Some(args[2].clone());
            Config {
                method: Method::Decode,
                word: w,
            }
        }
        "extra1" => Config {
            method: Method::Extra1,
            word: None,
        },
        "extra2" => Config {
            method: Method::Extra2,
            word: None,
        },
        "extra3" => Config {
            method: Method::Extra3,
            word: None,
        },
        "extra4" => Config {
            method: Method::Extra4,
            word: None,
        },
        "permutations" => {
            let w = if args.len() == 3 {
                Some(args[2].clone())
            } else if args.len() == 2 {
                None
            } else {
                return Err("Too many arguments");
            };
            Config {
                method: Method::Permutations,
                word: w,
            }
        }
        _ => return Err("Wrong verb argument"),
    };
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        assert_eq!(
            get_config(&[
                "programname".to_string(),
                "encode".to_string(),
                "Carlotta".to_string()
            ]),
            Ok(Config {
                method: Method::Encode,
                word: Some("Carlotta".to_string())
            })
        );
        assert_eq!(
            get_config(&[
                "programname".to_string(),
                "decode".to_string(),
                "-.-.-.--".to_string()
            ]),
            Ok(Config {
                method: Method::Decode,
                word: Some("-.-.-.--".to_string())
            })
        );
        assert_eq!(
            get_config(&["programname".to_string(), "extra1".to_string()]),
            Ok(Config {
                method: Method::Extra1,
                word: None
            })
        );
        assert_eq!(
            get_config(&["programname".to_string(), "extra2".to_string()]),
            Ok(Config {
                method: Method::Extra2,
                word: None
            })
        );
        assert_eq!(
            get_config(&["programname".to_string(), "extra3".to_string()]),
            Ok(Config {
                method: Method::Extra3,
                word: None
            })
        );
        assert_eq!(
            get_config(&["programname".to_string(), "extra4".to_string()]),
            Ok(Config {
                method: Method::Extra4,
                word: None
            })
        );
        assert!(
            get_config(&["programname".to_string(), "nope".to_string()]).is_err(),
            "Wrong verb argument"
        );
    }
}
