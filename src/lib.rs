//This is the library to handle files in Rust.
use std::fs;
//This library let us call Return the error and the ? (closure) in the fn run
use std::error::Error;
//This library let us use the enviroment variable library (case_sensitive in Config)
use std::env;
//Lets relate the values that we use in a Struct
pub struct Config {
    pub query: String,
		pub filename: String,
		//This field is to switch between case_sensitive or insensitive search
		pub case_sensitive: bool,
}
/*The function to extract the parse logic it's inside a Config implementation so we
can call an instance like -> Config::new(&[String])*/
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        /*The new fn output is a Result<T,E> To handle success and to handle errors
        and give clear messages -> */
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        /*args will have three values the first one is the name of our binary, we
        don't need it here, so lets exclude it */
        let query = args[1].clone();
        let filename = args[2].clone();
				/*If the env::var is set to something it will perform the case senstive search 
				if isn't set, we get an error and the opposite search'll be performed*/
				let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
				/*To set the env::var a value we call it in the command line and give it a 
				value like CASE_INSENSITIVE=1 then cargo run to poem.txt*/
        return Ok(Config { query, filename, case_sensitive });
    }
}
//We Return a Result<T, E> dyn -> means dynamic and the Error is the error trait (?)
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
		//The fs::read_to_string takes the file, read it and returns a Resul<String>
    /*The ? will let us return the error value from the current function for the
		caller to handle it*/
		let contents = fs::read_to_string(config.filename)?;
		/*Lets create a variable to check the value of case_sensitive field and decide
		what kind of seach do we need to do*/
		let result = if config.case_sensitive {
				search(&config.query, &contents)
		} else {
			search_case_insensitive(&config.query, &contents)
		};

		for line in result {
				println!("{}", line);
		}
    //After all we need to wrap the unit type value in the Ok value->
    Ok(())
}
//Now let's create the functions that we need to test ->
//We connect the result lifetime to contents because the result its a slice of it
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
			//Let's make a vector to capture the coincidences
			let mut result = Vec::new();
			//.lines() is a method to handle line-byline and return an iterator
			for line in contents.lines() {
						if line.contains(query) {
								result.push(line);
						}

			}
			result
}
//Lets create a function to handle case sensitive strings
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
		let query = query.to_lowercase();
		let mut result = Vec::new();
			for line in contents.lines() {
						if line.to_lowercase().contains(&query) {
								result.push(line);
						}

			}
			result
}
//Let´s creat a test mod
#[cfg(test)]
mod test {
    use super::*;
    /*And now let's create a text to search the string "duct", we search on three
    lines, expecting to get just the line that contains "duct"*/
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
		}
		#[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
