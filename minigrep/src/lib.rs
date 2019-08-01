use std::{fs, error::Error, env};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            Err("not enough arguments")
        } else {
            Ok(Config{
                query: args[1].clone(),
                filename: args[2].clone(),
                case_sensitive: env::var("CASE_INSENSITIVE").is_err()
            }
            )
        }
    } 
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let res = match config.case_sensitive {
        true => search(&config.query, &content),
        false => search_case_insensitive(&config.query, &content),
    };
    for line in res {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut array = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            array.push(line);
        }
    }
    array
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut array = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            array.push(line);
        }
    }
    array
}


#[cfg(test)]
mod test{
    use super::*;

     #[test]
     fn case_sensitive(){
         let query = "duct";
         let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

         assert_eq!(
             vec!["safe, fast, productive."],
             search(query, contents)
         );
     }

     #[test]
     fn case_insensitive(){
let query = "rUst";
         let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

         assert_eq!(
             vec!["Rust:", "Trust me."],
             search_case_insensitive(query, contents)
         );
     }
}