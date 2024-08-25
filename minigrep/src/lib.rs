use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

	// ? is shorthand for propagating errors. 
	let content = fs::read_to_string(config.file_path)?;

	let results = if config.ignore_case {
		search_case_insensitive(&config.query, &content)
	} else {
		search_case_sensitive(&config.query, &content)
	};

    for line in results {
        println!("{}", line);
    }
	Ok(())
}

pub struct Config {
	pub query: String,
	pub file_path: String,
	pub ignore_case: bool,
}

impl Config {
	pub fn build(
		mut args: impl Iterator<Item = String>
	) -> Result<Config, &'static str> { // error is a string with static lifetime
		args.next(); // skip the first argument, which is the program name

		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Didnt get a query string"),
		};

		let file_path = match args.next() {
			Some(arg) => arg,
			None => return Err("Didnt get a file path"),
		};

		// let ignore_case = args[3].parse::<bool>().unwrap_or(false);
		let ignore_case = env::var("IGNORE_CASE").is_ok(); // if not set, return false

		Ok(Config { query, file_path, ignore_case })
	}
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();

	// improved version below in `search_case_insensitive`
	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}
	results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	contents
	    .lines()
	    .filter(|line| line.to_lowercase().contains(&query))
	    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."], 
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
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
