use std::env;

#[cfg(test)]
mod tests {
    use super::*;
    mod config_tests {
        use super::*;
        #[test]
        fn config_constructor() {
            let args = vec![String::from("executable_path"), String::from("query"), String::from("file_path")];
    
            let c = Config::new(args.into_iter()).unwrap();
    
            assert_eq!(c.query, args[1]);
            assert_eq!(c.path, args[2]);
        }

        #[test]
        #[should_panic]
        fn config_constructor_error() {
            let args = vec![String::from("executable_path"), String::from("query")];

            Config::new(&args).unwrap();
        }
    }

    mod search_tests {
        use super::*;
        #[test]
        fn case_sensitive() {
            let query = "duct";
            let query2 = "DUCT";
            let content = "\
Rust:
safe, fast, productive.
pick three.";
            let case_sensitive = true;

            let search_config = SearchConfig::new(query, &content[0..], case_sensitive);
            let search_config2 = SearchConfig::new(query2, &content[0..], case_sensitive);

            assert_eq!(
                vec!["safe, fast, productive."], 
                search(&search_config),
                );
            assert_ne!(
                vec!["safe, fast, productive."], 
                search(&search_config2)
                );
        }

        #[test]
        fn case_insensitive() {
            let query = "DUCT";
            let query2 = "duct";
            let content = "\
Rust:
safe, fast, productive.
pick three.";
            let case_sensitive = false;

            let search_config = SearchConfig::new(query, &content[0..], case_sensitive);
            let search_config2 = SearchConfig::new(query2, &content[0..], case_sensitive);

            assert_eq!(
                vec!["safe, fast, productive."], 
                search(&search_config)
                );

            assert_eq!(
                vec!["safe, fast, productive."], 
                search(&search_config2)
                );
        }
    }
}

use std::fs;

#[derive(Debug)]
pub struct Config {
    query: String,
    path:  String
}

pub struct SearchConfig<'a> {
    pub query: String,
    pub content: &'a str,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough parameters provided!");
        }
        
        let query = match args.next() {
            Some(arg)   => arg,
            None        => return Err("Didn't get a query string"),
        };

        let path = match args.next() {
            Some(arg)   => arg,
            None        => return Err("Didn't get a path string"),
        };

        Ok(Config { query, path })
    }
}

impl<'a> SearchConfig<'a> {
    pub fn new(query: &str, content: &'a str, case_sensitive: bool) -> SearchConfig<'a> {
        SearchConfig {
            query: String::from(query),
            content: content,
            case_sensitive: case_sensitive
        }
    }
}

use std::error::Error;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string(&config.path)?;
    let case_sensitive = true;

    let search_config = SearchConfig::new(&config.query, &content[0..], case_sensitive);

    for l in search(&search_config) {
        println!("{}", l);
    }

    Ok(())
}

pub fn search<'a>(conf: &SearchConfig<'a>) -> Vec<&'a str> {

    conf.content.lines()
    .filter(|l| l.contains(&conf.query))
    .collect()

}
 






















