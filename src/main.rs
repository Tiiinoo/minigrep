/*The minigrep project aims to ->
Search a specified file for a specified string. To do so, grep takes as its
arguments a filename and a string. Then it reads the file, finds lines in that
file that contain the string argument, and prints those lines.*/
//First we call this library, to be able to read command lines arguments->
use std::env;
//This library help us handle the errors calling the Config::new()
use std::process;
//Here we call our library!
use minigrep::Config;

fn main() {
    /*env::args let as read command lines arguments making an iterator, .collect()
    method turns an interator to a collection, we first need to define what type our
    collector'll be, in this case a Vec<String>*/
    let args: Vec<String> = env::args().collect();
    //Here we call the function, but as an instance of the Config implementation.
    //Now we have to manage the output -> Result. We use the method unwrap_or_else
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    //Before, we called the function run, but now we handle the error with if let Err
    if let Err(e) = minigrep::run(config) {
				/*adding that "e" before printing means we print errors just in the command
				line. If we run the program wit > some file, no error will be there!*/
				eprintln!("Application error: {}", e);
				
        process::exit(1)
    };
}
